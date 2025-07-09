use crate::services::RecordService;
use crate::extractors::UserType;
use actix_web::{delete, error, web, HttpResponse};

#[delete("/{id}")]
pub async fn delete_handler(
    path: web::Path<(i64,)>,
    record_service: web::Data<RecordService>,
    user_type: UserType,
) -> error::Result<HttpResponse> {
    let record_id = path.0;
    // TODO Check access in middleware
    match user_type {
        UserType::None => Err(error::ErrorUnauthorized("Unauthorized")),
        UserType::Admin(_) => Ok(()),
        UserType::User(user) => {
            // find owner of record
            let owner_id = record_service
                .find_record_by_id(record_id)
                .await?
                .ok_or(error::ErrorNotFound("Record not found"))?
                .owner_id
                .ok_or(error::ErrorForbidden("Record is anonymous"))?;

            // check the record belongs to user
            if owner_id != user.id {
                Err(error::ErrorForbidden("This is not your record"))
            } else {
                Ok(())
            }
        }
    }?;

    let delete_result = record_service.delete_by_id(record_id).await?;

    if delete_result.rows_affected == 0 {
        Ok(HttpResponse::NotModified().finish())
    } else {
        Ok(HttpResponse::Ok().finish())
    }
}
