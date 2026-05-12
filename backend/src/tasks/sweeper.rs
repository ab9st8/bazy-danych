use std::time::Duration;

use sqlx::PgPool;
use uuid::Uuid;

const SWEEP_INTERVAL: Duration = Duration::from_secs(5);

pub async fn run(pool: PgPool) {
    let mut ticker = tokio::time::interval(SWEEP_INTERVAL);
    loop {
        ticker.tick().await;
        if let Err(e) = sweep(&pool).await {
            eprintln!("sweeper error: {e}");
        }
    }
}

async fn sweep(pool: &PgPool) -> Result<(), sqlx::Error> {
    while sweep_one(pool).await? {}
    Ok(())
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

    let Some((seat_id, _event_id)) = expired else {
        tx.commit().await?;
        return Ok(false);
    };

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

    tx.commit().await?;
    Ok(true)
}
