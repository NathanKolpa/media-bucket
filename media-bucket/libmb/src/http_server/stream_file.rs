use std::io::Error;
use std::{
    cmp,
    future::Future,
    io,
    pin::Pin,
    task::{Context, Poll},
};

use actix_web::web::Bytes;
use futures_core::{ready, Stream};
use pin_project_lite::pin_project;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use crate::data_source::FileOutput;

pin_project! {
    pub struct ChunkedReadFile<F, Fut> {
        size: u64,
        offset: u64,
        #[pin]
        state: ChunkedReadFileState<Fut>,
        counter: u64,
        callback: F,
    }
}

pin_project! {
    #[project = ChunkedReadFileStateProj]
    #[project_replace = ChunkedReadFileStateProjReplace]
    enum ChunkedReadFileState<Fut> {
        File { file: Option<Pin<Box<dyn FileOutput>>>, },
        Future { #[pin] fut: Fut },
    }
}

pub fn new_chunked_read(
    size: u64,
    offset: u64,
    file: Pin<Box<dyn FileOutput>>,
) -> impl Stream<Item = Result<Bytes, Error>> {
    ChunkedReadFile {
        size,
        offset,
        state: ChunkedReadFileState::File { file: Some(file) },
        counter: 0,
        callback: chunked_read_file_callback,
    }
}

async fn chunked_read_file_callback(
    mut file: Pin<Box<dyn FileOutput>>,
    offset: u64,
    max_bytes: usize,
) -> Result<(Pin<Box<dyn FileOutput>>, Bytes), Error> {
    let mut buf = Vec::with_capacity(max_bytes);

    file.seek(io::SeekFrom::Start(offset)).await?;

    let n_bytes = file
        .as_mut()
        .take(max_bytes as u64)
        .read_to_end(&mut buf)
        .await?;

    if n_bytes == 0 {
        Err(io::Error::from(io::ErrorKind::UnexpectedEof))
    } else {
        Ok((file, Bytes::from(buf)))
    }
}

impl<F, Fut> Stream for ChunkedReadFile<F, Fut>
where
    F: Fn(Pin<Box<dyn FileOutput>>, u64, usize) -> Fut,
    Fut: Future<Output = Result<(Pin<Box<dyn FileOutput>>, Bytes), Error>>,
{
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut().project();
        match this.state.as_mut().project() {
            ChunkedReadFileStateProj::File { file } => {
                let size = *this.size;
                let offset = *this.offset;
                let counter = *this.counter;

                if size == counter {
                    Poll::Ready(None)
                } else {
                    let max_bytes = cmp::min(size.saturating_sub(counter), 65_536) as usize;

                    let file = file
                        .take()
                        .expect("ChunkedReadFile polled after completion");

                    let fut = (this.callback)(file, offset, max_bytes);

                    this.state
                        .project_replace(ChunkedReadFileState::Future { fut });

                    self.poll_next(cx)
                }
            }
            ChunkedReadFileStateProj::Future { fut } => {
                let (file, bytes) = ready!(fut.poll(cx))?;

                this.state
                    .project_replace(ChunkedReadFileState::File { file: Some(file) });

                *this.offset += bytes.len() as u64;
                *this.counter += bytes.len() as u64;

                Poll::Ready(Some(Ok(bytes)))
            }
        }
    }
}
