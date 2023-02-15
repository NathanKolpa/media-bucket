use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BucketInfo {
    pub id: u64,
    pub name: String,
    pub password_protected: bool,
    pub encrypted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthRequest {
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub token: String,
    pub active_tokens: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    pub status: u16,
    pub status_text: String,
    pub message: String,
    pub inner_error: Option<String>,
}