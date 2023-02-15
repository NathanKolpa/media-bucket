use actix_web::{delete, get, post, web, Responder};
use chrono::Utc;
use log::info;
use serde::Deserialize;

use crate::data_source::PageParams;
use crate::http_server::instance::Session;
use crate::http_server::web_error::WebError;
use crate::model::Tag;

#[derive(Deserialize)]
pub struct SearchParams {
    query: Option<String>,
}

#[get("")]
pub async fn index(
    session: Session,
    page: PageParams,
    params: web::Query<SearchParams>,
) -> Result<impl Responder, WebError> {
    let tags = session
        .bucket()
        .data_source()
        .tags()
        .search(&page, params.query.as_deref().unwrap_or(""))
        .await?;

    Ok(web::Json(tags))
}

#[derive(Deserialize)]
pub struct CreateTagRequest {
    name: String,
    group: Option<u64>,
}

#[post("")]
pub async fn store(
    session: Session,
    req: web::Json<CreateTagRequest>,
) -> Result<impl Responder, WebError> {
    let mut tag = Tag {
        id: 0,
        name: req.name.clone(),
        group: None,
        created_at: Utc::now(),
    };

    session.bucket().data_source().tags().add(&mut tag).await?;

    info!("Created tag {} \"{}\"", tag.id, tag.name);

    Ok(web::Json(tag))
}

#[delete("/{id}")]
pub async fn delete(session: Session, id: web::Path<u64>) -> Result<impl Responder, WebError> {
    let tag = session
        .bucket()
        .data_source()
        .tags()
        .get_by_id(*id)
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    session.bucket().data_source().tags().delete(tag.id).await?;

    info!("Deleted tag {} \"{}\"", tag.id, tag.name);

    Ok(web::Json(tag))
}
