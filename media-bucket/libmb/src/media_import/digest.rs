use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::AsyncWrite;
use crate::data_source::MediaImportError;

pub trait DigestWrite {
    async fn write(&mut self, data: &[u8]) -> Result<(), MediaImportError>;
}

pub trait Digest {
    type Output;
    async fn digest(self) -> Result<Self::Output, MediaImportError>;
}