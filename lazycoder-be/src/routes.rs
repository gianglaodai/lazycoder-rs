use actix_web::web;
use crate::controllers::post_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    post_controller::routes(cfg);
}