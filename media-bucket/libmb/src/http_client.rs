use async_trait::async_trait;

use mediatype::MediaTypeBuf;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
    Body,
};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use thiserror::Error;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use url::Url;
use uuid::Uuid;

use crate::data_source::*;
use crate::http_models::*;
use crate::model::*;

const USER_AGENT: &str = "libmb/1.0";

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

    #[error("{0}")]
    DataSourceError(#[from] DataSourceError),
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

        let auth_response = open_client
            .post(format!("{url}/auth"))
            .json(&AuthRequest {
                password: password.map(|s| s.to_string()),
            })
            .send()
            .await
            .map_err(|e| HttpDataSourceError::LoginError(e))?;

        if !auth_response.status().is_success() {
            return Err(match auth_response.status().as_u16() {
                422 => HttpDataSourceError::PasswordRequired,
                401 => HttpDataSourceError::InvalidPassword,
                _ => HttpDataSourceError::ApiError(
                    auth_response
                        .json()
                        .await
                        .map_err(|e| HttpDataSourceError::LoginError(e))?,
                ),
            });
        }

        let auth_response: AuthResponse = auth_response
            .json()
            .await
            .map_err(|e| HttpDataSourceError::LoginError(e))?;

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

    async fn send_request<T: DeserializeOwned>(req: RequestBuilder) -> Result<T, DataSourceError> {
        let res = req
            .send()
            .await
            .map_err(|err| DataSourceError::HttpError(err))?;

        if res.status().is_success() {
            let body_text = res.text().await?;
            let data = serde_json::from_str(&body_text)
                .map_err(|err| DataSourceError::HttpProtocolError(err, body_text))?;

            Ok(data)
        } else {
            Err(res.json::<ErrorResponse>().await?.into())
        }
    }

    async fn send_resource_request<T: DeserializeOwned>(
        req: RequestBuilder,
    ) -> Result<Option<T>, DataSourceError> {
        let res = Self::send_request::<T>(req).await;

        match res {
            Ok(v) => Ok(Some(v)),
            Err(e) => match e {
                DataSourceError::NotFound => Ok(None),
                _ => Err(e),
            },
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
        let res = HttpDataSource::send_resource_request(
            self.client.get(format!("{}/media/{}", self.base, id)),
        )
        .await?;

        Ok(res)
    }

    async fn get_by_sha256(&self, sha256: &str) -> Result<Option<Media>, DataSourceError> {
        todo!()
    }

    async fn get_total_size(&self) -> Result<u64, DataSourceError> {
        todo!()
    }

    async fn get_count(&self) -> Result<u64, DataSourceError> {
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
        page: &PageParams,
    ) -> Result<Page<PostItem>, DataSourceError> {
        let mut url = format!("{}/posts/{}/items", self.base, post_id)
            .parse::<Url>()
            .expect("Cannot parse url");

        url.query_pairs_mut()
            .append_pair("offset", page.offset().to_string().as_str())
            .append_pair("size", page.page_size().to_string().as_str());

        let res: Page<SearchPostItem> = Self::send_request(self.client.get(url)).await?;

        Ok(Page {
            page_size: res.page_size,
            total_row_count: res.total_row_count,
            page_number: res.page_number,
            data: res.data.into_iter().map(|x| x.item).collect(),
        })
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
        let new_tag: Tag =
            HttpDataSource::send_request(self.client.post(format!("{}/tags", self.base)).json(
                &CreateTagRequest {
                    name: value.name.clone(),
                    group: value.group.as_ref().map(|g| g.id()),
                },
            ))
            .await?;

        value.id = new_tag.id;

        Ok(())
    }

    async fn update(&self, value: &Tag) -> Result<(), DataSourceError> {
        let res: Tag = HttpDataSource::send_request(
            self.client
                .put(format!("{}/tags/{}", self.base, value.id))
                .json(&UpdateTagRequest {
                    name: value.name.clone(),
                    group: value.group.as_ref().map(|g| g.id()),
                }),
        )
        .await?;

        Ok(())
    }

    async fn delete(&self, tag_id: u64) -> Result<(), DataSourceError> {
        todo!()
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Tag>, DataSourceError> {
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
impl TagGroupDataSource for HttpDataSource {
    async fn add(&self, value: &mut TagGroup) -> Result<(), DataSourceError> {
        let new_tag: TagGroup = HttpDataSource::send_request(
            self.client
                .post(format!("{}/tag-groups", self.base))
                .json(&CreateTagGroupRequest {
                    name: value.name.clone(),
                    hex_color: value.hex_color.clone(),
                }),
        )
        .await?;

        value.id = new_tag.id;

        Ok(())
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<TagGroup>, DataSourceError> {
        let res = HttpDataSource::send_resource_request(
            self.client.get(format!("{}/tag-groups/{}", self.base, id)),
        )
        .await?;

        Ok(res)
    }

    async fn search(
        &self,
        page: &PageParams,
        query: &str,
        exact: bool,
    ) -> Result<Page<TagGroup>, DataSourceError> {
        let mut url = format!("{}/tag-groups", self.base)
            .parse::<Url>()
            .expect("Cannot parse url");

        url.query_pairs_mut()
            .append_pair("offset", page.offset().to_string().as_str())
            .append_pair("size", page.page_size().to_string().as_str())
            .append_pair("exact", if exact { "true" } else { "false" })
            .append_pair("query", query);

        HttpDataSource::send_request(self.client.get(url)).await
    }
}

#[async_trait]
impl PasswordDataSource for HttpDataSource {
    async fn validate_password(&self, password: Option<&str>) -> Result<Option<[u8; 32]>, DataSourceError> {
        todo!()
    }
}

#[async_trait]
impl MediaImportDataSource for HttpDataSource {
    async fn import_media(
        &self,
        mime: MediaTypeBuf,
        stream: ImportSource<'_>,
    ) -> Result<Content, MediaImportError> {
        let mut req = self
            .client
            .post(format!("{}/content", self.base))
            .header(CONTENT_TYPE, mime.as_str());

        match stream {
            ImportSource::Stream(stream) => {
                req = req.body(Body::wrap_stream(ReaderStream::new(Box::into_pin(stream))));
            }
            ImportSource::File(path) => {
                let file = File::open(path).await?;
                req = req.body(file);
            }
        }

        HttpDataSource::send_request(req)
            .await
            .map_err(MediaImportError::DataSourceError)
    }
}

#[async_trait]
impl CrossDataSource for HttpDataSource {
    async fn get_post_detail(&self, post_id: u64) -> Result<Option<PostDetail>, DataSourceError> {
        todo!()
    }

    async fn search_posts(
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

            if let Some(order) = query.order.as_ref() {
                let order_text = match order {
                    PostSearchQueryOrder::Newest => "newest",
                    PostSearchQueryOrder::Oldest => "oldest",
                    PostSearchQueryOrder::Relevant => "relevant",
                    PostSearchQueryOrder::Random(_) => todo!(),
                };

                query_pairs.append_pair("order", order_text);
            }
        }

        HttpDataSource::send_request(self.client.get(url)).await
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
        let body: CreateFullPostResponse = HttpDataSource::send_request(
            self.client
                .post(format!("{}/posts", self.base))
                .json(&new_post),
        )
        .await?;

        Ok((body.batch, body.posts))
    }

    async fn search_tags(
        &self,
        page: &PageParams,
        query: &str,
        exact: bool,
    ) -> Result<Page<SearchTag>, DataSourceError> {
        let mut url = format!("{}/tags", self.base)
            .parse::<Url>()
            .expect("Cannot parse url");

        url.query_pairs_mut()
            .append_pair("offset", page.offset().to_string().as_str())
            .append_pair("size", page.page_size().to_string().as_str())
            .append_pair("exact", if exact { "true" } else { "false" })
            .append_pair("query", query);

        HttpDataSource::send_request(self.client.get(url)).await
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

    async fn get_tags_from_post(&self, post_id: u64) -> Result<Vec<SearchTag>, DataSourceError> {
        let tags = HttpDataSource::send_request(
            self.client
                .get(format!("{}/posts/{}/tags", self.base, post_id)),
        )
        .await?;
        Ok(tags)
    }

    async fn get_tag_detail(&self, tag_id: u64) -> Result<Option<TagDetail>, DataSourceError> {
        todo!()
    }
}
