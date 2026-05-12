use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

mod routes;
mod tasks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;
    println!("Migrations applied successfully.");

    let seed_sql = include_str!("../seeds/seed.sql");
    sqlx::raw_sql(seed_sql).execute(&pool).await?;
    println!("Seed data loaded.");

    tasks::spawn_all(pool.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening on :3000");
    axum::serve(listener, routes::router(pool)).await?;

    Ok(())
}
