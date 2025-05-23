use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct LoginRequest {
    pub identifier: UserIdentifier,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(tag = "type", content = "value")]
pub enum UserIdentifier {
    Email(String),
    Username(String),
}