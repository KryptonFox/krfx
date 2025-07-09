use serde::{Deserialize, Serialize};
use entity::user;

pub type OptionalUser = Option<user::Model>;

#[derive(Clone, Serialize, Deserialize)]
pub struct JwtPayload {
    pub user_id: i64,
    pub iat: usize,
    pub iss: String,
    pub exp: usize,
}
