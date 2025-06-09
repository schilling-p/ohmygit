use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct LoginRequest {
    pub identifier: UserIdentifier,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(tag = "type", content = "value")]
pub enum UserIdentifier {
    Id(Uuid),
    Email(String),
    Username(String),
}