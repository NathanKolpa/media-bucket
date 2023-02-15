use std::path::Path;

use thiserror::Error;

use crate::data_source::DataSource;

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
            todo!()
        }

        match password {
            None => Err(BucketError::PasswordRequired),
            Some(password) => Self::open_encrypted(Path::new(location), password).await,
        }
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
}
