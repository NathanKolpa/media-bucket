use std::path::Path;
use mediatype::MediaTypeBuf;
use tokio::io::AsyncWrite;
use uuid::Uuid;
use crate::data_source::MediaImportError;
use crate::model::Media;

mod digest;
mod file_data;

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