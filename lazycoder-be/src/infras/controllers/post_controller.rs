use crate::infras::controllers::response_result::{respond_result, respond_results};
use crate::define_to_with_common_fields;
use crate::services::post_service::Post;
use crate::state::AppState;
use actix_web::web::{Data, Json, Path, ServiceConfig, scope};
use actix_web::{Responder, delete, get, post, put};

define_to_with_common_fields!(PostTO {
    pub title: String,
    pub body: String,
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
pub async fn get_many(state: Data<AppState>) -> impl Responder {
    respond_results(state.post_service.get_many().await, PostTO::from)
}

#[get("/{id}")]
pub async fn get_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .post_service
            .get_by_id(id.into_inner())
            .await
            .map(PostTO::from),
    )
}

#[post("/")]
pub async fn create(state: Data<AppState>, post: Json<PostTO>) -> impl Responder {
    respond_result(
        state
            .post_service
            .create(Post::from(post.into_inner()))
            .await
            .map(PostTO::from),
    )
}

#[put("/{id}")]
pub async fn update(state: Data<AppState>, post: Json<PostTO>) -> impl Responder {
    respond_result(
        state
            .post_service
            .update(Post::from(post.into_inner()))
            .await
            .map(PostTO::from),
    )
}

#[delete("/{id}")]
pub async fn delete(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(state.post_service.delete_by_id(id.into_inner()).await)
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/posts")
            .service(get_many)
            .service(get_by_id)
            .service(create)
            .service(update)
            .service(delete),
    );
}
