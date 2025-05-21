use crate::record::name::match_name;
use crate::types::{AppState, UserType};
use actix_web::{error, post, web, HttpResponse, Responder};
use chrono::Utc;
use entity::record;
use k_snowflake::create_snowflake;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use serde::Deserialize;
use serde_json::json;
use url::Url;

#[derive(Debug, Deserialize, Clone)]
struct CreateLinkPayload {
    url: String,
    name: Option<String>,
}

#[post("/create_link")]
pub async fn create_link(
    state: web::Data<AppState>,
    payload: web::Json<CreateLinkPayload>,
    user_type: UserType,
) -> error::Result<impl Responder> {
    // validate or generate name
    let name = match_name(&state.conn, &payload.name).await?;
    // validate URL
    let url = match Url::parse(&payload.url) {
        Ok(url) => url.to_string(),
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(json!({
                "type":"url",
                "msg":"Invalid URL",
            })))
        }
    };
    // set owner id if user authorized else record will be anonymous
    let owner_id = match user_type {
        UserType::None => None,
        UserType::Admin(owner_id) | UserType::User(owner_id) => Some(owner_id),
    };

    // write to database
    let record = record::ActiveModel {
        id: Set(create_snowflake().to_decimal()),
        owner_id: Set(owner_id),
        name: Set(name.to_lowercase()),
        visible_name: Set(name.clone()),
        created_at: Set(Utc::now().naive_utc()),
        url: Set(url),
        ..Default::default()
    };

    let record = record
        .insert(&state.conn)
        .await
        .map_err(|_| error::ErrorInternalServerError("Error during database insert"))?;

    Ok(HttpResponse::Ok().json(record))
}
