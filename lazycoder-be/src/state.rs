use crate::db;
use crate::repositories::post_repository::PostRepository;
use crate::services::post_service::PostService;
use actix_web::web::Data;
use std::sync::Arc;

pub struct AppState {
    pub post_service: PostService,
}

pub async fn new_app_state() -> Data<AppState> {
    let pool = db::init_pool().await.expect("Failed to connect DB");
    let post_service = PostService::new(Arc::new(PostRepository::new(pool.clone())));
    Data::new(AppState { post_service })
}
