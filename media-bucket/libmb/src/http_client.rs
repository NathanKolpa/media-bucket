use async_trait::async_trait;
use reqwest::{Client, RequestBuilder};
use reqwest::header::{AUTHORIZATION, HeaderMap};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use thiserror::Error;
use url::Url;
use crate::data_source::*;
use crate::http_models::{AuthRequest, AuthResponse, BucketInfo, ErrorResponse};
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

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Password required")]
    PasswordRequired,

    #[error("Api error")]
    ApiError(ErrorResponse),
}


pub struct HttpDataSource {
    client: Client,
    base: Url,
    info: BucketInfo,
}

impl HttpDataSource {
    pub async fn open(url: Url, password: Option<&str>) -> Result<Self, HttpDataSourceError> {
        let open_client = Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .map_err(|e| HttpDataSourceError::ClientBuildError(e))?;

        let info: BucketInfo = open_client.get(url.as_str())
            .send()
            .await
            .map_err(|e| HttpDataSourceError::FetchBucketError(e))?
            .json()
            .await
            .map_err(|e| HttpDataSourceError::FetchBucketError(e))?;

        let auth_response = Self::send_request::<AuthResponse>(open_client.post(format!("{url}/auth"))
            .json(&AuthRequest {
                password: password.map(|s| s.to_string())
            }))
            .await
            .map_err(|e| HttpDataSourceError::LoginError(e))?
            .map_err(|e| match e.status {
                422 => HttpDataSourceError::PasswordRequired,
                401 => HttpDataSourceError::InvalidPassword,
                _ => HttpDataSourceError::ApiError(e)
            })?;

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
            info,
        })
    }

    pub fn is_encrypted(&self) -> bool {
        self.info.encrypted
    }

    async fn send_request<T: DeserializeOwned>(req: RequestBuilder) -> Result<Result<T, ErrorResponse>, reqwest::Error> {
        let res = req.send().await?;

        if res.status().is_success() {
            Ok(Ok(res.json::<T>().await?))
        }
        else {
            Ok(Err(res.json::<ErrorResponse>().await?))
        }
    }
}

#[async_trait]
impl DataSource for HttpDataSource {
    fn blobs(&self) -> &dyn BlobDataSource {
        todo!()
    }

    fn media(&self) -> &dyn MediaDataSource {
        todo!()
    }

    fn content(&self) -> &dyn ContentDataSource {
        todo!()
    }

    fn post_items(&self) -> &dyn PostItemDataSource {
        todo!()
    }

    fn posts(&self) -> &dyn PostDataSource {
        todo!()
    }

    fn import_batches(&self) -> &dyn ImportBatchDataSource {
        todo!()
    }

    fn tags(&self) -> &dyn TagDataSource {
        todo!()
    }

    fn tag_groups(&self) -> &dyn TagGroupDataSource {
        todo!()
    }

    fn passwords(&self) -> &dyn PasswordDataSource {
        todo!()
    }

    fn media_import(&self) -> &dyn MediaImportDataSource {
        todo!()
    }

    fn cross(&self) -> &dyn CrossDataSource {
        todo!()
    }
}