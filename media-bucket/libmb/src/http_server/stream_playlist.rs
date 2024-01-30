mod api_urls;
mod media_playlist_stream;
mod playlist_stream;

use crate::{
    data_source::{DataSourceError, PageParams},
    model::{PostDetail, PostItem},
};

use api_urls::*;
use media_playlist_stream::*;
use playlist_stream::*;

use crate::model::PostSearchQuery;
use crate::Bucket;
use actix_web::web::Bytes;
use futures_core::Stream;
use std::sync::Arc;
use std::{
    collections::VecDeque,
    io::{Error, ErrorKind},
};
use url::Url;

fn data_to_std_err(err: DataSourceError) -> Error {
    Error::new(ErrorKind::Other, Box::new(err))
}

pub fn new_post_playlist(
    base: Option<Arc<Url>>,
    bucket_id: u64,
    token: Option<String>,
    bucket: Arc<Bucket>,
    post: PostDetail,
    chunk_size: usize,
) -> impl Stream<Item = Result<Bytes, Error>> {
    let post_rc = Arc::new(post);
    PlaylistStream {
        playlist_title: post_rc.post.title.clone(),
        api_url: ApiUrl { bucket_id, base },
        auth_params: AuthParams { token },
        entries: Some(VecDeque::with_capacity(chunk_size)),
        header_written: false,
        next_page: PageParams::new(chunk_size, 0),
        state: StreamPlaylistState::PreRead,
        next_page_callback: move |params, buffer| {
            search_items(post_rc.clone(), bucket.clone(), params, buffer)
        },
        buffer: String::new(),
    }
}

async fn search_items(
    post: Arc<PostDetail>,
    bucket: Arc<Bucket>,
    params: PageParams,
    mut buffer: VecDeque<PlaylistEntry>,
) -> Result<VecDeque<PlaylistEntry>, Error> {
    let mut item_title = None;
    if post.item_count == 1 {
        item_title = post.post.title.clone();
    }

    let page = bucket
        .data_source()
        .cross()
        .search_items(post.post.id, params)
        .await
        .map_err(data_to_std_err)?;

    buffer.extend(page.data.into_iter().map(|item| {
        let media = item
            .item
            .content
            .as_ref()
            .obj()
            .and_then(|content| content.content.as_ref().obj());

        PlaylistEntry {
            url: EntryUrl::Item(item.item.post.id(), item.item.position),
            title: item_title.clone().or(item
                .item
                .upload
                .original_filename
                .or_else(|| post.post.title.clone())),
            file_size: media.map(|media| media.file_size),
            thumbnail_file: item.thumbnail.map(|t| t.id),
            runtime_seconds: item.duration.unwrap_or(-1),
        }
    }));

    Ok(buffer)
}

pub fn new_content_playlist(
    base: Option<Arc<Url>>,
    bucket_id: u64,
    token: Option<String>,
    bucket: Arc<Bucket>,
    item: PostItem,
) -> impl Stream<Item = Result<Bytes, Error>> {
    MediaPlaylist {
        api_url: ApiUrl { bucket_id, base },
        auth_params: AuthParams { token },
        done: false,
        fut: get_content(item, bucket),
    }
}

async fn get_content(item: PostItem, bucket: Arc<Bucket>) -> Result<MediaEntry, Error> {
    let content = match item.content {
        crate::model::ManyToOne::Obj(o) => Some(o),
        crate::model::ManyToOne::Id(id) => bucket
            .data_source()
            .content()
            .get_by_content_id(id)
            .await
            .map_err(data_to_std_err)?,
    };

    let Some(content) = content else {
        return Err(Error::from(ErrorKind::NotFound));
    };

    let thumbnail = match content.thumbnail {
        crate::model::ManyToOne::Obj(o) => Some(o),
        crate::model::ManyToOne::Id(id) => bucket
            .data_source()
            .media()
            .get_by_id(id)
            .await
            .map_err(data_to_std_err)?,
    };

    let media = match content.content {
        crate::model::ManyToOne::Obj(o) => Some(o),
        crate::model::ManyToOne::Id(id) => bucket
            .data_source()
            .media()
            .get_by_id(id)
            .await
            .map_err(data_to_std_err)?,
    };

    let Some(media) = media else {
        return Err(Error::from(ErrorKind::NotFound));
    };

    Ok(MediaEntry {
        media_id: media.id,
        thumbnail_id: thumbnail.map(|x| x.id),
        title: item
            .post
            .as_ref()
            .obj()
            .and_then(|post| post.title.clone())
            .or(item.upload.original_filename),
        runtime_seconds: media.metadata.duration().cloned().unwrap_or(-1),
        resolution: media
            .metadata
            .width()
            .and_then(|width| media.metadata.height().map(|height| (*width, *height))),
        size: media.file_size,
    })
}

pub fn new_search_playlist(
    base: Option<Arc<Url>>,
    bucket_id: u64,
    token: Option<String>,
    bucket: Arc<Bucket>,
    query: PostSearchQuery,
    chunk_size: usize,
) -> impl Stream<Item = Result<Bytes, Error>> {
    let query = Arc::new(query);

    PlaylistStream {
        playlist_title: query.text.clone(),
        entries: Some(VecDeque::with_capacity(chunk_size)),
        header_written: false,
        next_page: PageParams::new(chunk_size, 0),
        state: StreamPlaylistState::PreRead,
        next_page_callback: move |params, buffer| {
            search_posts(query.clone(), bucket.clone(), params, buffer)
        },
        buffer: String::new(),
        api_url: ApiUrl { bucket_id, base },
        auth_params: AuthParams { token },
    }
}

async fn search_posts(
    query: Arc<PostSearchQuery>,
    bucket: Arc<Bucket>,
    params: PageParams,
    mut buffer: VecDeque<PlaylistEntry>,
) -> Result<VecDeque<PlaylistEntry>, Error> {
    let results = bucket
        .data_source()
        .cross()
        .search_posts(&query, &params)
        .await;

    let page = match results {
        Ok(p) => p,
        Err(err) => return Err(data_to_std_err(err)),
    };

    buffer.extend(page.data.into_iter().map(|p| PlaylistEntry {
        url: EntryUrl::Post(p.post.id),
        file_size: None,
        title: p.post.title.or(p.file_name),
        thumbnail_file: p.thumbnail.map(|t| t.id),
        runtime_seconds: p.duration.unwrap_or(-1),
    }));

    Ok(buffer)
}

const PLAYLIST_HEADER: &str = "#EXTM3U\r\n#EXTENC:UTF-8";
