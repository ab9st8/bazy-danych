use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;
    println!("Migrations applied successfully.");

    let seed_sql = include_str!("../seeds/seed.sql");
    sqlx::raw_sql(seed_sql).execute(&pool).await?;
    println!("Seed data loaded.");

    let app = axum::Router::new().with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening on :3000");
    axum::serve(listener, app).await?;

    Ok(())
}
