use crate::services::RecordService;
use actix_web::web::Redirect;
use actix_web::{error, get, web, Either, HttpResponse};

#[get("/{link}")]
pub async fn link_handler(
    record_service: web::Data<RecordService>,
    client: web::Data<awc::Client>,
    path: web::Path<(String,)>,
) -> error::Result<Either<Redirect, HttpResponse>> {
    let record = record_service
        .find_record_by_name(&path.0)
        .await?
        .ok_or(error::ErrorNotFound("No record with this name was found"))?;

    // if not file return redirect
    if !record.is_file {
        return Ok(Either::Left(Redirect::to(record.url).permanent()));
    };
    // fetch file from cdn
    let res = client
        .get(record.url)
        .send()
        .await
        .map_err(error::ErrorInternalServerError)?;

    // build response
    let mut resp = HttpResponse::build(res.status());

    // if content type specified insert it in response
    if let Some(content_type) = record.mime_type {
        resp.append_header(("Content-Type", content_type));
    }
    // return response
    Ok(Either::Right(resp.streaming(res)))
}
