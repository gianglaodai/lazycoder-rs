use crate::infras::controllers::user_controller::UserTO;
use crate::infras::middleware::auth_guard::CurrentUser;
use crate::services::auth_service::{LoginRequest, RegisterRequest};
use crate::state::AppState;
use actix_session::Session;
use actix_web::web::{Data, Json, ServiceConfig, scope};
use actix_web::{HttpResponse, Responder, get, post};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequestTO {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequestTO {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl From<LoginRequestTO> for LoginRequest {
    fn from(login_req: LoginRequestTO) -> Self {
        Self {
            username: login_req.username,
            password: login_req.password,
        }
    }
}

impl From<RegisterRequestTO> for RegisterRequest {
    fn from(register_req: RegisterRequestTO) -> Self {
        Self {
            username: register_req.username,
            email: register_req.email,
            password: register_req.password,
        }
    }
}

#[post("/login")]
pub async fn login(
    state: Data<AppState>,
    login_req: Json<LoginRequestTO>,
    session: Session,
) -> impl Responder {
    let result = state
        .auth_service
        .login(LoginRequest::from(login_req.into_inner()))
        .await;
    match result {
        Ok(user) => {
            if let Err(err) = session.insert("user", UserTO::from(user)) {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to login user: {}", err));
            }
            HttpResponse::Ok().body("Logged in")
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}

#[post("/register")]
pub async fn register(
    state: Data<AppState>,
    register_req: Json<RegisterRequestTO>,
    session: Session,
) -> impl Responder {
    let result = state
        .auth_service
        .register(RegisterRequest::from(register_req.into_inner()))
        .await;

    match result {
        Ok(user) => {
            if let Err(err) = session.insert("user", UserTO::from(user)) {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to register user: {}", err));
            }
            HttpResponse::Created().body("Registered")
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}

#[post("/logout")]
pub async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok().body("Logged out")
}

#[get("/me")]
pub async fn me(user: CurrentUser) -> impl Responder {
    HttpResponse::Ok().json(user.0)
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(me).service(
        scope("/auth")
            .service(login)
            .service(register)
            .service(logout),
    );
}
