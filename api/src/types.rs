use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpMessage, HttpRequest};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

#[derive(Clone, Serialize, Deserialize)]
pub struct JwtPayload {
    pub user_id: i64,
    pub iat: usize,
    pub iss: String,
    pub exp: usize,
}

// TODO куда это блять вставить
#[derive(Clone, Debug)]
pub enum UserType {
    User(i64),
    Admin(i64),
    None,
}

impl FromRequest for UserType {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            match req.extensions().get::<UserType>() {
                Some(user_type) => Ok(user_type.to_owned()),
                None => Err(actix_web::error::ErrorInternalServerError(
                    "Cannot parse user type",
                )),
            }
        })
    }
}
