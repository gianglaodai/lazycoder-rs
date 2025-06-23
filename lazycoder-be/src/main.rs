mod app;
mod config;
mod db;
mod routes;
mod controllers;
mod repositories;
mod services;
mod state;

use actix_web::web::get;
use actix_web::{HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init_env();
    let pool = db::init_pool().await.expect("Failed to connect DB");
    app::run(pool).await
}
