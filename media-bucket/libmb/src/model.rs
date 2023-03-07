//! A module for data models.
//!
//! This module contains structs and enums that represent the data models used in the application.
//! These models define the structure of the data and how it is used throughout the application.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
pub enum ManyToOne<I, T> {
    #[serde(rename = "id")]
    Id(I),

    #[serde(rename = "obj")]
    Obj(T),
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
pub struct Page<T> {
    pub page_size: usize,
    pub total_row_count: usize,
    pub page_number: usize,
    pub data: Vec<T>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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
pub struct UploadMetadata {
    pub original_filename: Option<String>,
    pub original_directory: Option<String>,
    pub original_modified_at: Option<DateTime<Utc>>,
    pub original_accessed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: u64,
    pub source: Option<Url>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub import_batch: ManyToOne<u64, ImportBatch>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostItem {
    pub post: ManyToOne<u64, Post>,
    pub position: i32,
    pub content: ManyToOne<u64, Content>,
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
pub struct TagGroup {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: u64,
    pub name: String,
    pub group: Option<ManyToOne<u64, TagGroup>>,
    pub created_at: DateTime<Utc>,
}

// Joins

#[derive(Debug, Serialize, Deserialize)]
pub struct PostDetail {
    pub post: Post,
    pub tags: Vec<Tag>,
    pub item_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPost {
    pub post: Post,
    pub contains_image: bool,
    pub contains_video: bool,
    pub contains_moving_image: bool,
    pub item_count: usize,
    pub duration: Option<i32>,
    pub thumbnail: Option<Media>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPostItem {
    pub item: PostItem,
    pub contains_image: bool,
    pub contains_video: bool,
    pub contains_moving_image: bool,
    pub duration: Option<i32>,
    pub thumbnail: Option<Media>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateFullPostItem {
    pub content_id: u64,
    pub metadata: UploadMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateFullPost {
    pub title: Option<String>,
    pub description: Option<String>,
    pub source: Option<Url>,
    pub created_at: Option<DateTime<Utc>>,
    pub items: Vec<CreateFullPostItem>,
    pub tag_ids: Vec<u64>,
    pub flatten: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostSearchQuery {
    pub tags: Option<Vec<u64>>,
}

impl PostSearchQuery {
    pub fn has_criteria(&self) -> bool {
        self.tags.is_some()
    }
}
