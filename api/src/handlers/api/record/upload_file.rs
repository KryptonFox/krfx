use crate::extractors::UserId;
use crate::services::{NameService, RecordService};
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::{error, post, web, HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100 MB")]
    file: TempFile,
    name: Option<Text<String>>,
}

#[derive(Serialize)]
struct UploadFileResponse {
    id: String,
    owner_id: Option<String>,
    link: String,
    cdn_link: String,
    created_at: NaiveDateTime,
    hash: Option<String>,
    mime_type: Option<String>,
    is_temp: bool,
    expires_at: Option<NaiveDateTime>,
}

#[post("/upload")]
pub async fn upload_file_handler(
    record_service: web::Data<RecordService>,
    name_service: web::Data<NameService>,
    MultipartForm(form): MultipartForm<UploadForm>,
    UserId(owner_id): UserId,
    req: HttpRequest,
) -> error::Result<HttpResponse> {
    let name = form.name.map(|n| n.to_string());

    let record = record_service
        .upload_file(name_service.clone(), owner_id, form.file, &name)
        .await?;

    let mut link = req.full_url().clone();
    link.set_path(record.visible_name.as_str());

    let response = UploadFileResponse {
        id: record.id.to_string(),
        owner_id: record.owner_id.map(|id| id.to_string()),
        link: link.to_string(),
        cdn_link: record.url,
        hash: record.hash,
        mime_type: record.mime_type,
        created_at: record.created_at,
        is_temp: record.is_temp,
        expires_at: record.expires_at,
    };

    Ok(HttpResponse::Ok().json(response))
}
