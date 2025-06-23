use std::env;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn init_pool() -> Result<PgPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
