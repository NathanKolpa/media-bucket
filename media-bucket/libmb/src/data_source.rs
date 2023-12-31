use std::path::Path;

use async_trait::async_trait;
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncSeek, AsyncWrite};
use uuid::Uuid;

use crate::model::*;

/// A trait representing a data source.
///
/// The `DataSource` trait represents a data source that provides access to various types of data.
/// It has a number of associated types, each of which provides access to a specific type of data (e.g. `BlobDataSource` for blobs, `MediaDataSource` for media, etc.).
pub trait DataSource: Send + Sync {
    fn blobs(&self) -> &dyn BlobDataSource;
    fn media(&self) -> &dyn MediaDataSource;
    fn content(&self) -> &dyn ContentDataSource;
    fn post_items(&self) -> &dyn PostItemDataSource;
    fn posts(&self) -> &dyn PostDataSource;
    fn import_batches(&self) -> &dyn ImportBatchDataSource;
    fn tags(&self) -> &dyn TagDataSource;
    fn tag_groups(&self) -> &dyn TagGroupDataSource;

    fn passwords(&self) -> &dyn PasswordDataSource;
    fn media_import(&self) -> &dyn MediaImportDataSource;
    fn cross(&self) -> &dyn CrossDataSource;
}

#[derive(Debug, Error)]
pub enum DataSourceError {
    #[error("duplicate model")]
    Duplicate,

    #[error("model not found")]
    NotFound,

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[cfg(feature = "local")]
    #[error("SQL error: {0}")]
    SQLError(#[from] sqlx::Error),

    #[cfg(feature = "http-client")]
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
}

/// A struct representing pagination parameters.
///
/// The `PageParams` struct is used to specify pagination parameters when retrieving data from a data source.
/// It includes the size of each page and the page number to retrieve.
#[derive(Debug)]
pub struct PageParams {
    page_size: usize,
    offset: usize,
}

impl PageParams {
    /// Create a new instance of a page.
    pub fn new(page_size: usize, offset: usize) -> Self {
        Self { page_size, offset }
    }

    /// Get the number of maximum items on each page.
    /// ## Example
    /// ```
    /// use libmb::data_source::PageParams;
    /// let page = PageParams::new(10, 5);
    ///
    /// assert_eq!(page.page_size(), 10);
    /// ```
    pub fn page_size(&self) -> usize {
        self.page_size
    }

    /// Get the index (starting from 0) of the requested page.
    /// ## Example
    /// ```
    /// use libmb::data_source::PageParams;
    /// let page = PageParams::new(10, 5);
    ///
    /// assert_eq!(page.offset(), 5);
    /// ```
    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn next(&self) -> Self {
        PageParams::new(self.page_size, self.offset + self.page_size)
    }
}

pub trait FileInput: AsyncWrite + Sync + Send {}

impl<T: AsyncWrite + Sync + Send> FileInput for T {}

pub trait FileOutput: AsyncRead + AsyncSeek + Sync + Send {}

impl<T: AsyncRead + AsyncSeek + Sync + Send> FileOutput for T {}

/// A data source for blobs aka files.
#[async_trait]
pub trait BlobDataSource: Send + Sync {
    /// Create a new blob
    /// and return a `Write` where writing to said `Write` would result in the
    /// blob's content getting written.
    ///
    /// ## Errors
    /// - `DataSourceError::Duplicate` => If the id already exists.
    async fn add(&self, id: &Uuid) -> Result<Box<dyn FileInput>, DataSourceError>;

    /// Get a seekable stream of the blob's content.
    ///
    /// ## Errors
    /// - `DataSourceError::NotFound` => If the id cannot be found.
    async fn get_by_id(&self, id: &Uuid) -> Result<Box<dyn FileOutput>, DataSourceError>;

    /// Delete a blob by id.
    ///
    /// ## Errors
    /// - `DataSourceError::NotFound` => If the id cannot be found.
    async fn delete(&self, id: &Uuid) -> Result<(), DataSourceError>;

    /// Check if a certain id exists. Returns `true` if the id exists and vice versa.
    async fn has(&self, id: &Uuid) -> Result<bool, DataSourceError>;
}

#[async_trait]
pub trait ImportBatchDataSource: Sync + Send {
    async fn add(&self, value: &mut ImportBatch) -> Result<(), DataSourceError>;
}

#[async_trait]
pub trait MediaDataSource: Sync + Send {
    async fn add(&self, value: &mut Media) -> Result<(), DataSourceError>;
    async fn remove(&self, value: &Media) -> Result<(), DataSourceError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<Media>, DataSourceError>;
    async fn get_by_sha256(&self, sha256: &str) -> Result<Option<Media>, DataSourceError>;

