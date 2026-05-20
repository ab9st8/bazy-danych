use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;
use clap::Parser;
use rand::seq::SliceRandom;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::{Semaphore, mpsc};
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

    /// Interval for rolling stats snapshots.
    #[arg(long, default_value = "5s", value_parser = humantime::parse_duration)]
    snapshot_interval: Duration,
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

#[derive(Debug)]
enum Outcome {
    ReserveHeld { latency_ms: u64 },
    ReserveWaitlisted { latency_ms: u64 },
    ReserveError { latency_ms: u64 },
    ReserveNetworkError,
    PayOk { latency_ms: u64 },
    PayExpired { latency_ms: u64 },
    PayConflict { latency_ms: u64 },
    PayError { latency_ms: u64 },
    PayNetworkError,
}

#[derive(Default)]
struct Stats {
    reserve_held: u64,
    reserve_waitlisted: u64,
    reserve_error: u64,
    reserve_network_error: u64,
    pay_ok: u64,
    pay_expired: u64,
    pay_conflict: u64,
    pay_error: u64,
    pay_network_error: u64,
    latency_min_ms: Option<u64>,
    latency_max_ms: u64,
    latency_sum_ms: u128,
    latency_count: u64,
}

impl Stats {
    fn record(&mut self, outcome: Outcome) {
        match outcome {
            Outcome::ReserveHeld { latency_ms } => {
                self.reserve_held += 1;
                self.observe_latency(latency_ms);
            }
            Outcome::ReserveWaitlisted { latency_ms } => {
                self.reserve_waitlisted += 1;
                self.observe_latency(latency_ms);
            }
            Outcome::ReserveError { latency_ms } => {
                self.reserve_error += 1;
                self.observe_latency(latency_ms);
            }
            Outcome::ReserveNetworkError => {
                self.reserve_network_error += 1;
            }
            Outcome::PayOk { latency_ms } => {
                self.pay_ok += 1;
                self.observe_latency(latency_ms);
            }
            Outcome::PayExpired { latency_ms } => {
                self.pay_expired += 1;
                self.observe_latency(latency_ms);
            }
            Outcome::PayConflict { latency_ms } => {
                self.pay_conflict += 1;
                self.observe_latency(latency_ms);
            }
            Outcome::PayError { latency_ms } => {
                self.pay_error += 1;
                self.observe_latency(latency_ms);
            }
            Outcome::PayNetworkError => {
                self.pay_network_error += 1;
            }
        }
    }

    fn observe_latency(&mut self, ms: u64) {
        self.latency_min_ms = Some(self.latency_min_ms.map_or(ms, |m| m.min(ms)));
        self.latency_max_ms = self.latency_max_ms.max(ms);
        self.latency_sum_ms += ms as u128;
        self.latency_count += 1;
    }

    fn sessions(&self) -> u64 {
        self.reserve_held
            + self.reserve_waitlisted
            + self.reserve_error
            + self.reserve_network_error
    }

    fn avg_latency_ms(&self) -> u64 {
        if self.latency_count == 0 {
            0
        } else {
            (self.latency_sum_ms / self.latency_count as u128) as u64
        }
    }

    fn log(&self, elapsed: Duration, kind: &str) {
        let elapsed_s = elapsed.as_secs_f64();
        let rate = if elapsed_s > 0.0 {
            self.sessions() as f64 / elapsed_s
        } else {
            0.0
        };
        tracing::info!(
            elapsed_s = elapsed_s as u64,
            sessions = self.sessions(),
            rate_per_s = format!("{rate:.1}"),
            reserve_held = self.reserve_held,
            reserve_wait = self.reserve_waitlisted,
            reserve_err = self.reserve_error,
            reserve_net = self.reserve_network_error,
            pay_ok = self.pay_ok,
            pay_expired = self.pay_expired,
            pay_conflict = self.pay_conflict,
            pay_err = self.pay_error,
            pay_net = self.pay_network_error,
            latency_min_ms = self.latency_min_ms.unwrap_or(0),
            latency_avg_ms = self.avg_latency_ms(),
            latency_max_ms = self.latency_max_ms,
            "{kind}"
        );
    }
}

