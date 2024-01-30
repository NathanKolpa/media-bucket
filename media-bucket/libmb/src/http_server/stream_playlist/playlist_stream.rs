use crate::data_source::PageParams;

use super::PLAYLIST_HEADER;

use crate::Bucket;
use actix_web::web::Bytes;
use futures_core::{ready, Stream};
use pin_project_lite::pin_project;
use std::collections::VecDeque;
use std::fmt::Write;
use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use url::Url;

pub enum EntryUrl {
    Post(u64),
    Item(u64, i32),
}

pub struct PlaylistEntry {
    pub url: EntryUrl,
    pub thumbnail_file: Option<u64>,
    pub title: Option<String>,
    pub runtime_seconds: i32,
    pub file_size: Option<usize>,
}

pin_project! {
    pub struct PlaylistStream<N, Fut> {
        pub next_page_callback: N,
        pub next_page: PageParams,
        pub header_written: bool,
        pub playlist_title: Option<String>,

        pub bucket: Option<Arc<Bucket>>,
        pub entries: Option<VecDeque<PlaylistEntry>>,
        pub buffer: String,

        pub bucket_id: u64,
        pub base_url: Option<Arc<Url>>,
        pub token: Option<String>,

        #[pin]
        pub state: StreamPlaylistState<Fut>,
    }
}

pin_project! {
    #[project = StreamPlaylistProj]
    #[project_replace = StreamPlaylistProjReplace]
    pub enum StreamPlaylistState<Fut> {
        PreRead,
        Read { #[pin] fut: Fut },
    }
}

impl<N, Fut> Stream for PlaylistStream<N, Fut>
where
    N: Fn(Arc<Bucket>, PageParams, VecDeque<PlaylistEntry>) -> Fut,
    Fut: Future<Output = Result<(Arc<Bucket>, VecDeque<PlaylistEntry>), Error>>,
{
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut().project();

        if !*this.header_written {
            let mut header = String::from(PLAYLIST_HEADER);

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
