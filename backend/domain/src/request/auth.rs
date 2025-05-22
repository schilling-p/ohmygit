use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum UserIdentifier {
    Email(String),
    Username(String),
}