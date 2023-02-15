use std::path::Path;

use async_trait::async_trait;
use mediatype::MediaTypeBuf;
use thiserror::Error;
use uuid::Uuid;

use crate::data_source::*;
use crate::local::sqlite::{SqliteError, SqliteIndex};
use crate::media_import::{import_file_with_thumbnail, MediaImportOutput};
use crate::model::{Content, ManyToOne, Media};

#[cfg(feature = "encryption")]
mod encrypted_fs_storage;

mod sqlite;

#[cfg(feature = "encryption")]
mod secret;

#[derive(Error, Debug)]
pub enum LocalDataSourceError {
    #[error("Database does not exist")]
    DatabaseDoesNotExist,

    #[error("Media folder does not exist")]
    MediaDirectoryDoesNotExist,

    #[error("Password file does not exist")]
    PasswordFileDoesNotExist,

    #[error("Tried to create a file that already exists")]
    FileExists,

    #[error("Cannot create encryption metadata")]
    CannotCreateEncryptionMetadata,

    #[error("Cannot create media dir")]
    CannotCreateMediaDir,

    #[error("Failed to open passwords.json: {0}")]
    FailedToOpenPasswordsFile(std::io::Error),

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Sqlite error {0}")]
    SqliteError(#[from] SqliteError),
}

#[cfg(feature = "encryption")]
pub type EncryptedLocalDataSource =
    LocalDataSource<encrypted_fs_storage::EncryptedFileDataSource, secret::EncryptionMetadata>;

pub struct LocalDataSource<FileStorage: BlobDataSource, Passwords: PasswordDataSource> {
    passwords: Passwords,
    storage: FileStorage,
    sqlite: SqliteIndex,
}

#[cfg(feature = "encryption")]
impl EncryptedLocalDataSource {
    pub async fn create_encrypted(
        path: &Path,
        password: &str,
    ) -> Result<Self, LocalDataSourceError> {
        let db_location = path.join("index.db");
        let media_location = path.join("media");
        let passwords_location = path.join("encryption.json");

        if db_location.exists() {
            return Err(LocalDataSourceError::FileExists);
        }

        if media_location.exists() {
            return Err(LocalDataSourceError::FileExists);
        }

        if passwords_location.exists() {
            return Err(LocalDataSourceError::FileExists);
        }

        let secret = secret::Secret::random();

        let encrypted_secret = secret::EncryptedSecret::encrypt(password, &secret);
        let encryption_metadata = secret::EncryptionMetadata::new(encrypted_secret);

        encryption_metadata
            .save_to(&passwords_location)
            .await
            .map_err(|_| LocalDataSourceError::CannotCreateEncryptionMetadata)?;

        let sqlite = SqliteIndex::create_encrypted(&db_location, secret.clone()).await?;

        tokio::fs::create_dir(&media_location)
            .await
            .map_err(|_| LocalDataSourceError::CannotCreateMediaDir)?;

        let storage = encrypted_fs_storage::EncryptedFileDataSource::new(media_location, secret);

        Ok(Self {
            storage,
            sqlite,
            passwords: encryption_metadata,
        })
    }

    pub async fn open_encrypted(path: &Path, password: &str) -> Result<Self, LocalDataSourceError> {
        let db_location = path.join("index.db");
        let media_location = path.join("media");
        let passwords_location = path.join("encryption.json");

        if !db_location.exists() {
            return Err(LocalDataSourceError::DatabaseDoesNotExist);
        }

        if !media_location.exists() {
            return Err(LocalDataSourceError::MediaDirectoryDoesNotExist);
        }

        if !passwords_location.exists() {
            return Err(LocalDataSourceError::PasswordFileDoesNotExist);
        }

        let encryption_metadata = secret::EncryptionMetadata::from_file(&passwords_location)
            .await
            .map_err(LocalDataSourceError::FailedToOpenPasswordsFile)?;

        let secret = encryption_metadata
            .decrypt_secret(password)
            .ok_or(LocalDataSourceError::InvalidPassword)?;

        let sqlite = SqliteIndex::open_encrypted(&db_location, secret.clone()).await?;
        let storage = encrypted_fs_storage::EncryptedFileDataSource::new(media_location, secret);

        Ok(Self {
            storage,
            sqlite,
            passwords: encryption_metadata,
        })
    }
}
impl<FileStorage: BlobDataSource, Passwords: PasswordDataSource> DataSource
    for LocalDataSource<FileStorage, Passwords>
{
    fn blobs(&self) -> &dyn BlobDataSource {
        &self.storage
    }

    fn media(&self) -> &dyn MediaDataSource {
        &self.sqlite
    }

    fn content(&self) -> &dyn ContentDataSource {
        &self.sqlite
    }

    fn post_items(&self) -> &dyn PostItemDataSource {
        &self.sqlite
    }

    fn posts(&self) -> &dyn PostDataSource {
        &self.sqlite
    }

    fn import_batches(&self) -> &dyn ImportBatchDataSource {
        &self.sqlite
    }

    fn tags(&self) -> &dyn TagDataSource {
        &self.sqlite
    }

    fn tag_groups(&self) -> &dyn TagGroupDataSource {
        &self.sqlite
    }

    fn passwords(&self) -> &dyn PasswordDataSource {
        &self.passwords
    }

    fn media_import(&self) -> &dyn MediaImportDataSource {
        self
    }

    fn cross(&self) -> &dyn CrossDataSource {
        &self.sqlite
    }
}

#[async_trait]
impl<FileStorage: BlobDataSource, Passwords: PasswordDataSource> MediaImportDataSource
    for LocalDataSource<FileStorage, Passwords>
{
    async fn import_media(
        &self,
        mime: MediaTypeBuf,
        path: &Path,
    ) -> Result<Content, MediaImportError> {
        let media_id = Uuid::new_v4();
        let media_writer = self.blobs().add(&media_id).await?;

        let thumb_id = Uuid::new_v4();
        let thumb_writer = self.blobs().add(&thumb_id).await?;

        let MediaImportOutput { mut content, mut thumbnail }  = import_file_with_thumbnail(
            path,
            mime,
            media_id,
            Box::into_pin(media_writer),
            thumb_id,
            Box::into_pin(thumb_writer),
        )
        .await?;

        content = self.add_or_get_media(content).await?;
        thumbnail = self.add_or_get_media(thumbnail).await?;

        if let Some(mut existing) = self.content().get_by_content_id(content.id).await? {
            if existing.thumbnail.id() != thumbnail.id {
                self.content()
                    .update_thumbnail_id(thumbnail.id, &mut existing)
                    .await?;
            }

            Ok(Content {
                content: ManyToOne::Obj(content),
                thumbnail: ManyToOne::Obj(thumbnail),
            })
        } else {
            let mut new_content = Content {
                content: ManyToOne::Obj(content),
                thumbnail: ManyToOne::Obj(thumbnail),
            };

            self.content().add(&mut new_content).await?;

            Ok(new_content)
        }
    }
}

impl<FileStorage: BlobDataSource, Passwords: PasswordDataSource>
    LocalDataSource<FileStorage, Passwords>
{
    async fn add_or_get_media(&self, mut media: Media) -> Result<Media, DataSourceError> {
        if let Some(existing) = self.media().get_by_sha256(&media.sha256).await? {
            self.blobs().delete(&media.file_id).await?;
            Ok(existing)
        } else {
            self.media().add(&mut media).await?;
            Ok(media)
        }
    }
}
