use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct LoginResponse {
    pub message: &'static str,
    pub user_email: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct SignupResponse {
    pub message: &'static str,
    pub user_email: String,
}