use std::path::Path;

use async_trait::async_trait;
use mediatype::MediaTypeBuf;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use thiserror::Error;
use tokio::fs::File;
use url::Url;
use uuid::Uuid;

use crate::data_source::*;
use crate::http_models::{
    AuthRequest, AuthResponse, BucketInfo, CreateFullPostResponse, CreateTagRequest, ErrorResponse,
};
use crate::model::{
    Content, CreateFullPost, Graph, ImportBatch, Media, Page, Post, PostDetail, PostGraphQuery,
    PostItem, PostSearchQuery, SearchPost, SearchPostItem, Tag,
};

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

    #[error("Json response error")]
    JsonResponseError(reqwest::Error),
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

        let info: BucketInfo = open_client
            .get(url.as_str())
            .send()
            .await
            .map_err(|e| HttpDataSourceError::FetchBucketError(e))?
            .json()
            .await
            .map_err(|e| HttpDataSourceError::FetchBucketError(e))?;

        let auth_response = Self::send_request::<AuthResponse>(
            open_client.post(format!("{url}/auth")).json(&AuthRequest {
                password: password.map(|s| s.to_string()),
            }),
        )
        .await
        .map_err(|e| HttpDataSourceError::LoginError(e))?
        .map_err(|e| match e.status {
            422 => HttpDataSourceError::PasswordRequired,
            401 => HttpDataSourceError::InvalidPassword,
            _ => HttpDataSourceError::ApiError(e),
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

    async fn send_request<T: DeserializeOwned>(
        req: RequestBuilder,
    ) -> Result<Result<T, ErrorResponse>, reqwest::Error> {
        let res = req.send().await?;

        if res.status().is_success() {
            Ok(Ok(res.json::<T>().await?))
        } else {
            Ok(Err(res.json::<ErrorResponse>().await?))
        }
    }
}

#[async_trait]
impl DataSource for HttpDataSource {
    fn blobs(&self) -> &dyn BlobDataSource {
        self
    }

    fn media(&self) -> &dyn MediaDataSource {
        self
    }

    fn content(&self) -> &dyn ContentDataSource {
        self
    }

    fn post_items(&self) -> &dyn PostItemDataSource {
        self
    }

    fn posts(&self) -> &dyn PostDataSource {
        self
    }

    fn import_batches(&self) -> &dyn ImportBatchDataSource {
        self
    }

    fn tags(&self) -> &dyn TagDataSource {
        self
    }

    fn tag_groups(&self) -> &dyn TagGroupDataSource {
        self
    }

    fn passwords(&self) -> &dyn PasswordDataSource {
        self
    }

    fn media_import(&self) -> &dyn MediaImportDataSource {
        self
    }

    fn cross(&self) -> &dyn CrossDataSource {
        self
    }
}

#[async_trait]
impl BlobDataSource for HttpDataSource {
    async fn add(&self, id: &Uuid) -> Result<Box<dyn FileInput>, DataSourceError> {
        todo!()
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Box<dyn FileOutput>, DataSourceError> {
        todo!()
    }

    async fn delete(&self, id: &Uuid) -> Result<(), DataSourceError> {
        todo!()
    }

    async fn has(&self, id: &Uuid) -> Result<bool, DataSourceError> {
        todo!()
    }
}

#[async_trait]
impl MediaDataSource for HttpDataSource {
    async fn add(&self, value: &mut Media) -> Result<(), DataSourceError> {
        todo!()
    }

    async fn remove(&self, value: &Media) -> Result<(), DataSourceError> {
        todo!()
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Media>, DataSourceError> {
        todo!()
    }

    async fn get_by_sha256(&self, sha256: &str) -> Result<Option<Media>, DataSourceError> {
        todo!()
    }
}

#[async_trait]
impl ContentDataSource for HttpDataSource {
    async fn add(&self, value: &mut Content) -> Result<(), DataSourceError> {
        todo!()
    }

    async fn get_by_content_id(&self, id: u64) -> Result<Option<Content>, DataSourceError> {
        todo!()
    }

    async fn update_thumbnail_id(
        &self,
        new_id: u64,
        content: &mut Content,
    ) -> Result<(), DataSourceError> {
        todo!()
    }
}

#[async_trait]
impl PostItemDataSource for HttpDataSource {
    async fn add(&self, value: &mut PostItem) -> Result<(), DataSourceError> {
        todo!()
    }

    async fn get_by_id(
        &self,
        post_item: u64,
        position: i32,
    ) -> Result<Option<PostItem>, DataSourceError> {
        todo!()
    }

    async fn get_page_from_post(
        &self,
        post_id: u64,
        page: PageParams,
    ) -> Result<Page<PostItem>, DataSourceError> {
        todo!()
    }
}

#[async_trait]
impl PostDataSource for HttpDataSource {
    async fn add(&self, value: &mut Post) -> Result<(), DataSourceError> {
        todo!()
    }

    async fn update(&self, value: &Post) -> Result<(), DataSourceError> {
        todo!()
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Post>, DataSourceError> {
        todo!()
    }

    async fn get_page(&self, page: PageParams) -> Result<Page<Post>, DataSourceError> {
        todo!()
    }
}

#[async_trait]
impl ImportBatchDataSource for HttpDataSource {
    async fn add(&self, value: &mut ImportBatch) -> Result<(), DataSourceError> {
        todo!()
    }
}

#[async_trait]
impl TagDataSource for HttpDataSource {
    async fn add(&self, value: &mut Tag) -> Result<(), DataSourceError> {
        let res = self
            .client
            .post(format!("{}/tags", self.base))
            .json(&CreateTagRequest {
                name: value.name.clone(),
                group: value.group.as_ref().map(|g| g.id()),
            })
            .send()
            .await?;

        let new_tag = res.json::<Tag>().await?;

        value.id = new_tag.id;

        Ok(())
    }

    async fn delete(&self, tag_id: u64) -> Result<(), DataSourceError> {
        todo!()
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Tag>, DataSourceError> {
        todo!()
    }

    async fn search(
        &self,
        page: &PageParams,
        query: &str,
        exact: bool,
    ) -> Result<Page<Tag>, DataSourceError> {
        let mut url = format!("{}/tags", self.base)
            .parse::<Url>()
            .expect("Cannot parse url");

        url.query_pairs_mut()
            .append_pair("offset", page.offset().to_string().as_str())
            .append_pair("size", page.page_size().to_string().as_str())
            .append_pair("exact", if exact { "true" } else { "false" })
            .append_pair("query", query);

        Ok(self
            .client
            .get(url)
            .send()
            .await?
            .json::<Page<Tag>>()
            .await?)
    }

    async fn get_all_from_post(&self, post_id: u64) -> Result<Vec<Tag>, DataSourceError> {
        todo!()
    }

    async fn add_tag_to_post(&self, tag_id: u64, post_id: u64) -> Result<(), DataSourceError> {
        todo!()
    }

    async fn remove_tag_to_post(&self, tag_id: u64, post_id: u64) -> Result<(), DataSourceError> {
        todo!()
    }
}

#[async_trait]
impl TagGroupDataSource for HttpDataSource {}

#[async_trait]
impl PasswordDataSource for HttpDataSource {
    async fn is_valid_password(&self, password: Option<&str>) -> Result<bool, DataSourceError> {
        todo!()
    }
}

#[async_trait]
impl MediaImportDataSource for HttpDataSource {
    async fn import_media(
        &self,
        mime: MediaTypeBuf,
        stream: &Path,
    ) -> Result<Content, MediaImportError> {
        let file = File::open(stream).await?;

        self.client
            .post(format!("{}/content", self.base))
            .header(CONTENT_TYPE, mime.as_str())
            .body(file)
            .send()
            .await
            .map_err(|e| MediaImportError::DataSourceError(e.into()))?
            .json::<Content>()
            .await
            .map_err(|e| MediaImportError::DataSourceError(e.into()))
    }
}

#[async_trait]
impl CrossDataSource for HttpDataSource {
    async fn get_post_detail(&self, post_id: u64) -> Result<Option<PostDetail>, DataSourceError> {
        todo!()
    }

    async fn search(
        &self,
        query: &PostSearchQuery,
        page: &PageParams,
    ) -> Result<Page<SearchPost>, DataSourceError> {
        let mut url = format!("{}/posts", self.base)
            .parse::<Url>()
            .expect("Cannot parse url");

        {
            let mut query_pairs = url.query_pairs_mut();

            query_pairs.append_pair("offset", page.offset().to_string().as_str());
            query_pairs.append_pair("size", page.page_size().to_string().as_str());

            if let Some(source) = query.source.as_deref() {
                query_pairs.append_pair("source", source);
            }

            if let Some(text) = query.text.as_deref() {
                query_pairs.append_pair("text", text);
            }
        }

        Ok(self
            .client
            .get(url)
            .send()
            .await?
            .json::<Page<SearchPost>>()
            .await?)
    }

    async fn search_items(
        &self,
        post_id: u64,
        page: PageParams,
    ) -> Result<Page<SearchPostItem>, DataSourceError> {
        todo!()
    }

    async fn get_full_post_item(
        &self,
        post_id: u64,
        position: i32,
    ) -> Result<Option<PostItem>, DataSourceError> {
        todo!()
    }

    async fn add_full_post(
        &self,
        new_post: CreateFullPost,
    ) -> Result<(ImportBatch, Vec<Post>), DataSourceError> {
        let res = self
            .client
            .post(format!("{}/posts", self.base))
            .json(&new_post)
            .send()
            .await?;

        let body = res.json::<CreateFullPostResponse>().await?;

        Ok((body.batch, body.posts))
    }

    async fn update_full_post(&self, value: &Post, tags: &[u64]) -> Result<(), DataSourceError> {
        todo!()
    }

    async fn cascade_delete_post(&self, id: u64) -> Result<(), DataSourceError> {
        todo!()
    }

    async fn graph_post(&self, query: &PostGraphQuery) -> Result<Graph, DataSourceError> {
        todo!()
    }
}
