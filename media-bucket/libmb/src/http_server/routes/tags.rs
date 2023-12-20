use actix_web::{delete, get, post, web, Responder, put};
use chrono::Utc;
use log::info;
use serde::Deserialize;

use crate::data_source::PageParams;
use crate::http_models::{CreateTagRequest, UpdateTagRequest};
use crate::http_server::instance::Session;
use crate::http_server::web_error::WebError;
use crate::model::{ManyToOne, Tag};

#[derive(Deserialize)]
pub struct SearchParams {
    query: Option<String>,
    exact: Option<bool>,
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
        .search(
            &page,
            params.query.as_deref().unwrap_or(""),
            params.exact.unwrap_or(false),
        )
        .await?;

    Ok(web::Json(tags))
}

#[post("")]
pub async fn store(
    session: Session,
    req: web::Json<CreateTagRequest>,
) -> Result<impl Responder, WebError> {
    let mut tag = Tag {
        id: 0,
        name: req.name.clone(),
        group: req.group.map(|id| ManyToOne::Id(id)),
        created_at: Utc::now(),
    };

    session.bucket().data_source().tags().add(&mut tag).await?;

    info!("Created tag {}", tag.id);

    Ok(web::Json(tag))
}

#[put("/{id}")]
pub async fn update(
    session: Session,
    req: web::Json<UpdateTagRequest>,
    id: web::Path<(u64, u64)>,
) -> Result<impl Responder, WebError> {
    let id = id.into_inner().1;

    let mut tag = session
        .bucket()
        .data_source()
        .tags()
        .get_by_id(id)
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    tag.group = req.group.map(|id| ManyToOne::Id(id));
    tag.name = req.name.clone();

    session.bucket().data_source().tags().update(&tag).await?;

    info!("Updated tag {}", tag.id);

    Ok(web::Json(tag))
}

#[delete("/{id}")]
pub async fn delete(
    session: Session,
    id: web::Path<(u64, u64)>,
) -> Result<impl Responder, WebError> {
    let id = id.into_inner().1;

    let tag = session
        .bucket()
        .data_source()
        .tags()
        .get_by_id(id)
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    session.bucket().data_source().tags().delete(tag.id).await?;

    info!("Deleted tag {}", tag.id);

    Ok(web::Json(tag))
}
