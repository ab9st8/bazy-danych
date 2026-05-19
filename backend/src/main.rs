use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;

mod routes;
mod tasks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,tower_http=info,sqlx=warn")),
        )
        .with_target(true)
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;
    tracing::info!("migrations applied successfully");

    let seed_sql = include_str!("../seeds/seed.sql");
    sqlx::raw_sql(seed_sql).execute(&pool).await?;
    tracing::info!("seed data loaded");

    tasks::spawn_all(pool.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on :3000");
    axum::serve(listener, routes::router(pool)).await?;

    Ok(())
}