    async fn get_total_size(&self) -> Result<u64, DataSourceError>;
    async fn get_count(&self) -> Result<u64, DataSourceError>;
}

#[async_trait]
pub trait ContentDataSource: Sync + Send {
    async fn add(&self, value: &mut Content) -> Result<(), DataSourceError>;
    async fn get_by_content_id(&self, id: u64) -> Result<Option<Content>, DataSourceError>;
    async fn update_thumbnail_id(
        &self,
        new_id: u64,
        content: &mut Content,
    ) -> Result<(), DataSourceError>;
}

#[async_trait]
pub trait PostItemDataSource: Sync + Send {
    async fn add(&self, value: &mut PostItem) -> Result<(), DataSourceError>;
    async fn get_by_id(
        &self,
        post_item: u64,
        position: i32,
    ) -> Result<Option<PostItem>, DataSourceError>;
    async fn get_page_from_post(
        &self,
        post_id: u64,
        page: PageParams,
    ) -> Result<Page<PostItem>, DataSourceError>;
}

#[async_trait]
pub trait PostDataSource: Sync + Send {
    async fn add(&self, value: &mut Post) -> Result<(), DataSourceError>;
    async fn update(&self, value: &Post) -> Result<(), DataSourceError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<Post>, DataSourceError>;
    async fn get_page(&self, page: PageParams) -> Result<Page<Post>, DataSourceError>;
}

#[async_trait]
pub trait TagDataSource: Sync + Send {
    async fn add(&self, value: &mut Tag) -> Result<(), DataSourceError>;
    async fn update(&self, value: &Tag) -> Result<(), DataSourceError>;
    async fn delete(&self, tag_id: u64) -> Result<(), DataSourceError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<Tag>, DataSourceError>;
    async fn add_tag_to_post(&self, tag_id: u64, post_id: u64) -> Result<(), DataSourceError>;
    async fn remove_tag_to_post(&self, tag_id: u64, post_id: u64) -> Result<(), DataSourceError>;
}

#[async_trait]
pub trait TagGroupDataSource: Sync + Send {
    async fn add(&self, value: &mut TagGroup) -> Result<(), DataSourceError>;
    async fn search(
        &self,
        page: &PageParams,
        query: &str,
        exact: bool,
    ) -> Result<Page<TagGroup>, DataSourceError>;
}

#[async_trait]
pub trait PasswordDataSource: Sync + Send {
    async fn is_valid_password(&self, password: Option<&str>) -> Result<bool, DataSourceError>;
}

#[async_trait]
pub trait FileDataSource: Sync + Send {
    async fn create(&self, id: &str) -> Result<Box<dyn FileInput>, DataSourceError>;
    async fn read(&self, id: &str) -> Result<Box<dyn FileOutput>, DataSourceError>;
    async fn delete(&self, id: &str) -> Result<(), DataSourceError>;
    async fn has(&self, id: &str) -> Result<bool, DataSourceError>;
}

#[derive(Error, Debug)]
pub enum MediaImportError {
    #[error("missing program")]
    MissingProgram { name: &'static str },

    #[error("unsupported mimetype")]
    UnsupportedMimeType,

    #[error("unexpected program output")]
    UnexpectedOutput,

    #[error("datasource error")]
    DataSourceError(#[from] DataSourceError),

    #[error("io error")]
    IOError(#[from] std::io::Error),
}

#[async_trait]
pub trait MediaImportDataSource: Sync + Send {
    async fn import_media(
        &self,
        mime: mediatype::MediaTypeBuf,
        stream: &Path,
    ) -> Result<Content, MediaImportError>;
}

#[async_trait]
pub trait CrossDataSource: Sync + Send {
    async fn get_post_detail(&self, post_id: u64) -> Result<Option<PostDetail>, DataSourceError>;
    async fn search_posts(
        &self,
        query: &PostSearchQuery,
        page: &PageParams,
    ) -> Result<Page<SearchPost>, DataSourceError>;
    async fn search_items(
        &self,
        post_id: u64,
        page: PageParams,
    ) -> Result<Page<SearchPostItem>, DataSourceError>;
    async fn get_full_post_item(
        &self,
        post_id: u64,
        position: i32,
    ) -> Result<Option<PostItem>, DataSourceError>;
    async fn add_full_post(
        &self,
        new_post: CreateFullPost,
    ) -> Result<(ImportBatch, Vec<Post>), DataSourceError>;

    async fn search_tags(
        &self,
        page: &PageParams,
        query: &str,
        exact: bool,
    ) -> Result<Page<SearchTag>, DataSourceError>;

    async fn update_full_post(&self, value: &Post, tags: &[u64]) -> Result<(), DataSourceError>;

    async fn cascade_delete_post(&self, id: u64) -> Result<(), DataSourceError>;

    async fn graph_post(&self, query: &PostGraphQuery) -> Result<Graph, DataSourceError>;

    async fn get_tags_from_post(&self, post_id: u64) -> Result<Vec<SearchTag>, DataSourceError>;

    async fn get_tag_detail(&self, tag_id: u64) -> Result<Option<TagDetail>, DataSourceError>;
}
