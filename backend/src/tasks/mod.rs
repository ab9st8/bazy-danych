use sqlx::PgPool;

mod sweeper;

pub fn spawn_all(pool: PgPool) {
    tokio::spawn(sweeper::run(pool));
}
