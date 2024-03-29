//! A module for data models.
//!
//! This module contains structs and enums that represent the data models used in the application.
//! These models define the structure of the data and how it is used throughout the application.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;
use uuid::Uuid;

pub use chrono;
pub use mediatype;
pub use url;
pub use uuid;

/// An enum representing a many-to-one relationship.
///
/// The `ManyToOne` enum is used to represent a many-to-one relationship between two types of objects.
/// It can be either an `Id` variant, containing an identifier for the related object, or an `Obj` variant, containing the related object itself.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub enum ManyToOne<I, T> {
    #[serde(rename = "id")]
    Id(I),

    #[serde(rename = "obj")]
    Obj(T),
}

impl<I, T> ManyToOne<I, T> {
    pub fn obj(self) -> Option<T> {
        match self {
            ManyToOne::Id(_) => None,
            ManyToOne::Obj(o) => Some(o),
        }
    }

    pub fn as_ref(&self) -> ManyToOne<I, &T>
    where
        I: Copy,
    {
        match self {
            ManyToOne::Id(id) => ManyToOne::Id(*id),
            ManyToOne::Obj(obj) => ManyToOne::Obj(obj),
        }
    }
}

impl ManyToOne<u64, Media> {
    pub fn id(&self) -> u64 {
        match self {
            Self::Id(id) => *id,
            Self::Obj(obj) => obj.id,
        }
    }
}

impl ManyToOne<u64, TagGroup> {
    pub fn id(&self) -> u64 {
        match self {
            Self::Id(id) => *id,
            Self::Obj(obj) => obj.id,
        }
    }
}

/// A struct representing a paginated collection of data.
///
/// The `Page` struct is used to represent a collection of data that has been divided into pages.
/// It includes information about the size of each page, the total number of rows in the collection, and the current page number. It also includes a vector of data for the current page.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct Page<T> {
    pub page_size: usize,
    pub total_row_count: usize,
    pub page_number: usize,
    pub data: Vec<T>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct ImportBatch {
    pub id: u64,
}

