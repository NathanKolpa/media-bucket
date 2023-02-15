use actix_web::body::SizedStream;
use actix_web::http::{header, StatusCode};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use http_range::HttpRange;

use crate::http_server::instance::Session;
use crate::http_server::stream_file::new_chunked_read;
use crate::http_server::web_error::WebError;

#[get("/{id}")]
pub async fn show(session: Session, id: web::Path<u64>) -> Result<impl Responder, WebError> {
    let media = session
        .bucket()
        .data_source()
        .media()
        .get_by_id(id.into_inner())
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    Ok(web::Json(media))
}

#[get("/{id}/file")]
pub async fn file(
    session: Session,
    id: web::Path<u64>,
    req: HttpRequest,
) -> Result<impl Responder, WebError> {
    let media = session
        .bucket()
        .data_source()
        .media()
        .get_by_id(id.into_inner())
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    let file = session
        .bucket()
        .data_source()
        .blobs()
        .get_by_id(&media.file_id)
        .await?;

    let mut length = media.file_size;
    let mut offset: usize = 0;

    let mut response = HttpResponse::Ok();

    response
        .insert_header((header::ACCEPT_RANGES, "bytes"))
        .content_type(media.mime.as_str());

    if let Some(range_header) = req.headers().get(header::RANGE) {
        if let Ok(header_value) = range_header.to_str() {
            if let Ok(ranges) = HttpRange::parse(header_value, media.file_size as u64) {
                if let Some(range) = ranges.first() {
                    length = range.length as usize;
                    offset = range.start as usize;
                }

                response.insert_header((
                    header::CONTENT_RANGE,
                    format!(
                        "bytes {}-{}/{}",
                        offset,
                        offset + length - 1,
                        media.file_size
                    ),
                ));
            } else {
                response.insert_header((header::CONTENT_RANGE, format!("bytes */{length}")));
                return Ok(response.status(StatusCode::RANGE_NOT_SATISFIABLE).finish());
            }
        }
    }

    if offset != 0 || length != media.file_size {
        response.status(StatusCode::PARTIAL_CONTENT);
    }

    let stream = new_chunked_read(length as u64, offset as u64, Box::into_pin(file));

    Ok(response.body(SizedStream::new(length as u64, stream)))
}
