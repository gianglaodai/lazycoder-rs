use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn init_pool() -> Result<PgPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    let schema_sql = include_str!("migrations/create_tables_v1.sql");
    run_statements_from_str(pool, schema_sql).await
}

async fn run_statements_from_str(pool: &PgPool, sql: &str) -> Result<(), sqlx::Error> {
    for statement in sql.split(";") {
        let stmt = statement.trim();
        if !stmt.is_empty() {
            sqlx::query(stmt).execute(pool).await?;
        }
    }
    Ok(())
}
