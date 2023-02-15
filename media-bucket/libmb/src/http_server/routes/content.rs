use std::env;

use actix_web::{post, web, HttpMessage, HttpRequest, Responder};
use futures::StreamExt;
use log::info;
use mediatype::MediaTypeBuf;
use tokio::fs::{remove_file, OpenOptions};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use crate::http_server::web_error::WebError;

use crate::http_server::instance::Session;

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

    let tmp_file_path = env::temp_dir().join(Uuid::new_v4().to_string());

    {
        let mut tmp_file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&tmp_file_path)
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
        .import_media(mime, &tmp_file_path)
        .await;

    remove_file(&tmp_file_path).await?;
    let content = content_result?;

    info!("Uploaded content {}", content.content.id());

    Ok(web::Json(content))
}
