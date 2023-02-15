use std::error::Error;

use actix_web::body::BoxBody;
use actix_web::error::PayloadError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use log::{log, Level};
use thiserror::Error;

use crate::data_source::{DataSourceError, MediaImportError};
use crate::http_server::instance::LoginError;
use crate::BucketError;
use crate::http_models::{ErrorResponse};

#[derive(Debug, Error)]
pub enum WebError {
    #[error("Cannot parse parts of the request")]
    ParseError,

    #[error("Missing \"Authorization\" header or \"token\" query parameter")]
    MissingAuthToken,

    #[error("Missing \"x-bucket-id\" header or \"bucket-id\" query parameter")]
    MissingBucketId,

    #[error("The requested bucket id does exist")]
    InstanceNotFound,

    #[error("The given authorization token is not valid")]
    InvalidAuthToken,

    #[error("Internal error")]
    InternalError(#[from] BucketError),

    #[error("Internal error while fetching data")]
    InternalDataSourceError(DataSourceError),

    #[error("The requested resource doesn't exist")]
    ResourceNotFound,

    #[error("The requested endpoint doesn't exist")]
    EndpointNotFound,

    #[error("The submitted data conflicts with existing data")]
    Duplicate,

    #[error("Password required")]
    PasswordRequired,

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Internal IO error")]
    IOError(#[from] std::io::Error),

    #[error("Error reading request body")]
    ReadBodyError(#[from] PayloadError),

    #[error("Missing mime type")]
    MissingMimeType,

    #[error("Missing program \"{0}\" on $PATH")]
    MissingProgram(&'static str),

    #[error("Unsupported mime type")]
    UnsupportedMimeType,

    #[error("Unexpected program output")]
    UnexpectedProgramOutput,
}

impl From<DataSourceError> for WebError {
    fn from(value: DataSourceError) -> Self {
        match value {
            DataSourceError::Duplicate => Self::Duplicate,
            DataSourceError::NotFound => Self::ResourceNotFound,

            e => Self::InternalDataSourceError(e),
        }
    }
}

impl WebError {
    pub fn inner_error(&self) -> Option<&dyn Error> {
        match self {
            Self::InternalError(e) => Some(e),
            Self::InternalDataSourceError(e) => Some(e),
            Self::IOError(e) => Some(e),
            Self::ReadBodyError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<MediaImportError> for WebError {
    fn from(value: MediaImportError) -> Self {
        match value {
            MediaImportError::MissingProgram { name } => Self::MissingProgram(name),
            MediaImportError::UnsupportedMimeType => Self::UnsupportedMimeType,
            MediaImportError::UnexpectedOutput => Self::UnexpectedProgramOutput,
            MediaImportError::DataSourceError(e) => Self::InternalDataSourceError(e),
            MediaImportError::IOError(e) => Self::IOError(e),
        }
    }
}

impl From<LoginError> for WebError {
    fn from(value: LoginError) -> Self {
        match value {
            LoginError::InvalidPassword => Self::InvalidPassword,
            LoginError::PasswordRequired => Self::PasswordRequired,
            LoginError::LoadingError(e) => Self::InternalError(e),
            LoginError::FetchingError(e) => Self::InternalDataSourceError(e),
        }
    }
}

impl actix_web::error::ResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebError::ParseError => StatusCode::UNPROCESSABLE_ENTITY,
            WebError::MissingAuthToken => StatusCode::UNAUTHORIZED,
            WebError::MissingBucketId => StatusCode::UNAUTHORIZED,
            WebError::InstanceNotFound => StatusCode::NOT_FOUND,
            WebError::InvalidAuthToken => StatusCode::UNAUTHORIZED,
            WebError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::InternalDataSourceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::ResourceNotFound => StatusCode::NOT_FOUND,
            WebError::Duplicate => StatusCode::CONFLICT,
            WebError::PasswordRequired => StatusCode::UNPROCESSABLE_ENTITY,
            WebError::InvalidPassword => StatusCode::UNAUTHORIZED,
            WebError::EndpointNotFound => StatusCode::NOT_FOUND,
            WebError::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::ReadBodyError(_) => StatusCode::BAD_REQUEST,
            WebError::MissingMimeType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            WebError::MissingProgram(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::UnsupportedMimeType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            WebError::UnexpectedProgramOutput => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let status = self.status_code();

        let (target, level) = if status.is_client_error() {
            ("Client", Level::Warn)
        } else {
            ("Server", Level::Error)
        };

        log!(level, "{target} error: {self}",);

        HttpResponse::build(status).json(ErrorResponse {
            status: status.as_u16(),
            status_text: status.canonical_reason().unwrap_or("Unknown"),
            message: format!("{self}"),
            inner_error: self.inner_error().map(|e| format!("{e:?}")),
        })
    }
}
