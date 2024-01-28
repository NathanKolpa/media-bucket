use crate::data_source::PageParams;
use crate::model::PostSearchQuery;
use crate::Bucket;
use actix_web::web::Bytes;
use futures_core::{ready, Stream};
use pin_project_lite::pin_project;
use std::collections::VecDeque;
use std::future::Future;
use std::io::{Error, ErrorKind};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

pub fn new_search_playlist(
    bucket: Arc<Bucket>,
    query: PostSearchQuery,
    chunk_size: usize,
) -> impl Stream<Item = Result<Bytes, Error>> {
    let query_rc = Arc::new(query);

    StreamPlaylist {
        bucket: Some(bucket),
        entries: Some(VecDeque::with_capacity(chunk_size)),
        header_written: false,
        next_page: PageParams::new(chunk_size, 0),
        state: StreamPlaylistState::PreRead,
        current_write_state: EntryWriteState::Header,
        next_page_callback: move |bucket, params, buffer| {
            let query = query_rc.clone();
            search_posts(query, bucket, params, buffer)
        },
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
        title: p.post.title,
        thumbnail: p.thumbnail.map(|t| format!("/media/{}", t.file_id)),
        url: format!("/posts/{}/index.m3u", p.post.id),
    }));

    Ok((bucket, buffer))
}

struct PlaylistEntry {
    url: String,
    thumbnail: Option<String>,
    title: Option<String>,
}

pin_project! {
    struct StreamPlaylist<N, Fut> {
        next_page_callback: N,
        next_page: PageParams,
        header_written: bool,

        bucket: Option<Arc<Bucket>>,
        entries: Option<VecDeque<PlaylistEntry>>,
        current_write_state: EntryWriteState,

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

enum EntryWriteState {
    Header,

    ThumbnailPre,
    Thumbnail,
    ThumbnailPost,

    TitlePre,
    Title,

    HeaderEnd,

    Url,
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

                let Some(entry) = entries.front_mut() else {
                    let bucket = this.bucket.take().unwrap();
                    let entries = this.entries.take().unwrap();

                    let fut = (this.next_page_callback)(bucket, *this.next_page, entries);

                    this.state
                        .project_replace(StreamPlaylistState::Read { fut });

                    return self.poll_next(cx);
                };

                match this.current_write_state {
                    EntryWriteState::Header => {
                        if entry.thumbnail.is_some() {
                            *this.current_write_state = EntryWriteState::ThumbnailPre;
                        } else if entry.title.is_some() {
                            *this.current_write_state = EntryWriteState::TitlePre;
                        } else {
                            *this.current_write_state = EntryWriteState::HeaderEnd;
                        }

                        Poll::Ready(Some(Ok(Bytes::from_static("\n\n#EXTINF:-1".as_bytes()))))
                    }
                    EntryWriteState::ThumbnailPre => {
                        *this.current_write_state = EntryWriteState::Thumbnail;
                        Poll::Ready(Some(Ok(Bytes::from_static(" tvg-logo=\"".as_bytes()))))
                    }
                    EntryWriteState::Thumbnail => {
                        *this.current_write_state = EntryWriteState::ThumbnailPost;
                        let url: String = entry.thumbnail.take().unwrap().into();
                        Poll::Ready(Some(Ok(Bytes::from(url))))
                    }
                    EntryWriteState::ThumbnailPost => {
                        if entry.title.is_some() {
                            *this.current_write_state = EntryWriteState::TitlePre;
                        } else {
                            *this.current_write_state = EntryWriteState::HeaderEnd;
                        }

                        Poll::Ready(Some(Ok(Bytes::from_static("\"".as_bytes()))))
                    }
                    EntryWriteState::TitlePre => {
                        *this.current_write_state = EntryWriteState::Title;
                        Poll::Ready(Some(Ok(Bytes::from_static(", ".as_bytes()))))
                    }

                    EntryWriteState::Title => {
                        *this.current_write_state = EntryWriteState::HeaderEnd;
                        let title = entry.title.take().unwrap();
                        Poll::Ready(Some(Ok(Bytes::from(title))))
                    }

                    EntryWriteState::HeaderEnd => {
                        *this.current_write_state = EntryWriteState::Url;
                        Poll::Ready(Some(Ok(Bytes::from_static("\n".as_bytes()))))
                    }
                    EntryWriteState::Url => {
                        *this.current_write_state = EntryWriteState::Header;
                        let url: String = this
                            .entries
                            .as_mut()
                            .unwrap()
                            .pop_front()
                            .unwrap()
                            .url
                            .into();
                        Poll::Ready(Some(Ok(Bytes::from(url))))
                    }
                }
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
