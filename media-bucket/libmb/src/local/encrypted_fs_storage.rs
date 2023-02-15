use std::cell::RefCell;
use std::io::{Error, SeekFrom};
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_trait::async_trait;
use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use chacha20::XChaCha20;
use pin_utils::pin_mut;
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncSeek, AsyncWrite, ReadBuf};
use uuid::Uuid;

use crate::data_source::{BlobDataSource, DataSourceError, FileInput, FileOutput};
use crate::local::secret::Secret;

/// This struct represents a wrapper around a file that is encrypted using the XChaCha20 cipher.
///
/// It contains a reference to the underlying file, as well as a buffer for storing the encrypted data.
/// The `EncryptedFileWrapper` struct also contains a reference to the `XChaCha20` cipher, which is used to encrypt and decrypt the data as it is read from or written to the file.
struct EncryptedFileWrapper {
    file: File,
    cipher: XChaCha20,
    position: usize,
}

impl EncryptedFileWrapper {
    pub fn new(file: File, secret: &Secret) -> Self {
        let nonce: [u8; 24] = [0; 24];
        let cipher = XChaCha20::new(secret.bytes().into(), &nonce.into());

        Self {
            file,
            cipher,
            position: 0,
        }
    }
}

thread_local! {
    static BUFFER: RefCell<Vec<u8>> = Default::default();
}

impl AsyncWrite for EncryptedFileWrapper {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        BUFFER.with(|buffer| {
            let mut buffer = buffer.borrow_mut();

            buffer.resize(buf.len(), 0);
            for (i, byte) in buf.iter().enumerate() {
                buffer[i] = *byte;
            }

            self.cipher.apply_keystream(&mut buffer);

            let file = &mut self.file;
            pin_mut!(file);

            let result = file.poll_write(cx, &buffer);

            if let Poll::Ready(Ok(written)) = &result {
                self.position += written;

                if *written != buffer.len() {
                    let position = self.position;
                    self.cipher.seek(position);
                }
            } else {
                let position = self.position;
                self.cipher.seek(position);
            }

            result
        })
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        let file = &mut self.file;
        pin_mut!(file);

        file.poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        let file = &mut self.file;
        pin_mut!(file);

        file.poll_shutdown(cx)
    }
}

impl AsyncRead for EncryptedFileWrapper {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let start = buf.filled().len();

        let result = {
            let file = &mut self.file;
            pin_mut!(file);
            file.poll_read(cx, buf)
        };

        let end = buf.filled().len();

        self.cipher
            .apply_keystream(&mut buf.filled_mut()[start..end]);

        result
    }
}

impl AsyncSeek for EncryptedFileWrapper {
    fn start_seek(mut self: Pin<&mut Self>, position: SeekFrom) -> std::io::Result<()> {
        let file = &mut self.file;
        pin_mut!(file);
        file.start_seek(position)
    }

    fn poll_complete(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<u64>> {
        let file = &mut self.file;
        pin_mut!(file);
        let result = file.poll_complete(cx);

        if let Poll::Ready(Ok(byte_pos)) = &result {
            self.cipher.seek(*byte_pos);
        }

        result
    }
}

pub struct EncryptedFileDataSource {
    base: PathBuf,
    secret: Secret,
}

impl EncryptedFileDataSource {
    pub fn new(base: PathBuf, secret: Secret) -> Self {
        Self { base, secret }
    }
}

#[async_trait]
impl BlobDataSource for EncryptedFileDataSource {
    async fn add(&self, id: &Uuid) -> Result<Box<dyn FileInput>, DataSourceError> {
        if self.has(id).await? {
            return Err(DataSourceError::Duplicate);
        }

        let new_file = File::create(self.base.join(id.to_string())).await?;

        Ok(Box::new(EncryptedFileWrapper::new(
            new_file,
            &self.secret.derive_from_uuid(id),
        )))
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Box<dyn FileOutput>, DataSourceError> {
        if !self.has(id).await? {
            return Err(DataSourceError::NotFound);
        }

        let file = File::open(self.base.join(id.to_string())).await?;

        Ok(Box::new(EncryptedFileWrapper::new(
            file,
            &self.secret.derive_from_uuid(id),
        )))
    }

    async fn delete(&self, id: &Uuid) -> Result<(), DataSourceError> {
        if !self.has(id).await? {
            return Err(DataSourceError::NotFound);
        }

        tokio::fs::remove_file(self.base.join(id.to_string())).await?;

        Ok(())
    }

    async fn has(&self, id: &Uuid) -> Result<bool, DataSourceError> {
        Ok(self.base.join(id.to_string()).exists())
    }
}
