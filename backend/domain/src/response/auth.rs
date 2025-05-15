use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct LoginResponse {
    //pub session_id: String,
    pub message: &'static str,
    pub user_email: String,
    pub username: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct SignupResponse {
    pub message: &'static str,
    pub user_email: String,
}