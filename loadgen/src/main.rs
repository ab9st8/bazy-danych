use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;
use clap::Parser;
use rand::seq::SliceRandom;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

const SEEDED_EVENT_IDS: &[&str] = &[
    "a0000000-0000-0000-0000-000000000001",
    "a0000000-0000-0000-0000-000000000002",
    "a0000000-0000-0000-0000-000000000003",
    "a0000000-0000-0000-0000-000000000004",
];

#[derive(Parser, Debug)]
#[command(about = "HTTP load generator for the reservations backend")]
struct Args {
    #[arg(long, default_value = "http://localhost:3000")]
    base_url: String,

    /// Target requests per minute.
    #[arg(long, default_value_t = 60)]
    rpm: u32,

    /// Cap on in-flight requests.
    #[arg(long, default_value_t = 32)]
    concurrency: usize,

    /// How long to run before stopping.
    #[arg(long, default_value = "60s", value_parser = humantime::parse_duration)]
    duration: Duration,

    /// Event UUID to target. Repeatable. Defaults to the four seeded UUIDs.
    #[arg(long = "event")]
    events: Vec<Uuid>,
}

#[derive(Serialize)]
struct ReserveBody {
    user_id: Uuid,
}

#[derive(Deserialize)]
struct ReserveResponse {
    reservation_id: Uuid,
}

#[derive(Serialize)]
struct PaymentBody {
    user_id: Uuid,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(true)
        .init();

    let args = Args::parse();

    let events: Vec<Uuid> = if args.events.is_empty() {
        SEEDED_EVENT_IDS
            .iter()
            .map(|s| s.parse().expect("seeded uuid"))
            .collect()
    } else {
        args.events.clone()
    };

    let interval_per_request = Duration::from_secs_f64(60.0 / args.rpm as f64);
    tracing::info!(
        rpm = args.rpm,
        concurrency = args.concurrency,
        duration_s = args.duration.as_secs(),
        events = events.len(),
        "loadgen starting"
    );

    let client = Arc::new(Client::new());
    let sem = Arc::new(Semaphore::new(args.concurrency));
    let base_url = Arc::new(args.base_url);

    let mut ticker = tokio::time::interval(interval_per_request);
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    let deadline = Instant::now() + args.duration;
    while Instant::now() < deadline {
        ticker.tick().await;
        if Instant::now() >= deadline {
            break;
        }

        let permit = match sem.clone().try_acquire_owned() {
            Ok(p) => p,
            Err(_) => {
                tracing::warn!("dropped tick (at concurrency cap)");
                continue;
            }
        };

        let event_id = *events
            .choose(&mut rand::thread_rng())
            .expect("non-empty events");
        let client = client.clone();
        let base_url = base_url.clone();

        tokio::spawn(async move {
            let _permit = permit;
            run_one(client, base_url, event_id).await;
        });
    }

    let _drain = sem
        .acquire_many(args.concurrency as u32)
        .await
        .expect("acquire all");
    tracing::info!("loadgen done");

    Ok(())
}

async fn run_one(client: Arc<Client>, base_url: Arc<String>, event_id: Uuid) {
    let user_id = Uuid::new_v4();
    let reserve_url = format!("{}/events/{}/reservations", base_url, event_id);

    let start = Instant::now();
    let res = client
        .post(&reserve_url)
        .json(&ReserveBody { user_id })
        .send()
        .await;
    let latency_ms = start.elapsed().as_millis() as u64;

    let res = match res {
        Ok(r) => r,
        Err(e) => {
            tracing::error!(
                user = %user_id,
                event = %event_id,
                error = %e,
                "reserve network error"
            );
            return;
        }
    };

    let status = res.status().as_u16();

    match status {
        201 => {
            let body: ReserveResponse = match res.json().await {
                Ok(b) => b,
                Err(e) => {
                    tracing::error!(
                        user = %user_id,
                        event = %event_id,
                        error = %e,
                        "reserve 201 body parse failed"
                    );
                    return;
                }
            };
            tracing::info!(
                user = %user_id,
                event = %event_id,
                reservation = %body.reservation_id,
                status = 201,
                latency_ms,
                "reserve held"
            );

            pay(&client, &base_url, user_id, body.reservation_id).await;
        }
        202 => {
            tracing::info!(
                user = %user_id,
                event = %event_id,
                status = 202,
                latency_ms,
                "reserve waitlisted"
            );
        }
        s => {
            tracing::warn!(
                user = %user_id,
                event = %event_id,
                status = s,
                latency_ms,
                "reserve unexpected"
            );
        }
    }
}

async fn pay(client: &Client, base_url: &str, user_id: Uuid, reservation_id: Uuid) {
    let url = format!("{}/reservations/{}/payment", base_url, reservation_id);

    let start = Instant::now();
    let res = client.post(&url).json(&PaymentBody { user_id }).send().await;
    let latency_ms = start.elapsed().as_millis() as u64;

    match res {
        Ok(r) => {
            let s = r.status().as_u16();
            match s {
                200 => tracing::info!(
                    user = %user_id,
                    reservation = %reservation_id,
                    status = s,
                    latency_ms,
                    "pay ok"
                ),
                403 => tracing::info!(
                    user = %user_id,
                    reservation = %reservation_id,
                    status = s,
                    latency_ms,
                    "pay expired"
                ),
                409 => tracing::info!(
                    user = %user_id,
                    reservation = %reservation_id,
                    status = s,
                    latency_ms,
                    "pay conflict"
                ),
                _ => tracing::warn!(
                    user = %user_id,
                    reservation = %reservation_id,
                    status = s,
                    latency_ms,
                    "pay unexpected"
                ),
            }
        }
        Err(e) => {
            tracing::error!(
                user = %user_id,
                reservation = %reservation_id,
                error = %e,
                "pay network error"
            );
        }
    }
}
