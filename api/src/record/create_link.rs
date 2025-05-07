use crate::record::name::{generate_name, validate_name};
use crate::types::AppState;
use crate::utils::get_user_id_from_cookie;
use actix_web::{error, post, web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use entity::record;
use k_snowflake::create_snowflake;
use sea_orm::ActiveValue::Set;
use sea_orm::ActiveModelTrait;
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
    req: HttpRequest,
) -> error::Result<impl Responder> {
    // validate or generate name
    let name = match &payload.name {
        Some(name) => validate_name(&state.conn, name.to_string()).await?,
        None => generate_name(&state.conn).await,
    };
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
    let owner_id = get_user_id_from_cookie(&req, &state);

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
