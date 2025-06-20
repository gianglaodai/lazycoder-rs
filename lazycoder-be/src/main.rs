use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::web::get;
use actix_web::{App, HttpResponse, HttpServer};
use tokio::join;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let secret_key = Key::from(
        std::env::var("SECRET_KEY")
            .expect("SECRET_KEY must be set")
            .as_bytes(),
    );
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .configure(init_routes)
    })
    .bind(("127.0.0.1", 3000))?
    .run();
    log::info!("Server running at http://127.0.0.1:3000");
    let _ = join!(server);
    Ok(())
}

fn init_routes(app: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("/api").route("/hello", get().to(hello));
    app.service(scope);
}

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}
