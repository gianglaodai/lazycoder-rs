use crate::repositories::post_repository::PostRepository;
use crate::services::post_service::PostService;
use actix_web::web::Data;
use std::sync::Arc;
use sqlx::PgPool;
use crate::repositories::user_repository::UserRepository;
use crate::services::user_service::UserService;

pub struct AppState {
    pub post_service: PostService,
    pub user_service: UserService,
}

pub async fn new_app_state(pool: PgPool) -> Data<AppState> {
    let post_service = PostService::new(Arc::new(PostRepository::new(pool.clone())));
    let user_service = UserService::new(Arc::new(UserRepository::new(pool.clone())));
    Data::new(AppState { post_service, user_service })
}
