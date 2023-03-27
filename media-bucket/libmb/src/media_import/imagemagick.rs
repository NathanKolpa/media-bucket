use std::io::{Error, ErrorKind};
use std::path::Path;
use std::pin::Pin;
use std::process::Stdio;
use std::task::{Context, Poll};
use mediatype::MediaType;
use tokio::io::{AsyncRead, AsyncSeek, AsyncWrite, AsyncWriteExt, ReadBuf};
use tokio::process::{Child, ChildStdout, Command};
use crate::data_source::MediaImportError;
use crate::media_import::digest::{Digest, DigestWrite};
use crate::media_import::ImportInput;
use crate::model::{Dimensions, MediaMetadata};

const CONVERT_BIN: &str = "convert";
const IDENTIFY_BIN: &str = "identify";

pub struct ImageMagickThumbnailStreamOutput {
    stdout: ChildStdout,
}

pub struct ImageMagickThumbnailStream {
    process: Child,
}

impl ImageMagickThumbnailStream {
    fn new_from_input(input: &str, mime: MediaType, stdin: Stdio) -> Result<Self, MediaImportError> {
        let process = Command::new(CONVERT_BIN)
            .arg(input)
            .arg("-strip")
            .arg("-quality")
            .arg("50")
            .arg("-resize")
            .arg("300x300")
            .arg("-background")
            .arg("white")
            .arg("-alpha")
            .arg("remove")
            .arg("-alpha")
            .arg("off")
            .arg("jpg:-")
            .stderr(Stdio::inherit())
            .stdout(Stdio::piped())
            .stdin(stdin)
            .spawn()
            .map_err(|e| {
                match e.kind() {
                    ErrorKind::NotFound => MediaImportError::MissingProgram { name: CONVERT_BIN },
                    _ => MediaImportError::IOError(e)
                }
            })?;

        Ok(ImageMagickThumbnailStream { process })
    }

    pub fn new_from_file(path: &Path, mime: MediaType) -> Result<Self, MediaImportError> {
        Self::new_from_input(&format!("{}:{}[0]", mime.subty, path.display()), mime, Stdio::null())
    }

    pub fn new_from_stream(mime: MediaType) -> Result<Self, MediaImportError> {
        Self::new_from_input(&format!("{}:-[0]", mime.subty), mime, Stdio::piped())
    }
}

impl AsyncWrite for ImageMagickThumbnailStream {
    fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize, Error>> {
        Pin::new(self.process.stdin.as_mut().expect("No stdin")).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Pin::new(self.process.stdin.as_mut().expect("No stdin")).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Pin::new(self.process.stdin.as_mut().expect("No stdin")).poll_shutdown(cx)
    }
}

impl Digest for ImageMagickThumbnailStream {
    type Output = ImageMagickThumbnailStreamOutput;

    async fn digest(mut self) -> Result<Self::Output, MediaImportError> {
        let exit = self.process.wait().await?;

        if !exit.success() {
            return Err(MediaImportError::UnexpectedOutput);
        }

        Ok(ImageMagickThumbnailStreamOutput {
            stdout: self.process.stdout.expect("No stdout")
        })
    }
}

impl AsyncRead for ImageMagickThumbnailStreamOutput {
    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.stdout).poll_read(cx, buf)
    }
}

pub struct ImageMagickMetadata {
    process: Child,
}

impl ImageMagickMetadata {
    fn new_from_input(input: &str, mime: MediaType, stdin: Stdio) -> Result<Self, MediaImportError> {
        let process = Command::new(IDENTIFY_BIN)
            .arg("-ping")
            .arg("-format")
            .arg("%wx%h")
            .arg(input)
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .stdin(stdin)
            .spawn()
            .map_err(|e| {
                match e.kind() {
                    ErrorKind::NotFound => MediaImportError::MissingProgram { name: CONVERT_BIN },
                    _ => MediaImportError::IOError(e)
                }
            })?;

        Ok(ImageMagickMetadata { process })
    }

    pub fn new_from_file(path: &Path, mime: MediaType) -> Result<Self, MediaImportError> {
        Self::new_from_input(&format!("{}:{}[0]", mime.subty, path.display()), mime, Stdio::null())
    }

    pub fn new_from_stream(mime: MediaType) -> Result<Self, MediaImportError> {
        Self::new_from_input(&format!("{}:-[0]", mime.subty), mime, Stdio::piped())
    }
}

impl Digest for ImageMagickMetadata {
    type Output = MediaMetadata;

    async fn digest(self) -> Result<Self::Output, MediaImportError> {
        let output = self.process.wait_with_output().await?;

        if !output.status.success() {
            return Err(MediaImportError::UnexpectedOutput);
        }

        let output_str =
            String::from_utf8(output.stdout).map_err(|_| MediaImportError::UnexpectedOutput)?;

        let split: Vec<&str> = output_str.split('x').collect();

        if split.len() != 2 {
            return Err(MediaImportError::UnexpectedOutput);
        }

        Ok(MediaMetadata::Image {
            dims: Dimensions {
                width: split[0]
                    .trim()
                    .parse()
                    .map_err(|_| MediaImportError::UnexpectedOutput)?,
                height: split[1]
                    .trim()
                    .parse()
                    .map_err(|_| MediaImportError::UnexpectedOutput)?,
            }
        })
    }
}