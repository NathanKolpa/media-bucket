use actix_web::body::BodyStream;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;

use crate::http_server::stream_playlist::new_search_playlist;
use crate::http_server::web_error::WebError;
use crate::model::{CreateFullPost, PostGraphQuery, PostSearchQuery};
use crate::{data_source::PageParams, http_server::stream_playlist::new_post_playlist};
use crate::{
    http_models::{CreateFullPostResponse, UpdatePostRequest},
    http_server::stream_playlist::new_content_playlist,
};
use crate::{http_server::instance::Session, model::PostItemSearchQuery};

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[post("graph")]
pub async fn graph(
    session: Session,
    req: web::Json<PostGraphQuery>,
) -> Result<impl Responder, WebError> {
    let graph = session
        .bucket()
        .data_source()
        .cross()
        .graph_post(&req)
        .await?;

    Ok(web::Json(graph))
}

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[get("")]
pub async fn index(
    session: Session,
    query: PostSearchQuery,
    page: PageParams,
) -> Result<impl Responder, WebError> {
    let posts = session
        .bucket()
        .data_source()
        .cross()
        .search_posts(&query, &page)
        .await?;

    Ok(web::Json(posts))
}

#[derive(Deserialize)]
pub struct PlaylistParams {
    include_token: Option<bool>,
    require_playable: Option<bool>,
}

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[get("index.m3u")]
pub async fn index_playlist(
    session: Session,
    mut query: PostSearchQuery,
    params: web::Query<PlaylistParams>,
) -> Result<impl Responder, WebError> {
    let token = if params.include_token.unwrap_or(false) && session.read_only() {
        session.token().map(|s| s.to_string())
    } else {
        None
    };

    query.require_playable = params.require_playable.unwrap_or_default();

    let response = HttpResponse::Ok().body(BodyStream::new(new_search_playlist(
        session.instance().base_url(),
        session.instance().id(),
        token,
        session.bucket_arc(),
        query,
        100,
    )));

    Ok(response)
}

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[get("/{id}")]
pub async fn show(session: Session, id: web::Path<(u64, u64)>) -> Result<impl Responder, WebError> {
    let id = id.into_inner().1;

    let post = session
        .bucket()
        .data_source()
        .cross()
        .get_post_detail(id)
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    Ok(web::Json(post))
}

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[get("/{id}/index.m3u")]
pub async fn show_playlist(
    session: Session,
    id: web::Path<(u64, u64)>,
    params: web::Query<PlaylistParams>,
) -> Result<impl Responder, WebError> {
    let id = id.into_inner().1;

    let token = if params.include_token.unwrap_or(false) && session.read_only() {
        session.token().map(|s| s.to_string())
    } else {
        None
    };

    let post = session
        .bucket()
        .data_source()
        .cross()
        .get_post_detail(id)
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    let query = PostItemSearchQuery {
        require_playable: params.require_playable.unwrap_or_default(),
    };

    let response = HttpResponse::Ok().body(BodyStream::new(new_post_playlist(
        session.instance().base_url(),
        session.instance().id(),
        query,
        token,
        session.bucket_arc(),
        post,
        100,
    )));

    Ok(response)
}

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[get("/{id}/tags")]
pub async fn show_tags(
    session: Session,
    id: web::Path<(u64, u64)>,
) -> Result<impl Responder, WebError> {
    let id = id.into_inner().1;

    let post = session
        .bucket()
        .data_source()
        .posts()
        .get_by_id(id)
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    let tags = session
        .bucket()
        .data_source()
        .cross()
        .get_tags_from_post(post.id)
        .await?;

    Ok(web::Json(tags))
}

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[delete("/{id}")]
pub async fn delete(
    session: Session,
    id: web::Path<(u64, u64)>,
) -> Result<impl Responder, WebError> {
    let id = id.into_inner().1;

    session
        .bucket()
        .data_source()
        .cross()
        .cascade_delete_post(id)
        .await?;

    info!("Deleted post {id}");

    Ok(web::Json(()))
}

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[get("/{id}/items/{position}")]
pub async fn show_item(
    session: Session,
    path: web::Path<(u64, u64, i32)>,
) -> Result<impl Responder, WebError> {
    let (_, id, position) = path.into_inner();

    let item = session
        .bucket()
        .data_source()
        .cross()
        .get_full_post_item(id, position)
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    Ok(web::Json(item))
}

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[get("/{id}/items/{position}/index.m3u8")]
pub async fn show_item_playlist(
    session: Session,
    path: web::Path<(u64, u64, i32)>,
    params: web::Query<PlaylistParams>,
) -> Result<impl Responder, WebError> {
    let (_, id, position) = path.into_inner();

    let token = if params.include_token.unwrap_or(false) && session.read_only() {
        session.token().map(|s| s.to_string())
    } else {
        None
    };

    let item = session
        .bucket()
        .data_source()
        .cross()
        .get_full_post_item(id, position)
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    if let crate::model::ManyToOne::Obj(c) = &item.content {
        if let crate::model::ManyToOne::Obj(media) = &c.content {
            if media.metadata.duration().is_none() {
                return Err(WebError::ResourceNotFound);
            }
        }
    }

    let response = HttpResponse::Ok().body(BodyStream::new(new_content_playlist(
        session.instance().base_url(),
        session.instance().id(),
        token,
        session.bucket_arc(),
        item,
    )));

    Ok(response)
}

#[derive(Deserialize)]
pub struct PostItemsQueryParams {
    require_playable: Option<bool>,
}

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[get("/{id}/items")]
pub async fn index_items(
    session: Session,
    id: web::Path<(u64, u64)>,
    query: web::Query<PostItemsQueryParams>,
    page: PageParams,
) -> Result<impl Responder, WebError> {
    let id = id.into_inner().1;

    let post = session
        .bucket()
        .data_source()
        .posts()
        .get_by_id(id)
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    let query = PostItemSearchQuery {
        require_playable: query.require_playable.unwrap_or_default(),
    };

    let items = session
        .bucket()
        .data_source()
        .cross()
        .search_items(post.id, &query, page)
        .await?;

    Ok(web::Json(items))
}

#[derive(Deserialize)]
pub struct CreatePostTagRequest {
    tag_id: u64,
    enable: bool,
}
#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[post("/{id}/tags")]
pub async fn store_tags(
    id: web::Path<(u64, u64)>,
    session: Session,
    req: web::Json<CreatePostTagRequest>,
) -> Result<impl Responder, WebError> {
    let id = id.into_inner().1;

    if req.enable {
        session
            .bucket()
            .data_source()
            .tags()
            .add_tag_to_post(req.tag_id, id)
            .await?;

        info!("Added tag {} to post {}", req.tag_id, id);
    } else {
        session
            .bucket()
            .data_source()
            .tags()
            .remove_tag_to_post(req.tag_id, id)
            .await?;

        info!("Removed tag {} from post {}", req.tag_id, id);
    }

    Ok(web::Json(()))
}

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
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
#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[put("/{id}")]
pub async fn update(
    session: Session,
    id: web::Path<(u64, u64)>,
    req: web::Json<UpdatePostRequest>,
) -> Result<impl Responder, WebError> {
    let mut post = session
        .bucket()
        .data_source()
        .posts()
        .get_by_id(id.into_inner().1)
        .await?
        .ok_or(WebError::ResourceNotFound)?;

    post.title = req.title.clone();
    post.description = req.description.clone();
    post.source = req.source.clone();

    session
        .bucket()
        .data_source()
        .cross()
        .update_full_post(&post, &req.tag_ids)
        .await?;

    Ok(web::Json(post))
}