impl ManyToOne<u64, ImportBatch> {
    pub fn id(&self) -> u64 {
        match self {
            Self::Id(id) => *id,
            Self::Obj(obj) => obj.id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub enum MediaMetadata {
    Image {
        dims: Dimensions,
    },
    Video {
        dims: Dimensions,
        duration: i32,
        video_encoding: String,
    },
    Document {
        pages: i32,
        title: Option<String>,
        author: Option<String>,
        page_size: Dimensions,
    },
    Unknown,
}

impl MediaMetadata {
    pub fn video_encoding(&self) -> Option<&str> {
        match self {
            MediaMetadata::Video { video_encoding, .. } => Some(video_encoding.as_str()),
            _ => None,
        }
    }

    pub fn width(&self) -> Option<&i32> {
        match self {
            MediaMetadata::Image { dims } | MediaMetadata::Video { dims, .. } => Some(&dims.width),
            _ => None,
        }
    }

    pub fn height(&self) -> Option<&i32> {
        match self {
            MediaMetadata::Image { dims } | MediaMetadata::Video { dims, .. } => Some(&dims.height),
            _ => None,
        }
    }

    pub fn duration(&self) -> Option<&i32> {
        match self {
            MediaMetadata::Video { duration, .. } => Some(duration),
            _ => None,
        }
    }

    pub fn pages(&self) -> Option<&i32> {
        match self {
            MediaMetadata::Document { pages, .. } => Some(pages),
            _ => None,
        }
    }

    pub fn title(&self) -> Option<&str> {
        match self {
            MediaMetadata::Document { title, .. } => title.as_deref(),
            _ => None,
        }
    }

    pub fn author(&self) -> Option<&str> {
        match self {
            MediaMetadata::Document { author, .. } => author.as_deref(),
            _ => None,
        }
    }

    pub fn page_size(&self) -> Option<&Dimensions> {
        match self {
            MediaMetadata::Document { page_size, .. } => Some(page_size),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct Media {
    pub id: u64,
    pub file_id: Uuid,
    pub file_size: usize,
    pub sha1: String,
    pub sha256: String,
    pub md5: String,
    pub metadata: MediaMetadata,

    pub mime: ::mediatype::MediaTypeBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct Content {
    pub content: ManyToOne<u64, Media>,
    pub thumbnail: ManyToOne<u64, Media>,
}

impl ManyToOne<u64, Content> {
    pub fn id(&self) -> u64 {
        match self {
            Self::Id(id) => *id,
            Self::Obj(obj) => obj.content.id(),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct UploadMetadata {
    pub original_filename: Option<String>,
    pub original_directory: Option<String>,
    pub original_modified_at: Option<DateTime<Utc>>,
    pub original_accessed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct Post {
    pub id: u64,
    pub source: Option<Url>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub import_batch: ManyToOne<u64, ImportBatch>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct PostItem {
    pub post: ManyToOne<u64, Post>,
    pub position: i32,
    pub content: ManyToOne<u64, Content>,
    pub upload: UploadMetadata,
}

impl ManyToOne<u64, Post> {
    pub fn id(&self) -> u64 {
        match self {
            Self::Id(id) => *id,
            Self::Obj(obj) => obj.id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct TagGroup {
    pub id: u64,
    pub name: String,
    pub hex_color: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct Tag {
    pub id: u64,
    pub name: String,
    pub group: Option<ManyToOne<u64, TagGroup>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct SearchTag {
    pub tag: Tag,
    pub linked_posts: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct TagDetail {
    pub tag: Tag,
}

// Joins

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct PostDetail {
    pub post: Post,
    pub tags: Vec<SearchTag>,
    pub item_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct SearchPost {
    pub post: Post,
    pub contains_image: bool,
    pub contains_video: bool,
    pub contains_moving_image: bool,
    pub contains_document: bool,
    pub item_count: usize,
    pub duration: Option<i32>,
    pub thumbnail: Option<Media>,
    pub file_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct SearchPostItem {
    pub item: PostItem,
    pub contains_image: bool,
    pub contains_video: bool,
    pub contains_moving_image: bool,
    pub contains_document: bool,
    pub duration: Option<i32>,
    pub thumbnail: Option<Media>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct CreateFullPostItem {
    pub content_id: u64,
    pub metadata: UploadMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct CreateFullPost {
    pub title: Option<String>,
    pub description: Option<String>,
    pub source: Option<Url>,
    pub created_at: Option<DateTime<Utc>>,
    pub items: Vec<CreateFullPostItem>,
    pub tag_ids: Vec<u64>,
    pub flatten: bool,
    pub batch_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub enum PostSearchQueryOrder {
    Newest,
    Oldest,
    Relevant,
    Random(f32),
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct PostSearchQuery {
    pub tags: Option<Vec<u64>>,
    pub text: Option<String>,
    pub source: Option<String>,
    pub order: Option<PostSearchQueryOrder>,
    pub require_playable: bool,
}

impl PostSearchQuery {
    pub fn has_criteria(&self) -> bool {
        self.tags.is_some() || self.text.is_some() || self.source.is_some()
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct PostItemSearchQuery {
    pub require_playable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub enum GraphValue {
    Date(DateTime<Utc>),
    None,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct GraphPoint {
    pub x: GraphValue,
    pub y: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct Graph {
    pub points: Vec<GraphPoint>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub enum GraphDiscriminator {
    Duration(Duration),
    None,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub enum GraphSelect {
    Count,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct PostGraphQuery {
    pub filter: PostSearchQuery,
    pub discriminator: GraphDiscriminator,
    pub select: GraphSelect,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::ToSchema))]
pub struct BucketDetails {
    pub sessions_created: u64,
    pub total_file_size: u64,
    pub file_count: u64,
}
