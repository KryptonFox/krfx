use crate::errors::{RecordRequestError, RecordRequestErrorType};
use crate::state::AppState;
use actix_web::error;
use entity::prelude::Record;
use entity::record;
use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, EntityTrait};
use std::sync::Arc;

static CHARSET: Lazy<Vec<&str>> = Lazy::new(|| {
    let char_str = "abcdefghijklmnopqrstuvwxyz0123456789";
    let chars: Vec<&str> = char_str.split("").collect();
    chars[1..chars.len() - 1].to_vec()
});

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9a-zA-Z._\-]+$").unwrap());

pub struct NameService {
    app_state: Arc<AppState>,
}

impl NameService {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }

    pub async fn unwrap_name(&self, name: &Option<String>) -> error::Result<String> {
        Ok(match name {
            Some(name) => self.validate(name.clone()).await?,
            _ => self.generate().await,
        })
    }

    pub async fn validate(&self, name: String) -> Result<String, RecordRequestError> {
        if name == "" {
            return Ok(self.generate().await);
        };
        if self.name_is_used(&name).await {
            return Err(Self::err("This name is used"));
        };
        if &name.chars().count() < &3 {
            return Err(Self::err("Name is too short"));
        };
        if !RE.is_match(&name) {
            return Err(Self::err("Name contains invalid characters"));
        }
        Ok(name)
    }

    pub async fn generate(&self) -> String {
        let mut name = Self::gen();
        while self.name_is_used(&name).await {
            name = Self::gen();
        }
        name
    }

    async fn name_is_used(&self, name: &String) -> bool {
        match Record::find()
            .filter(record::Column::Name.eq(name.to_lowercase()))
            .one(&self.app_state.conn)
            .await
        {
            Ok(result) => result.is_some(),
            Err(_) => false,
        }
    }

    fn err(msg: &str) -> RecordRequestError {
        RecordRequestError::new(RecordRequestErrorType::Name, msg.to_string())
    }

    fn gen() -> String {
        fastrand::choose_multiple(CHARSET.clone().into_iter(), 3).join("")
    }
}
