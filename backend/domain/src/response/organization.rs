use serde::Serialize;
use crate::models::Organization;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ListOrganizationsResponse {
    pub organizations: Vec<Organization>,
    pub user_email: String,
}