use crate::record::name::{generate_name, validate_name};
use crate::types::AppState;
use crate::utils::get_user_id_from_cookie;
use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{error, post, web, HttpRequest, HttpResponse, Responder};
use aws_sdk_s3::primitives::{ByteStream, SdkBody};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use chrono::Utc;
use entity::prelude::Record;
use entity::record;
use k_snowflake::create_snowflake;
use sea_orm::ActiveValue::Set;
use sea_orm::{EntityTrait, TryIntoModel};
use std::io::Read;
use url::Url;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100 MB")]
    file: TempFile,
    name: Option<Text<String>>,
}

#[post("/upload")]
pub async fn upload(
    MultipartForm(form): MultipartForm<UploadForm>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> error::Result<impl Responder> {
    // validate or generate name
    let name = match form.name {
        Some(name) => validate_name(&state.conn, name.to_string()).await?,
        None => generate_name(&state.conn).await,
    };
    // set owner id if user authorized else record will be anonymous
    let owner_id = get_user_id_from_cookie(&req, &state);

    // read file
    let mut file = form.file.file.into_file();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let file = ByteStream::new(SdkBody::from(buffer.clone()));

    // file meta
    let mime_type = form.file.content_type.map(|mime| mime.to_string());
    let file_ext = form
        .file
        .file_name
        .and_then(|name| name.split(".").last().map(|ext| ext.to_string()));
    let digest = md5::compute(buffer.clone());

    // upload
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);
    client
        .put_object()
        .bucket(state.env.bucket_name.clone())
        .key(format!(
            "{}.{}",
            name.to_lowercase(),
            file_ext.clone().unwrap_or_else(|| "txt".to_string())
        ))
        .body(file)
        .content_type(
            mime_type
                .clone()
                .unwrap_or_else(|| "application/octet-stream".to_string()),
        )
        .content_md5(BASE64_STANDARD.encode(digest.as_ref()))
        .send()
        .await
        .map_err(|err| {
            dbg!(&err);
            error::ErrorInternalServerError(format!("Unable to upload file\n{}", err))
        })?;

    // create url to file
    let cdn_url = Url::parse(&state.env.cdn_url).expect("Invalid cdn url in env");
    let file_url = cdn_url
        .join(
            format!("{}.{}", name, file_ext.unwrap_or_else(|| "txt".to_string()))
                .to_lowercase()
                .as_str(),
        )
        .unwrap();

    // write to database
    let record = record::ActiveModel {
        id: Set(create_snowflake().to_decimal()),
        owner_id: Set(owner_id),
        name: Set(name.to_lowercase()),
        visible_name: Set(name.clone()),
        created_at: Set(Utc::now().naive_utc()),
        url: Set(file_url.to_string()),
        is_file: Set(true),
        hash: Set(Some(format!("{:x}", digest))),
        mime_type: Set(mime_type),
    };

    Record::insert(record.clone())
        .exec(&state.conn)
        .await
        .map_err(|_| error::ErrorInternalServerError("Error during database insert"))?;

    Ok(
        HttpResponse::Ok().json(record.try_into_model().map_err(|_| {
            error::ErrorInternalServerError("Error generating response, but file writed")
        })?),
    )
}
