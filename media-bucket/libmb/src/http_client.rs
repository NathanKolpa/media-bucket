use async_trait::async_trait;
use reqwest::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap};
use thiserror::Error;
use url::Url;
use crate::data_source::{DataSourceError, PageParams, PostDataSource};
use crate::http_models::{AuthRequest, AuthResponse, BucketInfo};
use crate::model::{Page, Post};

const USER_AGENT: &'static str = "libmb/1.0";

#[derive(Error, Debug)]
pub enum HttpDataSourceError {
    #[error("Client build error")]
    ClientBuildError(reqwest::Error),

    #[error("Fetch bucket error")]
    FetchBucketError(reqwest::Error),

    #[error("Login error")]
    LoginError(reqwest::Error),
}

pub struct HttpDataSource {
    client: Client,
    base: Url,
}

impl HttpDataSource {
    pub async fn open(url: Url, password: Option<&str>) -> Result<Self, HttpDataSourceError> {
        let open_client = Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .map_err(|e| HttpDataSourceError::ClientBuildError(e))?;

        let bucket: BucketInfo = open_client.get(url.as_str())
            .send()
            .await
            .map_err(|e| HttpDataSourceError::FetchBucketError(e))?
            .json()
            .await
            .map_err(|e| HttpDataSourceError::FetchBucketError(e))?;

        let auth_response: AuthResponse = open_client.post(format!("{url}/auth"))
            .json(&AuthRequest {
                password: password.map(|s| s.to_string())
            })
            .send()
            .await
            .map_err(|e| HttpDataSourceError::LoginError(e))?
            .json()
            .await
            .map_err(|e| HttpDataSourceError::FetchBucketError(e))?;

        let mut default_headers = HeaderMap::new();
        default_headers.insert(AUTHORIZATION, auth_response.token.parse().unwrap());

        let client = Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(default_headers)
            .build()
            .map_err(|e| HttpDataSourceError::ClientBuildError(e))?;

        Ok(HttpDataSource {
            client,
            base: url,
        })
    }
}
