use crate::services::{NameService, RecordService};
use crate::types::UserType;
use actix_web::{error, post, web, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct CreateLinkPayload {
    url: String,
    name: Option<String>,
}

#[post("/create_link")]
pub async fn create_link_handler(
    record_service: web::Data<RecordService>,
    name_service: web::Data<NameService>,
    payload: web::Json<CreateLinkPayload>,
    user_type: UserType,
) -> error::Result<HttpResponse> {
    let record = record_service
        .create_link(name_service.clone(), user_type, &payload.url, &payload.name)
        .await?;

    Ok(HttpResponse::Ok().json(record))
}
