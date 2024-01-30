use crate::{
    data_source::PageParams,
    model::{PostDetail, PostItem},
};

use crate::model::PostSearchQuery;
use crate::Bucket;
use actix_web::web::Bytes;
use futures_core::{ready, Stream};
use pin_project_lite::pin_project;
use std::collections::VecDeque;
use std::fmt::Write;
use std::future::Future;
use std::io::{Error, ErrorKind};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use url::Url;

pub fn new_post_playlist(
    base_url: Option<Arc<Url>>,
    bucket_id: u64,
    token: Option<String>,
    bucket: Arc<Bucket>,
    post: PostDetail,
    chunk_size: usize,
) -> impl Stream<Item = Result<Bytes, Error>> {
    StreamPlaylist {
        playlist_title: post.post.title,
        base_url,
        bucket: Some(bucket),
        entries: Some(VecDeque::with_capacity(chunk_size)),
        header_written: false,
        next_page: PageParams::new(chunk_size, 0),
        state: StreamPlaylistState::PreRead,
        next_page_callback: move |bucket, params, buffer| {
            search_items(post.post.id, post.item_count, bucket, params, buffer)
        },
        buffer: String::new(),
        bucket_id,
        token,
    }
}

async fn search_items(
    post_id: u64,
    _item_count: usize,
    bucket: Arc<Bucket>,
    params: PageParams,
    mut buffer: VecDeque<PlaylistEntry>,
) -> Result<(Arc<Bucket>, VecDeque<PlaylistEntry>), Error> {
    let post = bucket
        .data_source()
        .cross()
        .get_post_detail(post_id)
        .await
        .map_err(|err| Error::new(ErrorKind::Other, Box::new(err)))?
        .ok_or(Error::from(ErrorKind::NotFound))?;

    let mut item_title = None;
    if post.item_count == 1 {
        item_title = post.post.title.clone();
    }

    let page = bucket
        .data_source()
        .cross()
        .search_items(post_id, params)
        .await
        .map_err(|err| Error::new(ErrorKind::Other, Box::new(err)))?;

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

    Ok((bucket, buffer))
}

pub fn new_content_playlist(
    base_url: Option<Arc<Url>>,
    bucket_id: u64,
    token: Option<String>,
    bucket: Arc<Bucket>,
    item: PostItem,
) -> impl Stream<Item = Result<Bytes, Error>> {
    MediaPlaylist {
        base_url,
        bucket_id,
        token,
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
            .map_err(|err| Error::new(ErrorKind::Other, Box::new(err)))?,
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
            .map_err(|err| Error::new(ErrorKind::Other, Box::new(err)))?,
    };

    let media = match content.content {
        crate::model::ManyToOne::Obj(o) => Some(o),
        crate::model::ManyToOne::Id(id) => bucket
            .data_source()
            .media()
            .get_by_id(id)
            .await
            .map_err(|err| Error::new(ErrorKind::Other, Box::new(err)))?,
    };

    let Some(media) = media else {
        return Err(Error::from(ErrorKind::NotFound));
    };

    Ok(MediaEntry {
        media_id: media.id,
        thumbnail_id: thumbnail.map(|x| x.id),
        title: item.upload.original_filename,
        runtime_seconds: media.metadata.duration().cloned().unwrap_or(-1),
        resolution: media
            .metadata
            .width()
            .and_then(|width| media.metadata.height().map(|height| (*width, *height))),
        size: media.file_size,
    })
}

pub fn new_search_playlist(
    base_url: Option<Arc<Url>>,
    bucket_id: u64,
    token: Option<String>,
    bucket: Arc<Bucket>,
    query: PostSearchQuery,
    chunk_size: usize,
) -> impl Stream<Item = Result<Bytes, Error>> {
    let query_rc = Arc::new(query);

    StreamPlaylist {
        playlist_title: query_rc.text.clone(),
        base_url,
        bucket: Some(bucket),
        entries: Some(VecDeque::with_capacity(chunk_size)),
        header_written: false,
        next_page: PageParams::new(chunk_size, 0),
        state: StreamPlaylistState::PreRead,
        next_page_callback: move |bucket, params, buffer| {
            let query = query_rc.clone();
            search_posts(query, bucket, params, buffer)
        },
        buffer: String::new(),
        bucket_id,
        token,
    }
}

