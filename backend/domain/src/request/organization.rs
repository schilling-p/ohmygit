use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct FetchOrganizationsRequest {
    pub user_email: String,
}