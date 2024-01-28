#[cfg(feature = "local")]
use std::path::Path;

use futures::future::join_all;
use thiserror::Error;
use tokio::join;
use url::Url;

use crate::{
    data_source::{DataSource, DataSourceError, ImportSource, MediaImportError, PageParams},
    model::{
        CreateFullPost, CreateFullPostItem, ManyToOne, Post, PostItem, PostSearchQuery, Tag,
        TagGroup,
    },
};
use crate::model::ImportBatch;

#[derive(Clone, Copy, Debug)]
pub enum SyncMatchStategy {
    Url,
    None,
}

#[derive(Error, Debug)]
pub enum BucketError {
    #[error("Password required")]
    PasswordRequired,

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Missing feature {0}")]
    MissingFeature(&'static str),

    #[error("Invalid location")]
    InvalidLocation,

    #[cfg(feature = "local")]
    #[error("{0}")]
    LocalDataSourceError(crate::local::LocalDataSourceError),

    #[cfg(feature = "http-client")]
    #[error("{0}")]
    HttpDataSourceError(crate::http_client::HttpDataSourceError),

    #[error("{0}")]
    DataSourceError(#[from] DataSourceError),

    #[error("{0}")]
    ImportError(#[from] MediaImportError),
}

#[cfg(feature = "local")]
impl From<crate::local::LocalDataSourceError> for BucketError {
    fn from(value: crate::local::LocalDataSourceError) -> Self {
        match value {
            crate::local::LocalDataSourceError::InvalidPassword => Self::InvalidPassword,
            e => Self::LocalDataSourceError(e),
        }
    }
}

#[cfg(feature = "http-client")]
impl From<crate::http_client::HttpDataSourceError> for BucketError {
    fn from(value: crate::http_client::HttpDataSourceError) -> Self {
        match value {
            crate::http_client::HttpDataSourceError::InvalidPassword => Self::InvalidPassword,
            e => Self::HttpDataSourceError(e),
        }
    }
}

/// The `Bucket` struct provides a high-level interface for interacting with a bucket of data.
/// It abstracts away the details of how the data is stored and retrieved, allowing users to focus on the data itself.
///
/// Some examples of actions that can be performed on a `Bucket` include adding data, retrieving data, and deleting data.
/// The `Bucket` struct also has a flag indicating whether the data is encrypted.
pub struct Bucket {
    is_encrypted: bool,
    data_source: Box<dyn DataSource>,
}

impl Bucket {
    /// Opens a bucket at the given location.
    ///
    /// This function attempts to open a bucket at the specified location. If the bucket is encrypted, a password must be provided.
    ///
    /// # Arguments
    ///
    /// * `location` - The location of the bucket to open.
    /// * `password` - An optional password to use when opening an encrypted bucket.
    /// # Returns
    ///
    /// If the bucket is successfully opened, this function returns a `Bucket` instance.
    /// If an error occurs, it returns a `BucketError`. Possible errors include:
    ///
    /// * `BucketError::InvalidLocation` - The location does not point to a valid bucket.
    /// * `BucketError::MissingFeature` - The location is valid, but it requires a feature that is not active.
    /// * `BucketError::PasswordRequired` - A password is required to open the bucket, but none was provided.
    pub async fn open(location: &str, password: Option<&str>) -> Result<Self, BucketError> {
        if location.starts_with("http://") || location.starts_with("https://") {
            return Ok(Self::open_http_client(location.parse().unwrap(), password).await?);
        }

        #[cfg(feature = "local")]
        return match password {
            None => Err(BucketError::PasswordRequired),
            Some(password) => Self::open_encrypted(Path::new(location), password).await,
        };

        #[cfg(not(feature = "local"))]
        return Err(BucketError::MissingFeature("local"));
    }

    pub async fn password_protected(location: &str) -> std::io::Result<bool> {
        Ok(true)
    }

    #[cfg(feature = "local")]
    pub async fn dir_errors(path: &Path) -> Option<BucketError> {
        todo!()
    }

    #[cfg(feature = "local")]
    pub async fn is_dir_encrypted_bucket(path: &Path) -> bool {
        todo!()
    }

    #[cfg(feature = "http-client")]
    pub async fn open_http_client(base: Url, password: Option<&str>) -> Result<Self, BucketError> {
        use crate::http_client::HttpDataSource;

        let data_source = HttpDataSource::open(base, password).await?;

        Ok(Self {
            is_encrypted: data_source.is_encrypted(),
            data_source: Box::new(data_source),
        })
    }

    /// This method opens an encrypted bucket at the specified `path`, using the given `password`.
    #[cfg(feature = "encryption")]
    pub async fn open_encrypted(path: &Path, password: &str) -> Result<Self, BucketError> {
        use crate::local::LocalDataSource;

        Ok(Self {
            is_encrypted: true,
            data_source: Box::new(LocalDataSource::open_encrypted(path, password).await?),
        })
    }

    #[cfg(feature = "encryption")]
    pub async fn create_encrypted(path: &Path, password: &str) -> Result<Self, BucketError> {
        use crate::local::LocalDataSource;

        Ok(Self {
            is_encrypted: true,
            data_source: Box::new(LocalDataSource::create_encrypted(path, password).await?),
        })
    }

    pub fn data_source(&self) -> &dyn DataSource {
        &*self.data_source
    }

    /// Returns true if the `data_source` is stored using encryption.
    pub fn is_encrypted(&self) -> bool {
        self.is_encrypted
    }

    pub async fn sync_from(
        &self,
        source: &Self,
        strat: SyncMatchStategy,
        delete_synced: bool,
        on_sync: &impl Fn(&Post),
    ) -> Result<(), BucketError> {
        let mut batch = ImportBatch {
            id: 0
        };

        self.data_source.import_batches().add(&mut batch).await?;

        let query = PostSearchQuery::default();
        let mut page = PageParams::new(1, 0);

        let mut posts_to_delete = Vec::<u64>::new();

        loop {
            let results = source
                .data_source()
                .cross()
                .search_posts(&query, &page)
                .await?;

            if results.data.is_empty() {
                break;
            }

            if delete_synced {
                posts_to_delete.extend(results.data.iter().map(|s| s.post.id));
            }

            let futures = results
                .data
                .into_iter()
                .map(|search_post| self.sync_post(source, search_post.post, strat, on_sync, &batch));

            join_all(futures)
                .await
                .into_iter()
                .collect::<Result<(), BucketError>>()?;

            page = page.next();
        }

        for post_id in posts_to_delete {
            source
                .data_source()
                .cross()
                .cascade_delete_post(post_id)
                .await?;
        }

        Ok(())
    }

    async fn sync_post(
        &self,
        source: &Self,
        post: Post,
        strat: SyncMatchStategy,
        on_sync: &impl Fn(&Post),
        batch: &ImportBatch
    ) -> Result<(), BucketError> {
        let matched_post = match strat {
            SyncMatchStategy::None => None,
            SyncMatchStategy::Url => {
                let Some(source_url) = post.source.as_ref() else {
                    return Ok(());
                };

                let query = PostSearchQuery {
                    source: Some(source_url.to_string()),
                    ..Default::default()
                };

                let page = PageParams::new(64, 0);

                let search_result = self
                    .data_source()
                    .cross()
                    .search_posts(&query, &page)
                    .await?;

                search_result.data.into_iter().last()
            }
        };

        // the post is already synced, no need to do anything.
        if matched_post.is_some() {
            return Ok(());
        }

        let (tag_ids, items) = join!(
            self.sync_post_tags(source, &post),
            self.sync_post_items(source, &post)
        );

        let (tag_ids, items) = (tag_ids?, items?);

        let (_, new_post) = self
            .data_source()
            .cross()
            .add_full_post(CreateFullPost {
                title: post.title,
                description: post.description,
                source: post.source,
                created_at: Some(post.created_at),
                items,
                tag_ids,
                flatten: false,
                batch_id: Some(batch.id)
            })
            .await?;

        if let Some(new_post) = new_post.first() {
            on_sync(new_post)
        }

        Ok(())
    }

    async fn sync_post_tags(&self, source: &Self, post: &Post) -> Result<Vec<u64>, BucketError> {
        let tags = source
            .data_source()
            .cross()
            .get_tags_from_post(post.id)
            .await?;

        let futures = tags
            .into_iter()
            .map(|search_tag| self.sync_tag(source, search_tag.tag));
        join_all(futures).await.into_iter().collect()
    }

    async fn sync_tag(&self, source: &Self, tag: Tag) -> Result<u64, BucketError> {
        let page = PageParams::new(1, 0);
        let search_result = self
            .data_source()
            .cross()
            .search_tags(&page, &tag.name, true)
            .await?;

        let group = search_result
            .data
            .first()
            .and_then(|t| t.tag.group.as_ref());

        let group_result = match group {
            None => None,
            Some(ManyToOne::Obj(v)) => Some(v.clone()),
            Some(ManyToOne::Id(id)) => source.data_source().tag_groups().get_by_id(*id).await?,
        };

        let synced_group = match group_result {
            None => None,
            Some(group) => {
                let existing_group = self
                    .data_source()
                    .tag_groups()
                    .search(&page, &group.name, true)
                    .await?;

                if existing_group.data.is_empty() {
                    let mut new_group = TagGroup {
                        id: 0,
                        name: group.name,
                        hex_color: group.hex_color,
                        created_at: group.created_at,
                    };

                    self.data_source().tag_groups().add(&mut new_group).await?;

                    Some(new_group)
                } else {
                    existing_group.data.into_iter().last()
                }
            }
        };

        let mut synced_tag = match search_result.data.into_iter().last() {
            Some(existing_tag) => existing_tag.tag,
            None => {
                let mut new_tag = Tag {
                    id: 0,
                    name: tag.name,
                    group: synced_group.as_ref().map(|g| ManyToOne::Id(g.id)),
                    created_at: tag.created_at,
                };
                self.data_source().tags().add(&mut new_tag).await?;

                new_tag
            }
        };

        if synced_tag.group.is_none() && synced_group.is_some() {
            synced_tag.group = synced_group.as_ref().map(|g| ManyToOne::Id(g.id));
            self.data_source().tags().update(&synced_tag).await?;
        }

        Ok(synced_tag.id)
    }

    async fn sync_post_items(
        &self,
        source: &Self,
        post: &Post,
    ) -> Result<Vec<CreateFullPostItem>, BucketError> {
        let mut page = PageParams::new(64, 0);
        let mut new_items = Vec::new();

        loop {
            let search_results = source
                .data_source()
                .post_items()
                .get_page_from_post(post.id, &page)
                .await?;

            if search_results.data.is_empty() {
                break;
            }

            let items = join_all(
                search_results
                    .data
                    .into_iter()
                    .map(|item| self.sync_content(source, item)),
            )
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

            new_items.extend(items);

            page = page.next();
        }

        Ok(new_items)
    }

    async fn sync_content(
        &self,
        source: &Self,
        item: PostItem,
    ) -> Result<CreateFullPostItem, BucketError> {
        let media_id = item.content.id();

        let media = source
            .data_source()
            .media()
            .get_by_id(media_id)
            .await?
            .ok_or(BucketError::DataSourceError(DataSourceError::NotFound))?;

        let stream = source
            .data_source()
            .blobs()
            .get_by_id(&media.file_id)
            .await?;

        let content_upload = self
            .data_source()
            .media_import()
            .import_media(media.mime, ImportSource::Stream(stream))
            .await?;

        Ok(CreateFullPostItem {
            content_id: content_upload.content.id(),
            metadata: item.upload,
        })
    }
}
