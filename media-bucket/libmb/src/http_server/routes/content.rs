use crate::{data_source::ImportSource, http_server::web_error::WebError, media_import::TmpFile};
use actix_web::{post, web, HttpMessage, HttpRequest, Responder};
use futures::StreamExt;
use log::info;
use mediatype::MediaTypeBuf;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

use crate::http_server::instance::Session;

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[post("")]
pub async fn store(
    req: HttpRequest,
    session: Session,
    mut body: web::Payload,
) -> Result<impl Responder, WebError> {
    let mime: MediaTypeBuf = req
        .mime_type()
        .map_err(|_| WebError::ParseError)?
        .ok_or(WebError::MissingMimeType)?
        .as_ref()
        .parse()
        .map_err(|_| WebError::ParseError)?;

    let tmp_file_path = TmpFile::new().await?;

    {
        let mut tmp_file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&tmp_file_path.path())
            .await?;

        while let Some(item) = body.next().await {
            tmp_file.write_all(&item?).await?;
        }

        tmp_file.flush().await?;
    }

    let content_result = session
        .bucket()
        .data_source()
        .media_import()
        .import_media(mime, ImportSource::File(tmp_file_path.path()))
        .await;

    let content = content_result?;

    info!("Uploaded content {}", content.content.id());

    Ok(web::Json(content))
}
