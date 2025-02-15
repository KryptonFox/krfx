use crate::types::AppState;
use actix_web::web::Redirect;
use actix_web::{error, get, web, Either, HttpResponse, Responder};
use entity::prelude::Record;
use entity::record;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[get("/{link}")]
pub async fn link_service(
    state: web::Data<AppState>,
    client: web::Data<awc::Client>,
    path: web::Path<(String,)>,
) -> error::Result<impl Responder> {
    // find record. Return 404 if not found
    let Some(found_record) = Record::find()
        .filter(record::Column::Name.eq(path.0.to_lowercase()))
        .one(&state.conn)
        .await
        .map_err(error::ErrorInternalServerError)?
    else {
        return Err(error::ErrorNotFound("Record not found"));
    };
    // if not file return redirect
    if !found_record.is_file {
        return Ok(Either::Left(Redirect::to(found_record.url).permanent()));
    };
    // fetch file from cdn
    let res = client
        .get(found_record.url)
        .send()
        .await
        .map_err(error::ErrorInternalServerError)?;

    // build response
    let mut resp = HttpResponse::build(res.status());

    // if content type specified insert it in response
    if let Some(content_type) = found_record.mime_type {
        resp.append_header(("Content-Type", content_type));
    }
    // return response
    Ok(Either::Right(resp.streaming(res)))
}
