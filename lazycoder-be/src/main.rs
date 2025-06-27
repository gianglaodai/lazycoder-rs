mod app;
mod config;
mod db;
mod routes;
mod infras;
mod services;
mod state;
mod macros;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init_env();
    let pool = db::init_pool().await.expect("Failed to connect DB");
    db::run_migrations(&pool).await.expect("Failed to run migrations");
    app::run(pool).await
}
