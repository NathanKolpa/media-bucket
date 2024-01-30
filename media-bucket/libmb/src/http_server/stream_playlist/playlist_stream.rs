use crate::data_source::PageParams;

use super::PLAYLIST_HEADER;

use actix_web::web::Bytes;
use futures_core::{ready, Stream};
use pin_project_lite::pin_project;
use std::collections::VecDeque;
use std::fmt::Write;
use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};

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

        pub api_url: super::api_urls::ApiUrl,
        pub auth_params: super::api_urls::AuthParams,

        pub entries: Option<VecDeque<PlaylistEntry>>,
        pub buffer: String,


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
    N: Fn(PageParams, VecDeque<PlaylistEntry>) -> Fut,
    Fut: Future<Output = Result<VecDeque<PlaylistEntry>, Error>>,
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
                    let entries = this.entries.take().unwrap();

                    let fut = (this.next_page_callback)(*this.next_page, entries);

                    this.state
                        .project_replace(StreamPlaylistState::Read { fut });

                    return self.poll_next(cx);
                }

                this.buffer.clear();
                let api_url = &*this.api_url;

                for entry in entries.iter().filter(|x| x.runtime_seconds.is_positive()) {
                    write!(this.buffer, "\r\n\r\n#EXTINF:{}", entry.runtime_seconds,).unwrap();

                    if let Some(thumbnail_id) = entry.thumbnail_file {
                        write!(
                            this.buffer,
                            " tvg-logo=\"{api_url}/media/{thumbnail_id}/file{}\"",
                            this.auth_params.without_include()
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
                            "#EXTIMG:{api_url}/media/{thumbnail_id}/file{}\r\n",
                            this.auth_params.without_include()
                        )
                        .unwrap();
                        write!(
                            this.buffer,
                            "#EXTALBUMARTURL:{api_url}/media/{thumbnail_id}/file{}\r\n",
                            this.auth_params.without_include()
                        )
                        .unwrap();
                    }

                    match &entry.url {
                        EntryUrl::Post(post_id) => write!(
                            this.buffer,
                            "{api_url}/posts/{post_id}/index.m3u{}\r\n",
                            this.auth_params.include_token()
                        )
                        .unwrap(),
                        EntryUrl::Item(post_id, position) => write!(
                            this.buffer,
                            "{api_url}/posts/{post_id}/items/{position}/index.m3u8{}\r\n",
                            this.auth_params.include_token()
                        )
                        .unwrap(),
                    }
                }

                entries.clear();

                Poll::Ready(Some(Ok(Bytes::copy_from_slice(this.buffer.as_bytes()))))
            }
            StreamPlaylistProj::Read { fut } => {
                let entries = ready!(fut.poll(cx))?;

                if entries.is_empty() {
                    return Poll::Ready(None);
                }

                *this.entries = Some(entries);

                this.state.project_replace(StreamPlaylistState::PreRead);

                *this.next_page = this.next_page.next();

                self.poll_next(cx)
            }
        }
    }
}
