use crate::routes;
use crate::state::new_app_state;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use sqlx::PgPool;
use tokio::join;

pub async fn run(pool: PgPool) -> std::io::Result<()> {
    let secret_key = Key::from(
        std::env::var("SECRET_KEY")
            .expect("SECRET_KEY must be set")
            .as_bytes(),
    );
    let host: String = std::env::var("DB_HOST").unwrap_or("127.0.0.1".to_string());
    let port: u16 = std::env::var("DB_PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .expect("PORT must be a valid u16");

    let state = new_app_state(pool.clone()).await;
    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .configure(routes::config)
    })
    .bind((host.clone(), port))?
    .run();
    log::info!("Server running at http://{}:{}", host, port);
    let _ = join!(server);
    Ok(())
}
