use crate::state::AppState;
use crate::types::UserType;
use crate::utils::get_user_id_from_cookie;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::{error, web, Error, HttpMessage};
use awc::body::MessageBody;
use entity::prelude::User;
use entity::user;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

pub async fn auth_middleware(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    req.extensions_mut().insert(UserType::None);

    let state = req
        .extract::<web::Data<AppState>>()
        .await
        .map_err(error::ErrorInternalServerError)?;

    let user_id = get_user_id_from_cookie(req.request(), &state);

    match user_id {
        None => next.call(req).await,
        Some(user_id) => {
            let user = User::find()
                .filter(user::Column::Id.eq(user_id))
                .one(&state.conn)
                .await
                .map_err(error::ErrorUnauthorized)?;

            match user {
                None => next.call(req).await,
                Some(user) => {
                    req.extensions_mut().insert(if user.is_admin {
                        UserType::Admin(user_id)
                    } else {
                        UserType::User(user_id)
                    });

                    next.call(req).await
                }
            }
        }
    }
}
