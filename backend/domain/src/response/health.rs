use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct HealthResponse {
    pub message: &'static str,
}