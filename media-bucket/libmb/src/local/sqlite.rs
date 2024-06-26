use std::ops::DerefMut;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mediatype::MediaTypeBuf;
use sqlx::sqlite::{
    SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteRow, SqliteSynchronous,
};
use sqlx::{ConnectOptions, Executor, Row, Sqlite, SqlitePool};
use thiserror::Error;
use uuid::Uuid;

use crate::data_source::*;
use crate::model::*;

#[derive(Error, Debug)]
pub enum SqliteError {
    #[error("Invalid path")]
    InvalidPath,

    #[error("SQL error {0}")]
    SQLError(#[from] sqlx::Error),

    #[error("Migration error {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    #[error("Cannot find database file")]
    CannotFindDatabaseFile,
}

/// This struct represents SQLite database data.
///
/// It implements the `DataSource` trait, which defines the methods for interacting with the various data sources.
/// The `Sqlite` struct maps the models in the application to the corresponding SQL tables, and vice versa.
///
/// The `Sqlite` struct also manages a pool of database connections.
/// This allows the application to efficiently reuse connections and improve performance.
pub struct SqliteIndex {
    read_pool: SqlitePool,
    write_pool: SqlitePool,
}

impl SqliteIndex {
    #[cfg(feature = "encryption")]
    pub async fn open_encrypted(
        path: &Path,
        secret: crate::local::secret::Secret,
    ) -> Result<Self, SqliteError> {
        if !path.is_file() {
            return Err(SqliteError::CannotFindDatabaseFile);
        }

        let (write_pool, read_pool) = Self::new_pools(path, secret, false).await?;

        Self::prepare_db(&write_pool).await?;
        Self::migrate(&write_pool).await?;

        Ok(Self {
            read_pool,
            write_pool,
        })
    }

    #[cfg(feature = "encryption")]
    pub async fn create_encrypted(
        path: &Path,
        secret: crate::local::secret::Secret,
    ) -> Result<Self, SqliteError> {
        let (write_pool, read_pool) = Self::new_pools(path, secret, true).await?;

        Self::migrate(&write_pool).await?;

        Ok(Self {
            read_pool,
            write_pool,
        })
    }

    #[cfg(feature = "encryption")]
    async fn new_pools(
        path: &Path,
        secret: crate::local::secret::Secret,
        create: bool,
    ) -> Result<(SqlitePool, SqlitePool), SqliteError> {
        let write_pool = Self::new_encrypted_pool(path, secret.clone(), create, false, 1).await?;
        let read_pool = Self::new_encrypted_pool(path, secret, false, true, 64).await?;

        Ok((write_pool, read_pool))
    }

    #[cfg(feature = "encryption")]
    async fn new_encrypted_pool(
        path: &Path,
        secret: crate::local::secret::Secret,
        create: bool,
        readonly: bool,
        max_conns: u32,
    ) -> Result<SqlitePool, SqliteError> {
        let key = format!("'{}'", hex::encode(secret.bytes()));

        let connect_options =
            SqliteConnectOptions::from_str(path.to_str().ok_or(SqliteError::InvalidPath)?)?
                .disable_statement_logging()
                .create_if_missing(create)
                .pragma("key", key)
                .journal_mode(SqliteJournalMode::Wal)
                .synchronous(SqliteSynchronous::Normal)
                .read_only(readonly)
                .busy_timeout(Duration::from_secs(10));

        Ok(SqlitePoolOptions::new()
            .max_connections(max_conns)
            .acquire_timeout(Duration::from_secs(60 * 60 * 6))
            .connect_with(connect_options)
            .await?)
    }

    async fn prepare_db(pool: &SqlitePool) -> Result<(), SqliteError> {
        let mut conn = pool.acquire().await?;
        conn.execute("PRAGMA wal_checkpoint(TRUNCATE)").await?;
        Ok(())
    }

    async fn migrate(pool: &SqlitePool) -> Result<(), SqliteError> {
        let mut conn = pool.acquire().await?;

        conn.execute("PRAGMA foreign_keys = off").await?;

        sqlx::migrate!("db/migrations")
            .run(conn.deref_mut())
            .await?;

        conn.execute("PRAGMA foreign_keys = on").await?;

        Ok(())
    }

    fn map_media(row: &SqliteRow) -> Result<Media, DataSourceError> {
        let mime_type: String = row.try_get("mime_type")?;
        let mime_sub_type: String = row.try_get("mime_sub_type")?;
        let file_id: &[u8] = row.try_get("file_id")?;

        Ok(Media {
            id: row.try_get::<'_, i64, _>("media_id")? as u64,
            metadata: match (mime_type.as_str(), mime_sub_type.as_str()) {
                ("video", _) => MediaMetadata::Video {
                    dims: Dimensions {
                        width: row.try_get("width")?,
                        height: row.try_get("height")?,
                    },
                    duration: row.try_get("duration")?,
                    video_encoding: row.try_get("video_encoding")?,
                },
                ("image", _) => MediaMetadata::Image {
                    dims: Dimensions {
                        width: row.try_get("width")?,
                        height: row.try_get("height")?,
                    },
                },
                (_, "pdf") => MediaMetadata::Document {
                    pages: row.try_get("document_pages")?,
                    title: row.try_get("document_title")?,
                    author: row.try_get("document_author")?,
                    page_size: Dimensions {
                        width: row.try_get("page_width")?,
                        height: row.try_get("page_height")?,
                    },
                },
                _ => MediaMetadata::Unknown,
            },
            file_id: Uuid::from_bytes(file_id.try_into().unwrap()),
            file_size: row.try_get::<'_, i64, _>("file_size")? as usize,
            sha1: row.try_get("sha1")?,
            sha256: row.try_get("sha256")?,
            md5: row.try_get("md5")?,
            mime: format!("{mime_type}/{mime_sub_type}").parse().unwrap(),
        })
    }

