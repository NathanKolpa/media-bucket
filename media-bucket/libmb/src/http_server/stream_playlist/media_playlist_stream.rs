use super::PLAYLIST_HEADER;

use actix_web::web::Bytes;
use futures_core::Stream;
use pin_project_lite::pin_project;
use std::fmt::Write;
use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};

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
        pub api_url: super::api_urls::ApiUrl,
        pub auth_params: super::api_urls::AuthParams,

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
        let api_url = &*this.api_url;

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
                "\r\n#EXTIMG:{api_url}/media/{thumbnail_id}/file{}",
                this.auth_params.without_include()
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
            "\r\n{api_url}/media/{}/file{}",
            media.media_id,
            this.auth_params.without_include()
        )
        .unwrap();

        Poll::Ready(Some(Ok(Bytes::from(buffer))))
    }
}
