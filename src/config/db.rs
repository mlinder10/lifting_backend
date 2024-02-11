use dotenv::dotenv;
use sqlx::{postgres::PgPool, Pool, Postgres};

pub async fn connect() -> Pool<Postgres> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be in .env file");

    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
