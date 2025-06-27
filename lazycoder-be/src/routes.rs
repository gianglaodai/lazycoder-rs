use actix_web::web;
use crate::infras::controllers::{post_controller, user_controller, auth_controller};

pub fn config(cfg: &mut web::ServiceConfig) {
    post_controller::routes(cfg);
    user_controller::routes(cfg);
    auth_controller::routes(cfg);
}