async fn search_posts(
    query: Arc<PostSearchQuery>,
    bucket: Arc<Bucket>,
    params: PageParams,
    mut buffer: VecDeque<PlaylistEntry>,
) -> Result<(Arc<Bucket>, VecDeque<PlaylistEntry>), Error> {
    let results = bucket
        .data_source()
        .cross()
        .search_posts(&query, &params)
        .await;

    let page = match results {
        Ok(p) => p,
        Err(err) => return Err(Error::new(ErrorKind::Other, Box::new(err))),
    };

    buffer.extend(page.data.into_iter().map(|p| PlaylistEntry {
        url: EntryUrl::Post(p.post.id),
        file_size: None,
        title: p.post.title.or(p.file_name),
        thumbnail_file: p.thumbnail.map(|t| t.id),
        runtime_seconds: p.duration.unwrap_or(-1),
    }));

    Ok((bucket, buffer))
}

enum EntryUrl {
    Post(u64),
    Item(u64, i32),
}

struct MediaEntry {
    media_id: u64,
    title: Option<String>,
    thumbnail_id: Option<u64>,
    runtime_seconds: i32,
    resolution: Option<(i32, i32)>,
    size: usize,
}

pin_project! {
    struct MediaPlaylist<Fut> {
        bucket_id: u64,
        base_url: Option<Arc<Url>>,
        token: Option<String>,

        #[pin]
        fut: Fut,

        done: bool
    }
}

impl<Fut> Stream for MediaPlaylist<Fut>
where
    Fut: Future<Output = Result<MediaEntry, Error>>,
{
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.done {
            return Poll::Ready(None);
        }

        let this = self.as_mut().project();

        let Poll::Ready(media) = this.fut.poll(cx) else {
            return Poll::Pending;
        };

        *this.done = true;

        let media = match media {
            Ok(media) => media,
            Err(err) => return Poll::Ready(Some(Err(err))),
        };

        let mut buffer = String::from(HEADER);

        let token_str = this
            .token
            .as_ref()
            .map(|t| format!("?token={}", t))
            .unwrap_or_default();

        let base_url_str = this.base_url.as_ref().map(|x| x.as_str()).unwrap_or(".");

        if let Some(title) = media.title.as_deref() {
            write!(&mut buffer, "\r\n#PLAYLIST:{}", title).unwrap();
        }

        write!(&mut buffer, "\r\n#EXTBYT:{}", media.size).unwrap();

        if let Some(title) = media.title.as_deref() {
            write!(
                &mut buffer,
                "\r\n#EXTINF:{},{}",
                media.runtime_seconds, title
            )
            .unwrap();
        }

        if let Some(thumbnail_id) = media.thumbnail_id {
            write!(
                &mut buffer,
                "\r\n#EXTIMG:{}buckets/{}/media/{}/file{}",
                base_url_str, *this.bucket_id, thumbnail_id, token_str
            )
            .unwrap();
        }

        if let Some((width, height)) = media.resolution {
            write!(
                &mut buffer,
                "\r\n#EXT-STREAM-INF:RESOLUTION=\"{width}x{height}\",NAME=\"original\"",
            )
            .unwrap();
        }

        write!(
            &mut buffer,
            "\r\n{}buckets/{}/media/{}/file{}",
            base_url_str, *this.bucket_id, media.media_id, token_str
        )
        .unwrap();

        Poll::Ready(Some(Ok(Bytes::from(buffer))))
    }
}

struct PlaylistEntry {
    url: EntryUrl,
    thumbnail_file: Option<u64>,
    title: Option<String>,
    runtime_seconds: i32,
    file_size: Option<usize>,
}

pin_project! {
    struct StreamPlaylist<N, Fut> {
        next_page_callback: N,
        next_page: PageParams,
        header_written: bool,
        playlist_title: Option<String>,

        bucket: Option<Arc<Bucket>>,
        entries: Option<VecDeque<PlaylistEntry>>,
        buffer: String,

        bucket_id: u64,
        base_url: Option<Arc<Url>>,
        token: Option<String>,

        #[pin]
        state: StreamPlaylistState<Fut>,
    }
}

