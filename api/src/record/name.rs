use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use entity::prelude::Record;
use entity::record;
use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::json;
use std::fmt::Display;

static CHARSET: Lazy<Vec<&str>> = Lazy::new(|| {
    let char_str = "abcdefghijklmnopqrstuvwxyz0123456789";
    let chars: Vec<&str> = char_str.split("").collect();
    chars[1..chars.len() - 1].to_vec()
});

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9a-zA-Z._\-]+$").unwrap());

pub async fn match_name(
    conn: &DatabaseConnection,
    name: &Option<String>,
) -> Result<String, NameError> {
    match name {
        Some(name) => validate_name(conn, name.clone()).await,
        None => Ok(generate_name(conn).await),
    }
}

async fn validate_name(conn: &DatabaseConnection, name: String) -> Result<String, NameError> {
    if name == "" {
        return Ok(generate_name(conn).await);
    };
    if name_is_used(conn, &name).await {
        return Err(NameError("This name is used"));
    };
    if &name.chars().count() < &3 {
        return Err(NameError("Name is too short"));
    };
    if !RE.is_match(&name) {
        return Err(NameError("Name contains invalid characters"));
    }
    Ok(name)
}

async fn generate_name(conn: &DatabaseConnection) -> String {
    let mut name = gen();
    while name_is_used(conn, &name).await {
        name = gen();
    }
    name
}

async fn name_is_used(conn: &DatabaseConnection, name: &String) -> bool {
    match Record::find()
        .filter(record::Column::Name.eq(name.to_lowercase()))
        .one(conn)
        .await
    {
        Ok(result) => result.is_some(),
        Err(_) => false,
    }
}

fn gen() -> String {
    fastrand::choose_multiple(CHARSET.clone().into_iter(), 3).join("")
}

#[derive(Debug)]
pub struct NameError(&'static str);

impl ResponseError for NameError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::BAD_REQUEST)
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

impl Display for NameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = json!({
            "type": "name",
            "msg": self.0
        });

        write!(f, "{}", value)
    }
}
