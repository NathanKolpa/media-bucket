use super::PLAYLIST_HEADER;

use actix_web::web::Bytes;
use futures_core::Stream;
use pin_project_lite::pin_project;
use std::fmt::Write;
use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use url::Url;

pub struct MediaEntry {
    pub media_id: u64,
    pub title: Option<String>,
    pub thumbnail_id: Option<u64>,
    pub runtime_seconds: i32,
    pub resolution: Option<(i32, i32)>,
    pub size: usize,
}

pin_project! {
    pub struct MediaPlaylist<Fut> {
        pub bucket_id: u64,
        pub base_url: Option<Arc<Url>>,
        pub token: Option<String>,

        #[pin]
        pub fut: Fut,

        pub done: bool
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

        let mut buffer = String::from(PLAYLIST_HEADER);

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
