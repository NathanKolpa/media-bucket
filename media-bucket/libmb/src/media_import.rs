use std::path::Path;
use futures::AsyncSeek;
use mediatype::MediaTypeBuf;
use tokio::io::{AsyncRead, AsyncWrite};
use uuid::Uuid;
use crate::data_source::MediaImportError;
use crate::model::Media;

mod digest;
mod file_data;
mod imagemagick;

pub enum ImportInput<'a, R: AsyncRead + AsyncSeek> {
    File(&'a Path),
    Stream(R)
}

pub struct MediaImportOutput {
    pub content: Media,
    pub thumbnail: Media,
}

pub async fn import_file_with_thumbnail<O: AsyncWrite + Unpin>(
    file: &Path,
    mime: MediaTypeBuf,
    content_location: Uuid,
    content_output: O,
    thumbnail_location: Uuid,
    thumbnail_output: O,
) -> Result<MediaImportOutput, MediaImportError> {
    todo!()
}