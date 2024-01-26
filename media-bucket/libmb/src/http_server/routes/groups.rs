use crate::data_source::PageParams;
use crate::http_models::CreateTagGroupRequest;
use crate::http_server::instance::Session;
use crate::http_server::web_error::WebError;
use crate::model::TagGroup;
use actix_web::{get, post, web, Responder};
use chrono::Utc;
use log::info;
use serde::Deserialize;

fn is_valid_hex(value: &str) -> bool {
    if value.len() != 7 {
        return false;
    }

    if !value.starts_with('#') {
        return false;
    }

    for c in value.chars().skip(1) {
        if !c.is_ascii_uppercase() && !c.is_ascii_digit() {
            return false;
        }
    }

    true
}

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
    let groups = session
        .bucket()
        .data_source()
        .tag_groups()
        .search(
            &page,
            params.query.as_deref().unwrap_or(""),
            params.exact.unwrap_or(false),
        )
        .await?;

    Ok(web::Json(groups))
}

#[get("{id}")]
pub async fn show(session: Session, id: web::Path<(u64, u64)>) -> Result<impl Responder, WebError> {
    let id = id.into_inner().1;

    let group = session
        .bucket()
        .data_source()
        .tag_groups()
        .get_by_id(id)
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    Ok(web::Json(group))
}

#[post("")]
pub async fn store(
    session: Session,
    req: web::Json<CreateTagGroupRequest>,
) -> Result<impl Responder, WebError> {
    if !is_valid_hex(&req.hex_color) {
        return Err(WebError::ParseError);
    }

    let mut group = TagGroup {
        id: 0,
        name: req.name.clone(),
        hex_color: req.hex_color.clone(),
        created_at: Utc::now(),
    };

    session
        .bucket()
        .data_source()
        .tag_groups()
        .add(&mut group)
        .await?;

    info!("Created group {}", group.id);

    Ok(web::Json(group))
}
