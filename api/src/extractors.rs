use crate::types::OptionalUser;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpMessage, HttpRequest};
use entity::user;
use std::future::Future;
use std::pin::Pin;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum UserType {
    User(user::Model),
    Admin(user::Model),
    None,
}

impl FromRequest for UserType {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            match req.extensions().get::<OptionalUser>().cloned() {
                Some(opt_user) => Ok(match opt_user {
                    Some(user) => {
                        if user.is_admin {
                            UserType::Admin(user)
                        } else {
                            UserType::User(user)
                        }
                    }
                    None => UserType::None,
                }),
                None => Err(actix_web::error::ErrorInternalServerError(
                    "Cannot parse user",
                )),
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct UserId(pub Option<i64>);

impl FromRequest for UserId {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            if let Some(opt_user) = req.extensions().get::<OptionalUser>() {
                Ok(match opt_user {
                    Some(user) => Self(Some(user.id)),
                    None => Self(None)
                })
            } else {
                Err(actix_web::error::ErrorInternalServerError(
                    "Cannot parse user",
                ))
            }
        })
    }
}