use crate::services::post_service::PostService;
use actix_web::web::Data;
use std::sync::Arc;
use sqlx::PgPool;
use crate::infras::repositories::user_repository::UserRepository;
use crate::infras::repositories::post_repository::PostRepository;
use crate::services::user_service::UserService;
use crate::services::auth_service::{AuthService, JwtEncoder};

pub struct AppState {
    pub post_service: PostService,
    pub user_service: UserService,
    pub auth_service: AuthService,
}

pub async fn new_app_state(pool: PgPool) -> Data<AppState> {
    let user_repository = Arc::new(UserRepository::new(pool.clone()));
    let post_service = PostService::new(Arc::new(PostRepository::new(pool.clone())));
    let user_service = UserService::new(user_repository.clone());

    let secret_key = std::env::var("SECRET_KEY")
        .expect("SECRET_KEY must be set");

    let auth_service = AuthService::new(user_service.clone(), JwtEncoder::new(secret_key.clone()));

    Data::new(AppState { post_service, user_service, auth_service })
}
