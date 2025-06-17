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

impl UserIdentifier {
    pub fn as_email(&self) -> Option<&str> {
        match self {
            UserIdentifier::Email(email) => Some(email),
            _ => None,
        }
    }

    pub fn as_username(&self) -> Option<&str> {
        match self {
            UserIdentifier::Username(username) => Some(username),
            _ => None,
        }
    }

    pub fn as_uuid(&self) -> Option<Uuid> {
        match self {
            UserIdentifier::Id(id) => Some(*id),
            _ => None,
        }
    }
}