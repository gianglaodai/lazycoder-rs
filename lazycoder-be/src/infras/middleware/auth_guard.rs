use std::future::{ready, Ready};
use actix_session::SessionExt;
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use crate::infras::controllers::user_controller::UserTO;

#[derive(Debug)]
pub struct CurrentUser(pub UserTO);
impl FromRequest for CurrentUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let session = match req.get_session().get::<UserTO>("user") {
            Ok(opt) => opt,
            Err(err) => {
                log::error!("{}", err);
                return ready(Err(ErrorUnauthorized("Invalid session")))
            },
        };
        match session {
            Some(user) => ready(Ok(CurrentUser(user))),
            None => ready(Err(ErrorUnauthorized("Not logged in"))),
        }
    }
}