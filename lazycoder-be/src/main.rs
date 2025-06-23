mod app;
mod config;
mod db;
mod routes;
mod controllers;
mod repositories;
mod services;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init_env();
    app::run().await
}