pin_project! {
    #[project = StreamPlaylistProj]
    #[project_replace = StreamPlaylistProjReplace]
    enum StreamPlaylistState<Fut> {
        PreRead,
        Read { #[pin] fut: Fut },
    }
}

const HEADER: &str = "#EXTM3U\r\n#EXTENC:UTF-8";

impl<N, Fut> Stream for StreamPlaylist<N, Fut>
where
    N: Fn(Arc<Bucket>, PageParams, VecDeque<PlaylistEntry>) -> Fut,
    Fut: Future<Output = Result<(Arc<Bucket>, VecDeque<PlaylistEntry>), Error>>,
{
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut().project();

        if !*this.header_written {
            let mut header = String::from(HEADER);

            if let Some(title) = this.playlist_title {
                write!(&mut header, "\r\n#PLAYLIST:{}", title).unwrap();
            }

            write!(&mut header, "\r\n#EXTM3A").unwrap();

            *this.header_written = true;
            return Poll::Ready(Some(Ok(Bytes::from(header))));
        }

        match this.state.as_mut().project() {
            StreamPlaylistProj::PreRead => {
                let entries = this.entries.as_mut().unwrap();

                if entries.is_empty() {
                    let bucket = this.bucket.take().unwrap();
                    let entries = this.entries.take().unwrap();

                    let fut = (this.next_page_callback)(bucket, *this.next_page, entries);

                    this.state
                        .project_replace(StreamPlaylistState::Read { fut });

                    return self.poll_next(cx);
                }

                this.buffer.clear();
                let token_str = this
                    .token
                    .as_ref()
                    .map(|t| format!("?token={}", t))
                    .unwrap_or_default();

                let base_url_str = this.base_url.as_ref().map(|x| x.as_str()).unwrap_or(".");

                let include_str = if this.token.is_some() {
                    "&include_token=true"
                } else {
                    ""
                };

                for entry in entries.iter().filter(|x| x.runtime_seconds.is_positive()) {
                    write!(this.buffer, "\r\n\r\n#EXTINF:{}", entry.runtime_seconds,).unwrap();

                    if let Some(thumbnail_id) = entry.thumbnail_file {
                        write!(
                            this.buffer,
                            " tvg-logo=\"{}buckets/{}/media/{}/file{}\"",
                            base_url_str, *this.bucket_id, thumbnail_id, token_str
                        )
                        .unwrap();
                    }

                    if let Some(title) = entry.title.as_deref() {
                        write!(this.buffer, ", {title}").unwrap();
                    }

                    write!(this.buffer, "\r\n").unwrap();

                    if let Some(size) = entry.file_size {
                        write!(this.buffer, "#EXTBYT:{size}\r\n").unwrap();
                    }

                    if let Some(thumbnail_id) = entry.thumbnail_file {
                        write!(
                            this.buffer,
                            "#EXTIMG:{}buckets/{}/media/{}/file{}\r\n",
                            base_url_str, *this.bucket_id, thumbnail_id, token_str
                        )
                        .unwrap();
                        write!(
                            this.buffer,
                            "#EXTALBUMARTURL:{}buckets/{}/media/{}/file{}\r\n",
                            base_url_str, *this.bucket_id, thumbnail_id, token_str
                        )
                        .unwrap();
                    }

                    match &entry.url {
                        EntryUrl::Post(post_id) => write!(
                            this.buffer,
                            "{}buckets/{}/posts/{}/index.m3u{}{}\r\n",
                            base_url_str, *this.bucket_id, post_id, token_str, include_str
                        )
                        .unwrap(),
                        EntryUrl::Item(post_id, position) => write!(
                            this.buffer,
                            "{}buckets/{}/posts/{}/items/{}/index.m3u8{}{}\r\n",
                            base_url_str,
                            *this.bucket_id,
                            post_id,
                            position,
                            token_str,
                            include_str
                        )
                        .unwrap(),
                    }
                }

                entries.clear();

                Poll::Ready(Some(Ok(Bytes::copy_from_slice(this.buffer.as_bytes()))))
            }
            StreamPlaylistProj::Read { fut } => {
                let (bucket, entries) = ready!(fut.poll(cx))?;

                if entries.is_empty() {
                    return Poll::Ready(None);
                }

                *this.entries = Some(entries);
                *this.bucket = Some(bucket);

                this.state.project_replace(StreamPlaylistState::PreRead);

                *this.next_page = this.next_page.next();

                self.poll_next(cx)
            }
        }
    }
}
