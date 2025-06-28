use crate::infras::repositories::post_repository::PostRepository;
use crate::infras::repositories::user_repository::UserRepository;
use crate::services::auth_service::AuthService;
use crate::services::post_service::PostService;
use crate::services::user_service::UserService;
use actix_web::web::Data;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub post_service: PostService,
    pub user_service: UserService,
    pub auth_service: AuthService,
}

pub async fn new_app_state(pool: PgPool) -> Data<AppState> {
    let user_repository = Arc::new(UserRepository::new(pool.clone()));
    let post_service = PostService::new(Arc::new(PostRepository::new(pool.clone())));
    let user_service = UserService::new(user_repository.clone());

    let auth_service = AuthService::new(user_service.clone());

    Data::new(AppState {
        post_service,
        user_service,
        auth_service,
    })
}
