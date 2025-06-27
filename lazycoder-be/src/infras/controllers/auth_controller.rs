use crate::infras::controllers::response_result::respond_result;
use crate::services::auth_service::{LoginRequest, RegisterRequest};
use crate::state::AppState;
use actix_web::web::{Data, Json, ServiceConfig, scope};
use actix_web::{Responder, post};
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
pub async fn login(state: Data<AppState>, login_req: Json<LoginRequestTO>) -> impl Responder {
    let result = state
        .auth_service
        .login(LoginRequest::from(login_req.into_inner()))
        .await
        .map(|token| token);

    respond_result(result)
}

#[post("/register")]
pub async fn register(state: Data<AppState>, register_req: Json<RegisterRequestTO>) -> impl Responder {
    let result = state
        .auth_service
        .register(RegisterRequest::from(register_req.into_inner()))
        .await
        .map(|token| token);
    
    respond_result(result)
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/auth")
            .service(login)
            .service(register),
    );
}