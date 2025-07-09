use crate::errors::{RecordRequestError, RecordRequestErrorType};
use crate::services::name::NameService;
use crate::state::AppState;
use actix_multipart::form::tempfile::TempFile;
use actix_web::{error, web};
use aws_sdk_s3::primitives::{ByteStream, SdkBody};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use chrono::Utc;
use entity::prelude::Record;
use entity::record;
use k_snowflake::create_snowflake;
use md5::Digest;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::{ColumnTrait, DeleteResult};
use sea_orm::{ModelTrait, QueryFilter};
use std::io::Read;
use std::sync::Arc;
use url::Url;

pub struct RecordService {
    app_state: Arc<AppState>,
    s3_client: aws_sdk_s3::Client,
}

impl RecordService {
    pub async fn new(app_state: Arc<AppState>) -> Self {
        let config = aws_config::load_from_env().await;
        let s3_client = aws_sdk_s3::Client::new(&config);
        Self {
            app_state,
            s3_client,
        }
    }

    pub async fn find_record_by_id(&self, id: i64) -> error::Result<Option<record::Model>> {
        Record::find()
            .filter(record::Column::Id.eq(id))
            .one(&self.app_state.conn)
            .await
            .map_err(error::ErrorInternalServerError)
    }

    pub async fn find_record_by_name(&self, name: &String) -> error::Result<Option<record::Model>> {
        Record::find()
            .filter(record::Column::Name.eq(name.to_lowercase()))
            .one(&self.app_state.conn)
            .await
            .map_err(error::ErrorInternalServerError)
    }

    pub async fn delete_by_id(&self, id: i64) -> error::Result<DeleteResult> {
        // find record to delete
        let record = self
            .find_record_by_id(id)
            .await?
            .ok_or(error::ErrorNotFound("Record not found"))?;

        // if is file -> delete file
        if record.is_file {
            let url = Url::parse(record.url.as_str()).map_err(error::ErrorInternalServerError)?;

            let key = url
                .path_segments()
                .and_then(|segments| segments.last())
                .ok_or(error::ErrorInternalServerError("Wrong file URL"))?;

            self.delete_file(key).await?;
        }
        
        // delete and return
        record
            .delete(&self.app_state.conn)
            .await
            .map_err(error::ErrorInternalServerError)
    }

    pub async fn create_link(
        &self,
        name_service: web::Data<NameService>,
        owner_id: Option<i64>,
        url: &String,
        name: &Option<String>,
    ) -> error::Result<record::Model> {
        let name = name_service.unwrap_name(name).await?;

        let url = Url::parse(url.as_str())
            .map(|n| n.to_string())
            .map_err(|_| {
                RecordRequestError::new(RecordRequestErrorType::Url, "Invalid URL".to_string())
            })?;

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

        // TODO maybe update response (str id, shortened_url)
        record
            .insert(&self.app_state.conn)
            .await
            .map_err(error::ErrorInternalServerError)
    }

    pub async fn upload_file(
        &self,
        name_service: web::Data<NameService>,
        owner_id: Option<i64>,
        file: TempFile,
        name: &Option<String>,
    ) -> error::Result<record::Model> {
        let name = name_service.unwrap_name(name).await?;
        let (file_url, digest, mime_type) = self.upload_file_to_sdk(file, &name).await?;

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
            mime_type: Set(Some(mime_type)),
            ..Default::default()
        };

        record
            .insert(&self.app_state.conn)
            .await
            .map_err(error::ErrorInternalServerError)
    }

    async fn delete_file(&self, key: &str) -> error::Result<()> {
        self.s3_client
            .delete_object()
            .bucket(&self.app_state.env.bucket_name)
            .key(key)
            .send()
            .await
            .map_err(error::ErrorInternalServerError)?;

        Ok(())
    }

    async fn upload_file_to_sdk(
        &self,
        file: TempFile,
        name: &String,
    ) -> error::Result<(Url, Digest, String)> {
        // read file
        let mut fs_file = file.file.into_file();
        let mut buffer = Vec::new();
        fs_file.read_to_end(&mut buffer)?;
        let md5_digest = md5::compute(&buffer);
        let sdk_file = ByteStream::new(SdkBody::from(buffer));

        // file meta
        let mime_type = file
            .content_type
            .map(|mime| mime.to_string())
            .unwrap_or("application/octet-stream".to_string());
        let file_ext = file
            .file_name
            .and_then(|name| name.split(".").last().map(|ext| ext.to_string()))
            .unwrap_or("txt".to_string());

        // upload
        self.s3_client
            .put_object()
            .bucket(&self.app_state.env.bucket_name)
            .key(format!("{}.{}", name.to_lowercase(), file_ext.clone()))
            .body(sdk_file)
            .content_type(&mime_type)
            .content_md5(BASE64_STANDARD.encode(md5_digest.as_ref()))
            .content_disposition("inline")
            .send()
            .await
            .map_err(|err| {
                dbg!(&err);
                error::ErrorInternalServerError(format!("Unable to upload file\n{}", err))
            })?;

        // create url to file
        let cdn_url = Url::parse(&self.app_state.env.cdn_url).expect("Invalid cdn url in env");
        let file_url = cdn_url
            .join(format!("{}.{}", name, file_ext.to_lowercase().as_str()).as_str())
            .map_err(error::ErrorInternalServerError)?;

        Ok((file_url, md5_digest, mime_type))
    }
}
