use crate::controllers::response_result::{respond_result, respond_results};
use crate::define_to_with_common_fields;
use crate::services::post_service::Post;
use crate::state::AppState;
use actix_web::web::Data;
use actix_web::{Responder, delete, get, post, put, web};

define_to_with_common_fields!(PostTO {
    title: String,
    body: String,
});

impl From<PostTO> for Post {
    fn from(post: PostTO) -> Self {
        Self {
            id: post.id,
            uid: post.uid,
            created_at: post.created_at,
            updated_at: post.updated_at,
            title: post.title,
            body: post.body,
        }
    }
}

impl From<Post> for PostTO {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            uid: post.uid,
            created_at: post.created_at,
            updated_at: post.updated_at,
            title: post.title,
            body: post.body,
        }
    }
}

#[get("/")]
pub async fn get_posts(state: Data<AppState>) -> impl Responder {
    respond_results(state.post_service.get_posts().await, PostTO::from)
}

#[get("/{id}")]
pub async fn get_post_by_id(state: Data<AppState>, id: web::Path<i32>) -> impl Responder {
    respond_result(
        state
            .post_service
            .get_post(id.into_inner())
            .await
            .map(PostTO::from),
    )
}

#[post("/")]
pub async fn create_post(state: Data<AppState>, post: web::Json<PostTO>) -> impl Responder {
    respond_result(
        state
            .post_service
            .create_post(Post::from(post.into_inner()))
            .await
            .map(PostTO::from),
    )
}

#[put("/{id}")]
pub async fn update_post(state: Data<AppState>, post: web::Json<PostTO>) -> impl Responder {
    respond_result(
        state
            .post_service
            .update_post(Post::from(post.into_inner()))
            .await
            .map(PostTO::from),
    )
}

#[delete("/{id}")]
pub async fn delete_post(state: Data<AppState>, id: web::Path<i32>) -> impl Responder {
    respond_result(state.post_service.delete_post(id.into_inner()).await)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(get_posts)
            .service(get_post_by_id)
            .service(create_post)
            .service(update_post)
            .service(delete_post),
    );
}
