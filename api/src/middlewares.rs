use crate::state::AppState;
use crate::types::{JwtPayload, OptionalUser};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::{error, web, Error, HttpMessage};
use awc::body::MessageBody;
use entity::prelude::User;
use entity::user;
use jsonwebtoken::{decode, DecodingKey, Validation};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

pub async fn auth_middleware(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    req.extensions_mut().insert::<OptionalUser>(None);

    let state = req
        .extract::<web::Data<AppState>>()
        .await
        .map_err(error::ErrorInternalServerError)?;

    let user_id = req
        .cookie("token")
        .and_then(|cookie| {
            decode::<JwtPayload>(
                &cookie.value(),
                &DecodingKey::from_secret(state.env.secret.as_bytes()),
                &Validation::default(),
            )
            .ok()
        })
        .map(|token| token.claims.user_id);

    match user_id {
        Some(user_id) => {
            req.extensions_mut().insert::<OptionalUser>(
                User::find()
                    .filter(user::Column::Id.eq(user_id))
                    .one(&state.conn)
                    .await
                    .map_err(error::ErrorInternalServerError)?,
            );
        }
        _ => (),
    }
    
    next.call(req).await
}