async fn aggregate(
    mut rx: mpsc::Receiver<Outcome>,
    snapshot_interval: Duration,
    started_at: Instant,
) {
    let mut stats = Stats::default();
    let mut snapshot = tokio::time::interval(snapshot_interval);
    snapshot.tick().await; // discard the immediate first tick

    loop {
        tokio::select! {
            msg = rx.recv() => match msg {
                Some(outcome) => stats.record(outcome),
                None => break,
            },
            _ = snapshot.tick() => {
                stats.log(started_at.elapsed(), "snapshot");
            }
        }
    }

    stats.log(started_at.elapsed(), "summary");
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

    let (tx, rx) = mpsc::channel::<Outcome>(1024);
    let started_at = Instant::now();
    let aggregator = tokio::spawn(aggregate(rx, args.snapshot_interval, started_at));

    let client = Arc::new(Client::new());
    let sem = Arc::new(Semaphore::new(args.concurrency));
    let base_url = Arc::new(args.base_url);

    let mut ticker = tokio::time::interval(interval_per_request);
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    let deadline = started_at + args.duration;
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
        let tx = tx.clone();

        tokio::spawn(async move {
            let _permit = permit;
            run_one(client, base_url, event_id, tx).await;
        });
    }

    let _drain = sem
        .acquire_many(args.concurrency as u32)
        .await
        .expect("acquire all");
    drop(tx);
    aggregator.await.expect("aggregator panicked");

    Ok(())
}

async fn run_one(
    client: Arc<Client>,
    base_url: Arc<String>,
    event_id: Uuid,
    tx: mpsc::Sender<Outcome>,
) {
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
            let _ = tx.send(Outcome::ReserveNetworkError).await;
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
                    let _ = tx.send(Outcome::ReserveError { latency_ms }).await;
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
            let _ = tx.send(Outcome::ReserveHeld { latency_ms }).await;

            pay(&client, &base_url, user_id, body.reservation_id, &tx).await;
        }
        202 => {
            tracing::info!(
                user = %user_id,
                event = %event_id,
                status = 202,
                latency_ms,
                "reserve waitlisted"
            );
            let _ = tx.send(Outcome::ReserveWaitlisted { latency_ms }).await;
        }
        s => {
            tracing::warn!(
                user = %user_id,
                event = %event_id,
                status = s,
                latency_ms,
                "reserve unexpected"
            );
            let _ = tx.send(Outcome::ReserveError { latency_ms }).await;
        }
    }
}

async fn pay(
    client: &Client,
    base_url: &str,
    user_id: Uuid,
    reservation_id: Uuid,
    tx: &mpsc::Sender<Outcome>,
) {
    let url = format!("{}/reservations/{}/payment", base_url, reservation_id);

    let start = Instant::now();
    let res = client.post(&url).json(&PaymentBody { user_id }).send().await;
    let latency_ms = start.elapsed().as_millis() as u64;

    match res {
        Ok(r) => {
            let s = r.status().as_u16();
            match s {
                200 => {
                    tracing::info!(
                        user = %user_id,
                        reservation = %reservation_id,
                        status = s,
                        latency_ms,
                        "pay ok"
                    );
                    let _ = tx.send(Outcome::PayOk { latency_ms }).await;
                }
                403 => {
                    tracing::info!(
                        user = %user_id,
                        reservation = %reservation_id,
                        status = s,
                        latency_ms,
                        "pay expired"
                    );
                    let _ = tx.send(Outcome::PayExpired { latency_ms }).await;
                }
                409 => {
                    tracing::info!(
                        user = %user_id,
                        reservation = %reservation_id,
                        status = s,
                        latency_ms,
                        "pay conflict"
                    );
                    let _ = tx.send(Outcome::PayConflict { latency_ms }).await;
                }
                _ => {
                    tracing::warn!(
                        user = %user_id,
                        reservation = %reservation_id,
                        status = s,
                        latency_ms,
                        "pay unexpected"
                    );
                    let _ = tx.send(Outcome::PayError { latency_ms }).await;
                }
            }
        }
        Err(e) => {
            tracing::error!(
                user = %user_id,
                reservation = %reservation_id,
                error = %e,
                "pay network error"
            );
            let _ = tx.send(Outcome::PayNetworkError).await;
        }
    }
}
