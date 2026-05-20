use std::time::Duration;

use sqlx::PgPool;
use tokio::time::Instant;
use uuid::Uuid;

use crate::HOLD_DURATION_SECS;

const SWEEP_INTERVAL: Duration = Duration::from_secs(5);

pub async fn run(pool: PgPool) {
    let mut ticker = tokio::time::interval(SWEEP_INTERVAL);
    loop {
        ticker.tick().await;
        let now = Instant::now();
        match sweep(&pool).await {
            Ok(count) => {
                println!(
                    "[sweeper] sweeped {} reservation{} in {}us",
                    count,
                    if count == 1 { "" } else { "s" },
                    now.elapsed().as_micros()
                );
            }
            Err(e) => {
                eprintln!("[sweeper] error: {e}");
            }
        }
    }
}

async fn sweep(pool: &PgPool) -> Result<usize, sqlx::Error> {
    let mut count = 0;
    while sweep_one(pool).await? {
        count += 1;
    }
    Ok(count)
}

async fn sweep_one(pool: &PgPool) -> Result<bool, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let expired: Option<(Uuid, Uuid)> = sqlx::query_as(
        "SELECT id, event_id FROM seats
         WHERE status = 'held' AND held_until < now()
         LIMIT 1
         FOR UPDATE SKIP LOCKED",
    )
    .fetch_optional(&mut *tx)
    .await?;

    let Some((seat_id, event_id)) = expired else {
        tx.commit().await?;
        return Ok(false);
    };

    let next_in_line: Option<(Uuid, Uuid)> = sqlx::query_as(
        "SELECT id, user_id FROM waitlist
         WHERE event_id = $1
         ORDER BY joined_at ASC
         LIMIT 1
         FOR UPDATE SKIP LOCKED",
    )
    .bind(event_id)
    .fetch_optional(&mut *tx)
    .await?;

    match next_in_line {
        Some((waitlist_id, user_id)) => {
            sqlx::query(
                "UPDATE seats
                 SET held_by_user_id = $1,
                     held_until = now() + $2
                 WHERE id = $3",
            )
            .bind(user_id)
            .bind(Duration::from_secs(HOLD_DURATION_SECS))
            .bind(seat_id)
            .execute(&mut *tx)
            .await?;

            sqlx::query(
                "UPDATE reservations
                 SET user_id = $1, confirmed_at = now()
                 WHERE seat_id = $2",
            )
            .bind(user_id)
            .bind(seat_id)
            .execute(&mut *tx)
            .await?;

            sqlx::query("DELETE FROM waitlist WHERE id = $1")
                .bind(waitlist_id)
                .execute(&mut *tx)
                .await?;
        }
        None => {
            sqlx::query(
                "UPDATE seats
                 SET status = 'available',
                     held_until = NULL,
                     held_by_user_id = NULL
                 WHERE id = $1",
            )
            .bind(seat_id)
            .execute(&mut *tx)
            .await?;

            sqlx::query("DELETE FROM reservations WHERE seat_id = $1")
                .bind(seat_id)
                .execute(&mut *tx)
                .await?;
        }
    }

    tx.commit().await?;
    Ok(true)
}
