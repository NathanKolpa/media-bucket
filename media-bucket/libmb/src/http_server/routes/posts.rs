use actix_web::{delete, get, post, put, Responder, web};
use log::info;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::data_source::PageParams;
use crate::http_server::instance::Session;
use crate::http_server::web_error::WebError;
use crate::model::{CreateFullPost, ImportBatch, Post, PostSearchQuery};

#[get("")]
pub async fn index(session: Session, query: PostSearchQuery, page: PageParams) -> Result<impl Responder, WebError> {
    let posts = session.bucket().data_source().cross().search(&query, &page).await?;

    Ok(web::Json(posts))
}

#[get("/{id}")]
pub async fn show(session: Session, id: web::Path<u64>) -> Result<impl Responder, WebError> {
    let post = session
        .bucket()
        .data_source()
        .cross()
        .get_post_detail(id.into_inner())
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    Ok(web::Json(post))
}

#[delete("/{id}")]
pub async fn delete(session: Session, id: web::Path<u64>) -> Result<impl Responder, WebError> {
    session
        .bucket()
        .data_source()
        .cross()
        .cascade_delete_post(*id)
        .await?;

    info!("Deleted post {id}");

    Ok(web::Json(()))
}

#[get("/{id}/items/{position}")]
pub async fn show_item(
    session: Session,
    path: web::Path<(u64, i32)>,
) -> Result<impl Responder, WebError> {
    let (id, position) = path.into_inner();

    let item = session
        .bucket()
        .data_source()
        .cross()
        .get_full_post_item(id, position)
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    Ok(web::Json(item))
}

#[get("/{id}/items")]
pub async fn index_items(
    session: Session,
    id: web::Path<u64>,
    page: PageParams,
) -> Result<impl Responder, WebError> {
    let post = session
        .bucket()
        .data_source()
        .posts()
        .get_by_id(id.into_inner())
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    let items = session
        .bucket()
        .data_source()
        .cross()
        .search_items(post.id, page)
        .await?;

    Ok(web::Json(items))
}

#[derive(Deserialize)]
pub struct CreatePostTagRequest {
    tag_id: u64,
    enable: bool,
}

#[post("/{id}/tags")]
pub async fn store_tags(
    id: web::Path<u64>,
    session: Session,
    req: web::Json<CreatePostTagRequest>,
) -> Result<impl Responder, WebError> {
    if req.enable {
        session
            .bucket()
            .data_source()
            .tags()
            .add_tag_to_post(req.tag_id, *id)
            .await?;

        info!("Added tag {} to post {}", req.tag_id, id);
    } else {
        session
            .bucket()
            .data_source()
            .tags()
            .remove_tag_to_post(req.tag_id, *id)
            .await?;

        info!("Removed tag {} from post {}", req.tag_id, id);
    }

    Ok(web::Json(()))
}

#[derive(Serialize)]
pub struct CreateFullPostResponse {
    batch: ImportBatch,
    posts: Vec<Post>,
}

#[post("")]
pub async fn store(
    session: Session,
    req: web::Json<CreateFullPost>,
) -> Result<impl Responder, WebError> {
    let (batch, posts) = session
        .bucket()
        .data_source()
        .cross()
        .add_full_post(req.clone())
        .await?;

    info!("Created {} post(s) batch: {}", posts.len(), batch.id);

    Ok(web::Json(CreateFullPostResponse { posts, batch }))
}

#[derive(Deserialize)]
pub struct UpdatePostRequest {
    title: Option<String>,
    description: Option<String>,
    source: Option<Url>,
}

#[put("/{id}")]
pub async fn update(
    session: Session,
    id: web::Path<u64>,
    req: web::Json<UpdatePostRequest>,
) -> Result<impl Responder, WebError> {
    let mut post = session
        .bucket()
        .data_source()
        .posts()
        .get_by_id(*id)
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    post.title = req.title.clone();
    post.description = req.description.clone();
    post.source = req.source.clone();

    session.bucket().data_source().posts().update(&post).await?;

    Ok(web::Json(post))
}
