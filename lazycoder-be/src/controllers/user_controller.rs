use crate::controllers::response_result::{respond_result, respond_results};
use crate::define_to_with_common_fields;
use crate::services::user_service::User;
use crate::state::AppState;
use actix_web::web::{Data, Json, Path, ServiceConfig, scope};
use actix_web::{Responder, delete, get, post, put};

define_to_with_common_fields!(UserTO {
    username: String,
    email: String,
    password: String
});

impl From<UserTO> for User {
    fn from(user: UserTO) -> Self {
        Self {
            id: user.id,
            uid: user.uid,
            created_at: user.created_at,
            updated_at: user.updated_at,
            username: user.username,
            email: user.email,
            password: user.password,
        }
    }
}

impl From<User> for UserTO {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            uid: user.uid,
            created_at: user.created_at,
            updated_at: user.updated_at,
            username: user.username,
            email: user.email,
            password: user.password,
        }
    }
}

#[get("/")]
pub async fn get_users(state: Data<AppState>) -> impl Responder {
    respond_results(state.user_service.get_users().await, UserTO::from)
}

#[get("/{id}")]
pub async fn get_user_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .user_service
            .get_user_by_id(id.into_inner())
            .await
            .map(UserTO::from),
    )
}

#[post("/")]
pub async fn create_user(state: Data<AppState>, user: Json<UserTO>) -> impl Responder {
    respond_result(
        state
            .user_service
            .create_user(User::from(user.into_inner()))
            .await
            .map(UserTO::from),
    )
}

#[put("/{id}")]
pub async fn update_user(state: Data<AppState>, user: Json<UserTO>) -> impl Responder {
    respond_result(
        state
            .user_service
            .update_user(User::from(user.into_inner()))
            .await
            .map(UserTO::from),
    )
}

#[delete("/{id}")]
pub async fn delete_user(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(state.user_service.delete_user(id.into_inner()).await)
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/users")
            .service(get_users)
            .service(get_user_by_id)
            .service(create_user)
            .service(update_user)
            .service(delete_user),
    );
}
