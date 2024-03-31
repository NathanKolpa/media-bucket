use crate::data_source::DataSourceError;
use crate::model::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct BucketInfo {
    pub id: u64,
    pub name: String,
    pub password_protected: bool,
    pub encrypted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct AuthRequest {
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct AuthResponse {
    pub token: String,
    pub share_token: String,
    pub active_tokens: u64,
    pub now: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub lifetime: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct ErrorResponse {
    pub status: u16,
    pub status_text: String,
    pub message: String,
    pub inner_error: Option<String>,
}

impl Into<DataSourceError> for ErrorResponse {
    fn into(self) -> DataSourceError {
        match self.status {
            404 => DataSourceError::NotFound,
            409 => DataSourceError::Duplicate,
            _ => DataSourceError::UnhandledError {
                message: self.message,
                inner_error: self.inner_error,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct CreateFullPostResponse {
    pub batch: ImportBatch,
    pub posts: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct CreateTagGroupRequest {
    pub name: String,
    pub hex_color: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct CreateTagRequest {
    pub name: String,
    pub group: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct UpdateTagRequest {
    pub name: String,
    pub group: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub source: Option<Url>,
    pub tag_ids: Vec<u64>,
}
