use crate::extractors::UserId;
use crate::services::{NameService, RecordService};
use actix_web::{error, post, web, HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
struct CreateLinkPayload {
    url: String,
    name: Option<String>,
}

#[derive(Serialize)]
struct CreateLinkResponse {
    id: String,
    owner_id: Option<String>,
    link: String,
    created_at: NaiveDateTime,
    is_temp: bool,
    expires_at: Option<NaiveDateTime>,
}

#[post("/create_link")]
pub async fn create_link_handler(
    record_service: web::Data<RecordService>,
    name_service: web::Data<NameService>,
    payload: web::Json<CreateLinkPayload>,
    UserId(owner_id): UserId,
    req: HttpRequest,
) -> error::Result<HttpResponse> {
    let record = record_service
        .create_link(name_service.clone(), owner_id, &payload.url, &payload.name)
        .await?;

    let mut link = req.full_url().clone();
    link.set_path(record.visible_name.as_str());

    let response = CreateLinkResponse {
        id: record.id.to_string(),
        owner_id: record.owner_id.map(|id| id.to_string()),
        link: link.to_string(),
        created_at: record.created_at,
        is_temp: record.is_temp,
        expires_at: record.expires_at,
    };

    Ok(HttpResponse::Ok().json(response))
}
