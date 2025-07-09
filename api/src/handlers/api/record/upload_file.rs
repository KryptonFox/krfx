use crate::extractors::UserId;
use crate::services::{NameService, RecordService};
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::{error, post, web, HttpResponse};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100 MB")]
    file: TempFile,
    name: Option<Text<String>>,
}

#[post("/upload")]
pub async fn upload_file_handler(
    record_service: web::Data<RecordService>,
    name_service: web::Data<NameService>,
    MultipartForm(form): MultipartForm<UploadForm>,
    UserId(owner_id): UserId,
) -> error::Result<HttpResponse> {
    // convert multipart text to string
    let name = form.name.map(|n| n.to_string());

    //
    let record = record_service
        .upload_file(name_service.clone(), owner_id, form.file, &name)
        .await?;

    Ok(HttpResponse::Ok().json(record))
}
