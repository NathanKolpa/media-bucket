use crate::data_source::PageParams;

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
        title: p.post.title,
        thumbnail_file: p.thumbnail.map(|t| t.id),
        runtime_seconds: p.duration.unwrap_or(-1),
    }));

    Ok((bucket, buffer))
}

enum EntryUrl {
    Post(u64),
}

struct PlaylistEntry {
    url: EntryUrl,
    thumbnail_file: Option<u64>,
    title: Option<String>,
    runtime_seconds: i32,
}

pin_project! {
    struct StreamPlaylist<N, Fut> {
        next_page_callback: N,
        next_page: PageParams,
        header_written: bool,

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

impl<N, Fut> Stream for StreamPlaylist<N, Fut>
where
    N: Fn(Arc<Bucket>, PageParams, VecDeque<PlaylistEntry>) -> Fut,
    Fut: Future<Output = Result<(Arc<Bucket>, VecDeque<PlaylistEntry>), Error>>,
{
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut().project();

        if !*this.header_written {
            *this.header_written = true;
            return Poll::Ready(Some(Ok(Bytes::from_static("#EXTM3U".as_bytes()))));
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
                    write!(this.buffer, "\n\n#EXTINF:{}", entry.runtime_seconds,).unwrap();

                    if let Some(thumbnail_id) = entry.thumbnail_file.as_ref() {
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

                    writeln!(this.buffer).unwrap();

                    match entry.url {
                        EntryUrl::Post(post_id) => writeln!(
                            this.buffer,
                            "{}buckets/{}/posts/{}/index.m3u{}{}",
                            base_url_str, *this.bucket_id, post_id, token_str, include_str
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
