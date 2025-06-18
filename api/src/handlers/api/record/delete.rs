use crate::services::RecordService;
use crate::types::UserType;
use actix_web::{delete, error, web, HttpResponse};

#[delete("/{id}")]
pub async fn delete_handler(
    path: web::Path<(i64,)>,
    record_service: web::Data<RecordService>,
    user_type: UserType,
) -> error::Result<HttpResponse> {
    let record_id = path.0;
    match user_type {
        UserType::None => Err(error::ErrorUnauthorized("Unauthorized")),
        UserType::Admin(_) => {
            let result = record_service.delete_by_id(record_id).await?;
            if result.rows_affected == 0 {
                Ok(HttpResponse::NotModified().finish())
            } else {
                Ok(HttpResponse::Ok().finish())
            }
        }
        UserType::User(id) => {
            // find owner of record
            let owner_id = record_service
                .find_record_by_id(record_id)
                .await?
                .ok_or(error::ErrorNotFound("Record not found"))?
                .owner_id
                .ok_or(error::ErrorForbidden("Record is anonymous"))?;

            // check the record belongs to user
            if owner_id == id {
                let result = record_service.delete_by_id(record_id).await?;
                if result.rows_affected == 0 {
                    Ok(HttpResponse::NotModified().finish())
                } else {
                    Ok(HttpResponse::Ok().finish())
                }
            } else {
                Ok(HttpResponse::Forbidden().body("This is not your record"))
            }
        }
    }
}