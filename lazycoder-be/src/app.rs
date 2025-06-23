use crate::repositories::post_repository::PostRepository;
use crate::routes;
use crate::services::post_service::PostService;
use crate::state::AppState;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::join;

pub async fn run(pool: PgPool) -> std::io::Result<()> {
    let secret_key = Key::from(
        std::env::var("SECRET_KEY")
            .expect("SECRET_KEY must be set")
            .as_bytes(),
    );
    let post_repository = Arc::new(PostRepository::new(pool.clone()));
    let post_service = PostService::new(post_repository);
    let state = Data::new(AppState { post_service });
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
    .bind(("127.0.0.1", 3000))?
    .run();
    log::info!("Server running at http://127.0.0.1:3000");
    let _ = join!(server);
    Ok(())
}
