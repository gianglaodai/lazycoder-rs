use crate::services::post_service::Post;
use crate::state::AppState;
use actix_web::web::Data;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};

pub struct PostRequestBody {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
}

impl From<PostRequestBody> for Post{
    fn from(post: PostRequestBody) -> Self {
        Self {
            id: post.id,
            title: post.title,
            body: post.body,
        }
    }
}

#[get("/")]
pub async fn get_posts(state: Data<AppState>) -> impl Responder {
    match state.post_service.get_posts().await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
pub async fn get_post(state: Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match state.post_service.get_post(id.into_inner()).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => {
            log::error!("Failed to get post: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        },
    }
}

#[post("/")]
pub async fn create_post(state: Data<AppState>, post: web::Json<Post>) -> impl Responder {
    match state.post_service.create_post(post.into_inner()).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => {
            log::error!("Failed to create post: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        },
    }
}

#[put("/{id}")]
pub async fn update_post(state: Data<AppState>, post: web::Json<Post>) -> impl Responder {
    match state.post_service.update_post(post.into_inner()).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => {
            log::error!("Failed to update post: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        },
    }
}

#[delete("/{id}")]
pub async fn delete_post(state: Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match state.post_service.delete_post(id.into_inner()).await {
        Ok(effected_result) => HttpResponse::Ok().json(effected_result),
        Err(e) => {
            log::error!("Failed to delete post: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        },
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(get_posts)
            .service(get_post)
            .service(create_post)
            .service(update_post)
            .service(delete_post),
    );
}