    fn map_post_item(row: &SqliteRow) -> Result<PostItem, DataSourceError> {
        Ok(PostItem {
            post: ManyToOne::Id(row.try_get::<'_, i64, _>("post_id")? as u64),
            position: row.try_get("item_order")?,
            content: ManyToOne::Id(row.try_get::<'_, i64, _>("content_id")? as u64),
            upload: UploadMetadata {
                original_filename: row.try_get("original_name")?,
                original_directory: row.try_get("original_directory")?,
                original_modified_at: row.try_get("original_modified")?,
                original_accessed_at: row.try_get("original_accessed")?,
            },
        })
    }

    fn map_post(row: &SqliteRow) -> Result<Post, DataSourceError> {
        let source: Option<String> = row.try_get("source")?;

        Ok(Post {
            id: row.try_get::<'_, i64, _>("post_id")? as u64,
            source: source.map(|s| s.parse().unwrap()),
            title: row.try_get("title")?,
            description: row.try_get("description")?,
            import_batch: ManyToOne::Id(row.try_get::<'_, i64, _>("import_batch_id")? as u64),
            created_at: row.try_get("created_at")?,
        })
    }

    fn map_content(row: &SqliteRow) -> Result<Content, DataSourceError> {
        Ok(Content {
            content: ManyToOne::Id(row.try_get::<'_, i64, _>("content_id")? as u64),
            thumbnail: ManyToOne::Id(row.try_get::<'_, i64, _>("thumbnail_id")? as u64),
        })
    }

    fn map_tag(row: &SqliteRow) -> Result<Tag, DataSourceError> {
        let group: Option<i64> = row.try_get("group_id")?;

        Ok(Tag {
            id: row.try_get::<'_, i64, _>("tag_id")? as u64,
            name: row.try_get("name")?,
            group: group.map(|g| ManyToOne::Id(g as u64)),
            created_at: row.try_get("created_at")?,
        })
    }

    fn map_search_tag(row: &SqliteRow) -> Result<SearchTag, DataSourceError> {
        Ok(SearchTag {
            tag: Self::map_full_tag(row)?,
            linked_posts: row.try_get::<'_, i64, _>("linked_posts")? as u64,
        })
    }

    fn map_full_tag(row: &SqliteRow) -> Result<Tag, DataSourceError> {
        let group: Option<i64> = row.try_get("group_id").ok();
        let group_color: Option<String> = row.try_get("color").ok();
        let group_name: Option<String> = row.try_get("g_name").ok();
        let group_created_at: Option<DateTime<Utc>> = row.try_get("g_created_at").ok();

        Ok(Tag {
            id: row.try_get::<'_, i64, _>("tag_id")? as u64,
            name: row.try_get("name")?,
            group: match (group, group_color, group_name, group_created_at) {
                (Some(group_id), Some(group_color), Some(group_name), Some(group_created_at)) => {
                    Some(ManyToOne::Obj(TagGroup {
                        id: group_id as u64,
                        name: group_name,
                        hex_color: group_color,
                        created_at: group_created_at,
                    }))
                }
                _ => None,
            },
            created_at: row.try_get("created_at")?,
        })
    }

    fn map_tag_group(row: &SqliteRow) -> Result<TagGroup, DataSourceError> {
        Ok(TagGroup {
            id: row.try_get::<'_, i64, _>("group_id")? as u64,
            name: row.try_get("name")?,
            hex_color: row.try_get("color")?,
            created_at: row.try_get("created_at")?,
        })
    }

    fn map_search_post(row: &SqliteRow) -> Result<SearchPost, DataSourceError> {
        let thumbnail = (row.try_get::<'_, Option<i64>, _>("media_id")?)
            .and_then(|_| Self::map_media(row).ok());

        Ok(SearchPost {
            post: Self::map_post(row)?,
            contains_image: row.try_get("contains_image")?,
            contains_video: row.try_get("contains_video")?,
            contains_document: row.try_get("contains_document")?,
            contains_moving_image: row.try_get("contains_moving_image")?,
            item_count: row.try_get::<'_, i64, _>("item_count")? as usize,
            duration: row.try_get("total_duration")?,
            thumbnail,
            file_name: row.try_get("original_name")?,
        })
    }

    fn map_graph_point(row: &SqliteRow) -> Result<GraphPoint, DataSourceError> {
        let kind: &str = row.try_get(2)?;

        Ok(GraphPoint {
            y: row.try_get(0)?,
            x: match kind {
                "date" => GraphValue::Date(row.try_get(1)?),
                "none" => GraphValue::None,
                _ => {
                    return Err(DataSourceError::SQLError(sqlx::Error::ColumnDecode {
                        source: "map_graph_point".into(),
                        index: String::from("2"),
                    }));
                }
            },
        })
    }

    fn map_full_post_item(row: &SqliteRow) -> Result<PostItem, DataSourceError> {
        Ok(PostItem {
            post: ManyToOne::Obj(Self::map_post(row)?),
            position: row.try_get("item_order")?,
            content: ManyToOne::Obj(Content {
                content: ManyToOne::Obj(Self::map_media(row)?),
                thumbnail: ManyToOne::Id(row.try_get::<'_, i64, _>("thumbnail_id")? as u64),
            }),
            upload: UploadMetadata {
                original_filename: row.try_get("original_name")?,
                original_directory: row.try_get("original_directory")?,
                original_modified_at: row.try_get("original_modified")?,
                original_accessed_at: row.try_get("original_accessed")?,
            },
        })
    }

    fn map_search_post_item(row: &SqliteRow) -> Result<SearchPostItem, DataSourceError> {
        let mime: MediaTypeBuf = row
            .try_get::<'_, String, _>("content_mime_type")?
            .parse()
            .unwrap();

        Ok(SearchPostItem {
            item: Self::map_post_item(row)?,
            contains_image: mime.ty() == mediatype::names::IMAGE
                && mime.subty() != mediatype::names::GIF,
            contains_video: mime.ty() == mediatype::names::VIDEO,
            contains_moving_image: mime.subty() == mediatype::names::GIF,
            contains_document: mime.ty() != mediatype::names::APPLICATION
                && mime.subty() == mediatype::names::PDF,
            duration: row.try_get("content_duration")?,
            thumbnail: (row.try_get::<'_, Option<i64>, _>("media_id")?)
                .and_then(|_| Self::map_media(row).ok()),
        })
    }

    fn create_search_query_str<'a>(
        query: &PostSearchQuery,
        before_where: &'a str,
        after_where: &'a str,
    ) -> String {
        let mut where_clause = String::new();

        if query.has_criteria() {
            let mut is_first = true;

            where_clause.push_str("WHERE");

            if let Some(text) = query.text.as_deref() {
                let query_is_empty = text.len() < 3;

                if query_is_empty {
                    where_clause.push_str(" (p.source LIKE ? OR p.title LIKE ? OR p.description LIKE ? OR p.tags LIKE ? OR p.original_name LIKE ? OR p.original_directory LIKE ? OR p.document_title LIKE ? OR p.document_author LIKE ?)");
                } else {
                    where_clause.push_str(" posts_vtab MATCH (?)")
                }

                is_first = false;
            }

            if let Some(tag_ids) = query.tags.as_ref() {
                for tag_id in tag_ids.iter() {
                    if !is_first {
                        where_clause.push_str(" AND")
                    }

                    where_clause.push_str(
                        " EXISTS(SELECT tp.tag_id FROM tags_posts tp WHERE tp.post_id = p.post_id AND tp.tag_id = ?)",
                    );

                    is_first = false;
                }
            }

            if query.require_playable {
                if !is_first {
                    where_clause.push_str(" AND")
                }

                where_clause.push_str(" total_duration > 0");

                is_first = false;
            }

            if let Some(source) = query.source.as_ref() {
                if !is_first {
                    where_clause.push_str(" AND")
                }

                where_clause.push_str(" p.source = ?");
            }
        }

        format!("{before_where}\n{where_clause}\n{after_where}")
    }

    fn add_search_query_values<'a>(
        query_values: &'a PostSearchQuery,
        str: &'a str,
    ) -> sqlx::query::Query<'a, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'a>> {
        let mut query = sqlx::query(str);

        if let Some(text) = query_values.text.as_deref() {
            let query_is_empty = text.len() < 3;

            if !query_is_empty {
                let escaped = text
                    .trim()
                    .replace(",", "")
                    .replace(")", "")
                    .replace("\"", "")
                    .replace("(", "");

                let value = escaped.split("OR")
                    .map(|x| x.trim().to_lowercase())
                    .filter(|x| !x.is_empty())
                    .map(|text| {
                        if text.contains(" ") {
                            format!("({{title description source tags original_name original_directory document_title document_author}}: NEAR({}, 1000))", text)
                        } else {
                            format!("({{title description source tags original_name original_directory document_title document_author}}: \"{}\")", text)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" OR ");

                query = query.bind(value);
            } else {
                query = query.bind(format!("%{text}%"));
                query = query.bind(format!("%{text}%"));
                query = query.bind(format!("%{text}%"));
                query = query.bind(format!("%{text}%"));
                query = query.bind(format!("%{text}%"));
                query = query.bind(format!("%{text}%"));
                query = query.bind(format!("%{text}%"));
                query = query.bind(format!("%{text}%"));
            }
        }

        if let Some(tag_ids) = query_values.tags.as_ref() {
            for tag in tag_ids {
                query = query.bind(*tag as i64);
            }
        }

        if let Some(source) = query_values.source.as_ref() {
            query = query.bind(source.as_str());
        }

        query
    }

    async fn post_update<'a, E: Executor<'a, Database = Sqlite>>(
        &self,
        value: &Post,
        executor: E,
    ) -> Result<(), DataSourceError> {
        sqlx::query("UPDATE posts SET source = ?, title = ?, description = ?, created_at = ?, import_batch_id = ? WHERE post_id = ?")
            .bind(value.source.as_ref().map(|url| url.as_str()))
            .bind(value.title.as_deref())
            .bind(value.description.as_deref())
            .bind(value.created_at)
            .bind(value.import_batch.id() as i64)
            .bind(value.id as i64)
            .execute(executor)
            .await?;

        Ok(())
    }
}

#[async_trait]
impl MediaDataSource for SqliteIndex {
    async fn add(&self, value: &mut Media) -> Result<(), DataSourceError> {
        let id = sqlx::query("INSERT INTO media(width, height, duration, mime_type, mime_sub_type, file_size, file_id, sha256, md5, sha1, document_pages, document_title, document_author, page_width, page_height, video_encoding) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)")
            .bind(value.metadata.width())
            .bind(value.metadata.height())
            .bind(value.metadata.duration())
            .bind(value.mime.ty().as_str())
            .bind(value.mime.subty().as_str())
            .bind(value.file_size as i64)
            .bind(value.file_id.as_bytes().as_slice())
            .bind(value.sha256.as_str())
            .bind(value.md5.as_str())
            .bind(value.sha1.as_str())
            .bind(value.metadata.pages())
            .bind(value.metadata.title())
            .bind(value.metadata.author())
            .bind(value.metadata.page_size().map(|s| s.width))
            .bind(value.metadata.page_size().map(|s| s.height))
            .bind(value.metadata.video_encoding())
            .execute(&self.write_pool)
            .await?
            .last_insert_rowid();

        value.id = id as u64;

        Ok(())
    }

    async fn remove(&self, value: &Media) -> Result<(), DataSourceError> {
        sqlx::query("DELETE FROM media WHERE media_id = ?")
            .bind(value.id as i64)
            .execute(&self.write_pool)
            .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Media>, DataSourceError> {
        let mut rows = sqlx::query("SELECT * FROM media WHERE media_id = ?")
            .bind(id as i64)
            .map(|r| Self::map_media(&r))
            .fetch(&self.read_pool);

        if let Some(row) = rows.try_next().await? {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    async fn get_by_sha256(&self, sha256: &str) -> Result<Option<Media>, DataSourceError> {
        let mut rows = sqlx::query("SELECT * FROM media WHERE sha256 = ?")
            .bind(sha256)
            .map(|r| Self::map_media(&r))
            .fetch(&self.read_pool);

        if let Some(row) = rows.try_next().await? {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    async fn get_total_size(&self) -> Result<u64, DataSourceError> {
        let total_size: (i64,) = sqlx::query_as("SELECT SUM(file_size) FROM media")
            .fetch_one(&self.read_pool)
            .await?;

        Ok(total_size.0 as u64)
    }

    async fn get_count(&self) -> Result<u64, DataSourceError> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM media")
            .fetch_one(&self.read_pool)
            .await?;

        Ok(count.0 as u64)
    }
}

#[async_trait]
impl PostItemDataSource for SqliteIndex {
    async fn add(&self, value: &mut PostItem) -> Result<(), DataSourceError> {
        sqlx::query("INSERT INTO post_items(post_id, item_order, content_id) VALUES(?,?,?)")
            .bind(value.post.id() as i64)
            .bind(value.position)
            .bind(value.content.id() as i64)
            .execute(&self.write_pool)
            .await?;

        Ok(())
    }

    async fn get_by_id(
        &self,
        post_item: u64,
        position: i32,
    ) -> Result<Option<PostItem>, DataSourceError> {
        let mut rows = sqlx::query("SELECT * FROM post_items WHERE post_id = ? AND item_order = ?")
            .bind(post_item as i64)
            .bind(position)
            .map(|r| Self::map_post_item(&r))
            .fetch(&self.read_pool);

        if let Some(row) = rows.try_next().await? {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    async fn get_page_from_post(
        &self,
        post_id: u64,
        page: &PageParams,
    ) -> Result<Page<PostItem>, DataSourceError> {
        let mut conn = self.read_pool.acquire().await?;

        let total_row_count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM post_items WHERE post_id = ?")
                .bind(post_id as i64)
                .fetch_one(conn.deref_mut())
                .await?;

        let rows = sqlx::query(
            "SELECT * FROM post_items WHERE post_id = ? ORDER BY item_order ASC LIMIT ? OFFSET ?",
        )
        .bind(post_id as i64)
        .bind(page.page_size() as i64)
        .bind(page.offset() as i64)
        .map(|r| Self::map_post_item(&r))
        .fetch_all(conn.deref_mut())
        .await?;

        Ok(Page {
            page_size: page.page_size(),
            page_number: page.offset(),
            total_row_count: total_row_count.0 as usize,
            data: rows.into_iter().filter_map(|x| x.ok()).collect(),
        })
    }
}

#[async_trait]
impl PostDataSource for SqliteIndex {
    async fn add(&self, value: &mut Post) -> Result<(), DataSourceError> {
        let id = sqlx::query("INSERT INTO posts(source, title, description, import_batch_id, created_at) VALUES(?,?,?,?,?)")
            .bind(value.source.as_ref().map(|url| url.as_str()))
            .bind(value.title.as_deref())
            .bind(value.description.as_deref())
            .bind(value.import_batch.id() as i64)
            .bind(value.created_at)
            .execute(&self.write_pool)
            .await?
            .last_insert_rowid();

        value.id = id as u64;

        Ok(())
    }

    async fn update(&self, value: &Post) -> Result<(), DataSourceError> {
        self.post_update(value, &self.write_pool).await?;
        Ok(())
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Post>, DataSourceError> {
        let mut rows = sqlx::query(" SELECT * FROM posts WHERE post_id = ?")
            .bind(id as i64)
            .map(|r| Self::map_post(&r))
            .fetch(&self.read_pool);

        if let Some(row) = rows.try_next().await? {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    async fn get_page(&self, page: PageParams) -> Result<Page<Post>, DataSourceError> {
        let mut conn = self.read_pool.acquire().await?;

        let total_row_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts")
            .fetch_one(conn.deref_mut())
            .await?;

        let rows = sqlx::query("SELECT * FROM posts LIMIT ? OFFSET ?")
            .bind(page.page_size() as i64)
            .bind(page.offset() as i64)
            .map(|r| Self::map_post(&r))
            .fetch_all(conn.deref_mut())
            .await?;

        Ok(Page {
            page_size: page.page_size(),
            page_number: page.offset(),
            total_row_count: total_row_count.0 as usize,
            data: rows.into_iter().filter_map(|x| x.ok()).collect(),
        })
    }
}

#[async_trait]
impl ContentDataSource for SqliteIndex {
    async fn add(&self, value: &mut Content) -> Result<(), DataSourceError> {
        sqlx::query(
            "INSERT INTO content(content_id, thumbnail_id, compatibility_content_id) VALUES(?,?,?)",
        )
        .bind(value.content.id() as i64)
        .bind(value.thumbnail.id() as i64)
        .bind(Option::<i64>::None)
        .execute(&self.write_pool)
        .await?;

        Ok(())
    }

    async fn get_by_content_id(&self, id: u64) -> Result<Option<Content>, DataSourceError> {
        let mut rows = sqlx::query("SELECT * FROM content WHERE content_id = ?")
            .bind(id as i64)
            .map(|r| Self::map_content(&r))
            .fetch(&self.read_pool);

        if let Some(row) = rows.try_next().await? {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    async fn update_thumbnail_id(
        &self,
        new_id: u64,
        content: &mut Content,
    ) -> Result<(), DataSourceError> {
        sqlx::query("UPDATE content SET thumbnail_id = ? WHERE content_id = ?")
            .bind(new_id as i64)
            .bind(content.content.id() as i64)
            .execute(&self.write_pool)
            .await?;

        content.thumbnail = ManyToOne::Id(new_id);

        Ok(())
    }
}

#[async_trait]
impl ImportBatchDataSource for SqliteIndex {
    async fn add(&self, value: &mut ImportBatch) -> Result<(), DataSourceError> {
        let id = sqlx::query("INSERT INTO import_batches DEFAULT VALUES")
            .execute(&self.write_pool)
            .await?
            .last_insert_rowid();

        value.id = id as u64;

        Ok(())
    }
}

#[async_trait]
impl TagDataSource for SqliteIndex {
    async fn add(&self, value: &mut Tag) -> Result<(), DataSourceError> {
        let id = sqlx::query("INSERT INTO tags(name, group_id, created_at) VALUES(?, ?, ?)")
            .bind(value.name.as_str())
            .bind(value.group.as_ref().map(|g| g.id() as i64))
            .bind(value.created_at)
            .execute(&self.write_pool)
            .await?
            .last_insert_rowid();

        value.id = id as u64;

        Ok(())
    }

    async fn update(&self, value: &Tag) -> Result<(), DataSourceError> {
        sqlx::query("UPDATE tags SET name = ?, group_id = ? WHERE tag_id = ?")
            .bind(value.name.as_str())
            .bind(value.group.as_ref().map(|g| g.id() as i64))
            .bind(value.id as i64)
            .execute(&self.write_pool)
            .await?;

        Ok(())
    }

    async fn delete(&self, tag_id: u64) -> Result<(), DataSourceError> {
        let mut tx = self.write_pool.begin().await?;

        sqlx::query("DELETE FROM tags_posts WHERE tag_id = ?")
            .bind(tag_id as i64)
            .execute(tx.deref_mut())
            .await?;

        sqlx::query("DELETE FROM tags WHERE tag_id = ?")
            .bind(tag_id as i64)
            .execute(tx.deref_mut())
            .await?;

        tx.commit().await?;

        Ok(())
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<Tag>, DataSourceError> {
        let mut rows = sqlx::query("SELECT * FROM tags WHERE tag_id = ?")
            .bind(id as i64)
            .map(|r| Self::map_tag(&r))
            .fetch(&self.read_pool);

        if let Some(row) = rows.try_next().await? {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    async fn add_tag_to_post(&self, tag_id: u64, post_id: u64) -> Result<(), DataSourceError> {
        sqlx::query("INSERT INTO tags_posts(tag_id, post_id) VALUES(?, ?)")
            .bind(tag_id as i64)
            .bind(post_id as i64)
            .execute(&self.write_pool)
            .await?;

        Ok(())
    }

    async fn remove_tag_to_post(&self, tag_id: u64, post_id: u64) -> Result<(), DataSourceError> {
        sqlx::query("DELETE FROM tags_posts WHERE tag_id = ? AND post_id = ?")
            .bind(tag_id as i64)
            .bind(post_id as i64)
            .execute(&self.write_pool)
            .await?;

        Ok(())
    }
}

#[async_trait]
impl TagGroupDataSource for SqliteIndex {
    async fn add(&self, value: &mut TagGroup) -> Result<(), DataSourceError> {
        let id = sqlx::query("INSERT INTO tag_group(name, color, created_at) VALUES(?, ?, ?)")
            .bind(value.name.as_str())
            .bind(value.hex_color.as_str())
            .bind(value.created_at)
            .execute(&self.write_pool)
            .await?
            .last_insert_rowid();

        value.id = id as u64;

        Ok(())
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<TagGroup>, DataSourceError> {
        let mut rows = sqlx::query("SELECT * FROM tag_group WHERE group_id = ?")
            .bind(id as i64)
            .map(|r| Self::map_tag_group(&r))
            .fetch(&self.read_pool);

        if let Some(row) = rows.try_next().await? {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    async fn search(
        &self,
        page: &PageParams,
        query: &str,
        exact: bool,
    ) -> Result<Page<TagGroup>, DataSourceError> {
        let mut conn = self.read_pool.acquire().await?;

        let total_row_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tag_group")
            .fetch_one(conn.deref_mut())
            .await?;

        let query_is_empty = query.len() < 3;

        let where_clause = if exact {
            " WHERE name = ?"
        } else {
            if query_is_empty {
                " WHERE name LIKE ?"
            } else {
                "WHERE tag_groups_vtab MATCH (?)"
            }
        };

        let query_str =
            format!("SELECT * FROM tag_groups_vtab {where_clause} ORDER BY rank LIMIT ? OFFSET ?");

        let mut sql_query = sqlx::query(query_str.as_str());

        if exact {
            sql_query = sql_query.bind(query);
        } else {
            if !query_is_empty {
                let value = query
                    .trim()
                    .split(' ')
                    .map(|word| format!("{{name}}: \"{word}\""))
                    .collect::<Vec<String>>()
                    .join(" OR ");

                sql_query = sql_query.bind(value);
            } else {
                sql_query = sql_query.bind(format!("%{query}%"))
            }
        }

        let rows = sql_query
            .bind(page.page_size() as i64)
            .bind(page.offset() as i64)
            .map(|r| Self::map_tag_group(&r))
            .fetch_all(conn.deref_mut())
            .await?;

        Ok(Page {
            page_size: page.page_size(),
            page_number: page.offset(),
            total_row_count: total_row_count.0 as usize,
            data: rows.into_iter().filter_map(|x| x.ok()).collect(),
        })
    }
}

#[async_trait]
impl CrossDataSource for SqliteIndex {
    async fn get_post_detail(&self, post_id: u64) -> Result<Option<PostDetail>, DataSourceError> {
        let mut get_post_query = sqlx::query(" SELECT *, (SELECT COUNT(*) FROM post_items pi WHERE pi.post_id = p.post_id) as 'item_count' FROM posts p WHERE p.post_id = ?")
            .bind(post_id as i64)
            .fetch(&self.read_pool);

        if let Some(row) = get_post_query.try_next().await? {
            let post = Self::map_post(&row)?;

            let item_count: i64 = row.try_get("item_count")?;

            drop(row);
            drop(get_post_query);

            let tags = CrossDataSource::get_tags_from_post(self, post_id).await?;

            Ok(Some(PostDetail {
                post,
                tags,
                item_count: item_count as usize,
            }))
        } else {
            Ok(None)
        }
    }

    async fn search_posts(
        &self,
        query: &PostSearchQuery,
        page: &PageParams,
    ) -> Result<Page<SearchPost>, DataSourceError> {
        let mut conn = self.read_pool.acquire().await?;

        let order = match (query.order, query.text.is_some()) {
            (Some(PostSearchQueryOrder::Newest), _)
            | (None, _)
            | (Some(PostSearchQueryOrder::Relevant), false) => "p.created_at DESC",
            (Some(PostSearchQueryOrder::Oldest), _) => "p.created_at ASC",
            (Some(PostSearchQueryOrder::Relevant), true) => "rank ASC, p.created_at DESC",
            (Some(PostSearchQueryOrder::Random(_)), _) => {
                "substr(p.post_id * ?, length(p.post_id) + 2)"
            }
        };

        let table = if query.text.is_none() {
            "posts"
        } else {
            "posts_vtab"
        };

        let after_where = format!("ORDER BY {order} LIMIT ? OFFSET ?");

        let search_query_str = SqliteIndex::create_search_query_str(query, &format!("SELECT p.*, m.*, pi.original_name,
        (SELECT COUNT(*) FROM post_items pi WHERE pi.post_id = p.post_id) as 'item_count',
        (SELECT SUM(c.duration) FROM post_items pi JOIN media c ON pi.content_id = c.media_id WHERE pi.post_id = p.post_id) as 'total_duration',
        (SELECT COUNT(*) FROM media WHERE media_id IN (SELECT content_id FROM post_items WHERE post_id = p.post_id) AND mime_type = 'image' AND mime_sub_type != 'gif') as 'contains_image',
        (SELECT COUNT(*) FROM media WHERE media_id IN (SELECT content_id FROM post_items WHERE post_id = p.post_id) AND mime_type = 'video') as 'contains_video',
        (SELECT COUNT(*) FROM media WHERE media_id IN (SELECT content_id FROM post_items WHERE post_id = p.post_id) AND mime_type = 'application' AND mime_sub_type != 'pdf') as 'contains_document',
        (SELECT COUNT(*) FROM media WHERE media_id IN (SELECT content_id FROM post_items WHERE post_id = p.post_id) AND mime_type = 'image' AND mime_sub_type = 'gif') as 'contains_moving_image'
        FROM {table} p
        LEFT JOIN (SELECT * FROM post_items ORDER BY item_order ASC) pi ON pi.post_id = p.post_id AND pi.item_order = 0
        LEFT JOIN content c ON pi.content_id = c.content_id
        LEFT JOIN media m ON c.thumbnail_id = m.media_id"), &after_where);

        let mut search_query =
            SqliteIndex::add_search_query_values(query, search_query_str.as_str());

        if let Some(PostSearchQueryOrder::Random(seed)) = query.order {
            search_query = search_query.bind(seed);
        }

        let rows = search_query
            .bind(page.page_size() as i64)
            .bind(page.offset() as i64)
            .map(|r| Self::map_search_post(&r))
            .fetch_all(conn.deref_mut())
            .await?;

        let query_row_count_str = SqliteIndex::create_search_query_str(
            query,
            &format!("SELECT COUNT(*) FROM {table} p"),
            "",
        );
        let query_row_count = SqliteIndex::add_search_query_values(query, &query_row_count_str);

        let total_row_count: i64 = query_row_count
            .map(|r| r.get(0))
            .fetch_one(conn.deref_mut())
            .await?;

        Ok(Page {
            page_size: page.page_size(),
            page_number: page.offset(),
            total_row_count: total_row_count as usize,
            data: rows.into_iter().filter_map(|x| x.ok()).collect(),
        })
    }

    async fn search_items(
        &self,
        post_id: u64,
        query: &PostItemSearchQuery,
        page: PageParams,
    ) -> Result<Page<SearchPostItem>, DataSourceError> {
        let mut conn = self.read_pool.acquire().await?;

        let total_row_count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM post_items WHERE post_id = ?")
                .bind(post_id as i64)
                .fetch_one(conn.deref_mut())
                .await?;

        let rows = sqlx::query(
            "SELECT pi.*, m.*, cm.mime_type || '/' || cm.mime_sub_type as content_mime_type, cm.duration as 'content_duration' FROM post_items pi
        LEFT JOIN content c ON pi.content_id = c.content_id
        LEFT JOIN media cm ON pi.content_id = cm.media_id
        LEFT JOIN media m ON c.thumbnail_id = m.media_id
        WHERE pi.post_id = ? AND (? != 0 OR content_duration > 0)
        ORDER BY pi.item_order ASC
        LIMIT ? OFFSET ?",
        )
            .bind(post_id as i64)
            .bind(!query.require_playable)
            .bind(page.page_size() as i64)
            .bind(page.offset() as i64)
            .map(|r| Self::map_search_post_item(&r))
            .fetch_all(conn.deref_mut())
            .await?;

        Ok(Page {
            page_size: page.page_size(),
            page_number: page.offset(),
            total_row_count: total_row_count.0 as usize,
            data: rows.into_iter().filter_map(|x| x.ok()).collect(),
        })
    }

    async fn get_full_post_item(
        &self,
        post_id: u64,
        position: i32,
    ) -> Result<Option<PostItem>, DataSourceError> {
        let mut rows = sqlx::query(
            "SELECT * FROM post_items pi
            LEFT JOIN posts p ON p.post_id = pi.post_id
            LEFT JOIN content c ON c.content_id = pi.content_id
            LEFT JOIN media m ON m.media_id = pi.content_id
            WHERE pi.post_id = ? AND pi.item_order = ?",
        )
        .bind(post_id as i64)
        .bind(position as i64)
        .map(|r| Self::map_full_post_item(&r))
        .fetch(&self.read_pool);

        if let Some(row) = rows.try_next().await? {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    async fn add_full_post(
        &self,
        data: CreateFullPost,
    ) -> Result<(ImportBatch, Vec<Post>), DataSourceError> {
        let created_at = data.created_at.unwrap_or_else(Utc::now);

        let mut tx = self.write_pool.begin().await?;

        let batch_id = match data.batch_id {
            None => sqlx::query("INSERT INTO import_batches DEFAULT VALUES")
                .execute(tx.deref_mut())
                .await?
                .last_insert_rowid() as u64,
            Some(v) => v,
        };

        let batch = ImportBatch { id: batch_id };

        let amount_of_posts_to_create = data.items.len().max(1);
        let mut posts = Vec::with_capacity(amount_of_posts_to_create);

        for _ in 0..amount_of_posts_to_create {
            let id = sqlx::query("INSERT INTO posts(source, title, description, import_batch_id, created_at) VALUES(?,?,?,?,?)")
                .bind(data.source.as_ref().map(|url| url.as_str()))
                .bind(data.title.as_deref())
                .bind(data.description.as_deref())
                .bind(batch.id as i64)
                .bind(created_at)
                .execute(tx.deref_mut())
                .await?
                .last_insert_rowid();

            let post = Post {
                id: id as u64,
                source: data.source.clone(),
                title: data.title.clone(),
                description: data.description.clone(),
                import_batch: ManyToOne::Obj(batch.clone()),
                created_at,
            };

            posts.push(post);

            if !data.flatten {
                break;
            }
        }

        let mut items = Vec::with_capacity(data.items.len());

        if !data.flatten {
            for post in posts.iter() {
                for (position, item) in data.items.iter().enumerate() {
                    items.push((post.id, item, position));
                }
            }
        } else {
            for (i, post) in posts.iter().enumerate() {
                let item = &data.items[i];
                items.push((post.id, item, 0));
            }
        }

        if !data.items.is_empty() {
            let insert_str = "INSERT INTO post_items(post_id, item_order, content_id, original_name, original_accessed, original_modified, original_directory, uploaded_at) VALUES \n";
            let row_value_str = "(?, ?, ?, ?,?,?,?,?),\n";

            let mut insert_items_query =
                String::with_capacity(insert_str.len() + row_value_str.len() * items.len());

            insert_items_query.push_str(insert_str);

            for _ in items.iter() {
                insert_items_query.push_str(row_value_str);
            }

            // Remove the last newline and comma
            insert_items_query.pop();
            insert_items_query.pop();

            let mut query = sqlx::query(insert_items_query.as_str());

            for (post_id, item, position) in items {
                query = query.bind(post_id as i64);
                query = query.bind(position as i64);
                query = query.bind(item.content_id as i64);

                query = query.bind(item.metadata.original_filename.as_deref());
                query = query.bind(item.metadata.original_accessed_at);
                query = query.bind(item.metadata.original_modified_at);
                query = query.bind(item.metadata.original_directory.as_deref());
                query = query.bind(Utc::now());
            }

            query.execute(tx.deref_mut()).await?;
        }

        if !data.tag_ids.is_empty() {
            let insert_str = "INSERT INTO tags_posts(tag_id, post_id) VALUES \n";
            let row_value_str = "(?, ?),\n";

            let mut insert_items_query = String::with_capacity(
                insert_str.len() + row_value_str.len() * (data.tag_ids.len() * posts.len()),
            );

            insert_items_query.push_str(insert_str);

            for _ in 0..posts.len() * data.tag_ids.len() {
                insert_items_query.push_str(row_value_str);
            }

            // Remove the last newline and comma
            insert_items_query.pop();
            insert_items_query.pop();

            let mut query = sqlx::query(insert_items_query.as_str());

            for post in posts.iter() {
                for tag_id in data.tag_ids.iter() {
                    query = query.bind(*tag_id as i64);
                    query = query.bind(post.id as i64);
                }
            }

            query.execute(tx.deref_mut()).await?;
        }

        tx.commit().await?;

        Ok((batch, posts))
    }

    async fn search_tags(
        &self,
        page: &PageParams,
        query: &str,
        exact: bool,
    ) -> Result<Page<SearchTag>, DataSourceError> {
        let mut conn = self.read_pool.acquire().await?;

        let total_row_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tags")
            .fetch_one(conn.deref_mut())
            .await?;

        let query_is_empty = query.len() < 3;

        let where_clause = if query.is_empty() {
            ""
        } else {
            if exact {
                " WHERE t.name = ?"
            } else {
                if query_is_empty {
                    " WHERE t.name LIKE ?"
                } else {
                    "WHERE tags_vtab MATCH (?)"
                }
            }
        };

        let query_str =
            format!("SELECT t.*, g.name as g_name, g.color as color, g.created_at as g_created_at, (SELECT COUNT(*) FROM tags_posts tpc WHERE tpc.tag_id = t.tag_id) as 'linked_posts' FROM tags_vtab t LEFT JOIN tag_group g ON t.group_id = g.group_id {where_clause} ORDER BY rank, t.created_at DESC LIMIT ? OFFSET ?");

        let mut sql_query = sqlx::query(query_str.as_str());

        if !query.is_empty() {
            if exact {
                sql_query = sql_query.bind(query);
            } else {
                if !query_is_empty {
                    let value = query
                        .trim()
                        .split(' ')
                        .map(|word| format!("{{name}}: \"{word}\""))
                        .collect::<Vec<String>>()
                        .join(" OR ");

                    sql_query = sql_query.bind(value);
                } else {
                    sql_query = sql_query.bind(format!("%{query}%"))
                }
            }
        }

        let rows = sql_query
            .bind(page.page_size() as i64)
            .bind(page.offset() as i64)
            .map(|r| Self::map_search_tag(&r))
            .fetch_all(conn.deref_mut())
            .await?;

        Ok(Page {
            page_size: page.page_size(),
            page_number: page.offset(),
            total_row_count: total_row_count.0 as usize,
            data: rows.into_iter().filter_map(|x| x.ok()).collect(),
        })
    }

    async fn update_full_post(&self, value: &Post, tags: &[u64]) -> Result<(), DataSourceError> {
        let mut tx = self.write_pool.begin().await?;

        self.post_update(value, tx.deref_mut()).await?;

        sqlx::query("DELETE FROM tags_posts WHERE post_id = ?")
            .bind(value.id as i64)
            .execute(tx.deref_mut())
            .await?;

        if !tags.is_empty() {
            let value_binds = tags.iter().map(|_| "(?, ?)").collect::<Vec<_>>().join(", ");
            let insert_query =
                format!("INSERT INTO tags_posts(post_id, tag_id) VALUES {value_binds}");

            let mut query = sqlx::query(&insert_query);

            for tag in tags {
                query = query.bind(value.id as i64).bind(*tag as i64);
            }

            query.execute(tx.deref_mut()).await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn cascade_delete_post(&self, id: u64) -> Result<(), DataSourceError> {
        let mut tx = self.write_pool.begin().await?;

        sqlx::query("DELETE FROM post_items WHERE post_id = ?")
            .bind(id as i64)
            .execute(tx.deref_mut())
            .await?;

        sqlx::query("DELETE FROM tags_posts WHERE post_id = ?")
            .bind(id as i64)
            .execute(tx.deref_mut())
            .await?;

        sqlx::query("DELETE FROM posts WHERE post_id = ?")
            .bind(id as i64)
            .execute(tx.deref_mut())
            .await?;

        tx.commit().await?;

        Ok(())
    }

    async fn graph_post(&self, query: &PostGraphQuery) -> Result<Graph, DataSourceError> {
        let mut conn = self.read_pool.acquire().await?;

        let select = match query.select {
            GraphSelect::Count => "SUM(COUNT(*) * 1.0) over (ROWS UNBOUNDED PRECEDING)",
        };

        let y_axis = match query.discriminator {
            GraphDiscriminator::Duration(_) => "p.created_at",
            GraphDiscriminator::None => "null",
        };

        let y_axis_kind = match query.discriminator {
            GraphDiscriminator::Duration(_) => "date",
            GraphDiscriminator::None => "none",
        };

        let before_where = format!("SELECT {select}, {y_axis}, '{y_axis_kind}' FROM posts_vtab p");
        let after_where = match query.discriminator {
            GraphDiscriminator::Duration(_) => "GROUP BY strftime('%s', p.created_at) / ?",
            GraphDiscriminator::None => "",
        };

        let query_str =
            SqliteIndex::create_search_query_str(&query.filter, &before_where, &after_where);
        let mut graph_query = SqliteIndex::add_search_query_values(&query.filter, &query_str);

        match query.discriminator {
            GraphDiscriminator::Duration(duration) => {
                graph_query = graph_query.bind(duration.as_secs() as i64);
            }
            _ => {}
        }

        let rows = graph_query
            .map(|r| Self::map_graph_point(&r))
            .fetch_all(conn.deref_mut())
            .await?
            .into_iter()
            .collect::<Result<Vec<_>, DataSourceError>>()?;
        Ok(Graph { points: rows })
    }

    async fn get_tags_from_post(&self, post_id: u64) -> Result<Vec<SearchTag>, DataSourceError> {
        let rows = sqlx::query(
            "SELECT t.*, g.name as g_name, g.color as color, g.created_at as g_created_at, (SELECT COUNT(*) FROM tags_posts tpc WHERE tpc.tag_id = t.tag_id) as 'linked_posts' FROM tags_posts tp JOIN tags t ON t.tag_id = tp.tag_id LEFT JOIN tag_group g ON t.group_id = g.group_id WHERE post_id = ?",
        )
            .bind(post_id as i64)
            .map(|r| Self::map_search_tag(&r))
            .fetch_all(&self.read_pool)
            .await?;

        Ok(rows.into_iter().filter_map(|x| Some(x.unwrap())).collect())
    }

    async fn get_tag_detail(&self, tag_id: u64) -> Result<Option<TagDetail>, DataSourceError> {
        let mut get_tag_query = sqlx::query(
            "SELECT t.*, g.name as g_name, g.color as color, g.created_at as g_created_at FROM tags tp JOIN tags t ON t.tag_id = tp.tag_id LEFT JOIN tag_group g ON t.group_id = g.group_id WHERE t.tag_id = ?",
        )
            .bind(tag_id as i64)
            .fetch(&self.read_pool);

        if let Some(row) = get_tag_query.try_next().await? {
            let tag = Self::map_full_tag(&row)?;

            drop(row);
            drop(get_tag_query);

            Ok(Some(TagDetail { tag }))
        } else {
            Ok(None)
        }
    }

    async fn gc(&self) -> Result<u64, DataSourceError> {
        let mut conn = self.write_pool.acquire().await?;

        let mut rows_affected = 0;

        rows_affected += sqlx::query("pragma wal_checkpoint(truncate)")
            .execute(&mut *conn)
            .await?
            .rows_affected();

        rows_affected += sqlx::query("pragma vacuum")
            .execute(&mut *conn)
            .await?
            .rows_affected();

        rows_affected += sqlx::query("pragma optimize")
            .execute(&mut *conn)
            .await?
            .rows_affected();

        Ok(rows_affected)
    }
}
