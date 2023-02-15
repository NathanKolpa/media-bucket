use std::env;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use mediatype::{MediaType, MediaTypeBuf};
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::process::{Child, Command};
use uuid::Uuid;

use crate::data_source::MediaImportError;
use crate::model::{Dimensions, Media, MediaMetadata};

pub struct MediaImportOutput {
    pub content: Media,
    pub thumbnail: Media,
}

pub async fn import_file_with_thumbnail<O: AsyncWrite + Unpin>(
    file: &Path,
    mime: MediaTypeBuf,
    content_location: Uuid,
    content_output: O,
    thumbnail_location: Uuid,
    thumbnail_output: O,
) -> Result<MediaImportOutput, MediaImportError> {
    let thumbnail_mime = MediaTypeBuf::new(mediatype::names::IMAGE, mediatype::names::JPEG);

    let mut digest = ThumbnailWithMediaDigestable {
        media_digest: MediaDigestable {
            size_digest: Default::default(),
            sha256_digest: Default::default(),
            sha1_digest: Default::default(),
            md5_digest: Default::default(),
            metadata_digest: MetadataDigest::from_file(mime.to_ref(), file)?,
            mime: mime.clone(),
            location: content_location,
            output: content_output,
        },
        thumbnail_digest: ThumbnailDigestable {
            method: ThumbnailMethod::from_file(mime.to_ref(), file)?,
            media_digest: MediaDigestable {
                size_digest: Default::default(),
                sha256_digest: Default::default(),
                sha1_digest: Default::default(),
                md5_digest: Default::default(),
                metadata_digest: MetadataDigest::new_image_stream(thumbnail_mime.to_ref())?,
                mime: thumbnail_mime.clone(),
                location: thumbnail_location,
                output: thumbnail_output,
            },
        },
    };

    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .open(file)
        .await?;

    let mut buffer = [0_u8; 1024];

    loop {
        let size = file.read(&mut buffer).await?;

        if size == 0 {
            break;
        }

        digest.write(&buffer[0..size]).await?;
    }

    digest.digest().await
}

async fn get_video_codec(path: &Path) -> Result<String, MediaImportError> {
    let mut command = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-select_streams")
        .arg("v:0")
        .arg("-show_entries")
        .arg("stream=codec_name")
        .arg("-of")
        .arg("default=nokey=1:noprint_wrappers=1")
        .arg(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    command.wait().await?;

    let mut stdout = command
        .stdout
        .take()
        .ok_or(MediaImportError::UnexpectedOutput)?;
    let mut output = String::new();

    stdout.read_to_string(&mut output).await?;
    output.pop(); // remove '\n'

    Ok(output)
}

struct ThumbnailWithMediaDigestable<'a, O> {
    media_digest: MediaDigestable<'a, O>,
    thumbnail_digest: ThumbnailDigestable<'a, O>,
}

impl<'a, O> ThumbnailWithMediaDigestable<'a, O>
where
    O: AsyncWrite + Unpin,
{
    async fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.media_digest.write(data).await?;
        Ok(())
    }

    async fn digest(self) -> Result<MediaImportOutput, MediaImportError> {
        Ok(MediaImportOutput {
            content: self.media_digest.digest().await?,
            thumbnail: self.thumbnail_digest.digest().await?,
        })
    }
}

enum CompatMethod<'a> {
    Unoconv { path: &'a Path },
}

struct TmpFile {
    path: PathBuf,
}

impl TmpFile {
    async fn new() -> Result<Self, MediaImportError> {
        let tmp_file_path = env::temp_dir().join(Uuid::new_v4().to_string());

        Ok(Self {
            path: tmp_file_path,
        })
    }
}

impl Drop for TmpFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

enum ThumbnailMethod<'a> {
    ImageMagick { path: &'a Path, mime: MediaType<'a> },
    Ffmpeg { path: &'a Path, mime: MediaType<'a> },
}

impl<'a> ThumbnailMethod<'a> {
    fn from_file(
        mime: MediaType<'a>,
        path: &'a Path,
    ) -> Result<ThumbnailMethod<'a>, MediaImportError> {
        match (mime.ty.as_str(), mime.subty.as_str()) {
            ("image", _) | ("application", "pdf") => Ok(Self::ImageMagick { path, mime }),
            ("video", _) => Ok(Self::Ffmpeg { path, mime }),
            _ => Err(MediaImportError::UnsupportedMimeType),
        }
    }
}

fn map_ffmpeg_mime<'a>(mime: &'a MediaType<'_>) -> &'a str {
    match mime.subty.as_str() {
        "x-matroska" => "matroska",
        "quicktime" => "mov",
        x => x,
    }
}

struct ThumbnailDigestable<'a, O> {
    method: ThumbnailMethod<'a>,
    media_digest: MediaDigestable<'a, O>,
}

impl<'a, O> ThumbnailDigestable<'a, O>
where
    O: AsyncWrite + Unpin,
{
    async fn digest(mut self) -> Result<Media, MediaImportError> {
        let output_file = TmpFile::new().await?;

        match self.method {
            ThumbnailMethod::ImageMagick { path, mime } => {
                let mut command = Command::new("convert")
                    .arg(format!(
                        "{}:{}[0]",
                        mime,
                        path.display()
                    ))
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
                    .arg(format!("jpg:{}", output_file.path.display()))
                    .stderr(Stdio::inherit())
                    .spawn()?;

                command.wait().await?;
            }
            ThumbnailMethod::Ffmpeg { path, mime } => {
                let duration = get_video_duration(path).await?;

                let mut ffmpeg_command = Command::new("ffmpeg")
                    .arg("-ss")
                    .arg((duration / 2.0).to_string())
                    .arg("-f")
                    .arg(map_ffmpeg_mime(&mime))
                    .arg("-i")
                    .arg(path.to_str().unwrap())
                    .arg("-vframes")
                    .arg("1")
                    .arg("-c:v")
                    .arg("mjpeg")
                    .arg("-f")
                    .arg("mjpeg")
                    .arg(output_file.path.as_path())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::inherit())
                    .spawn()?;

                ffmpeg_command.wait().await?;
            }
        }

        let mut file = OpenOptions::new()
            .read(true)
            .write(false)
            .open(&output_file.path)
            .await?;

        let mut buffer = [0_u8; 1024];

        loop {
            let size = file.read(&mut buffer).await?;

            if size == 0 {
                break;
            }

            self.media_digest.write(&buffer[0..size]).await?;
        }

        self.media_digest.digest().await
    }
}

struct MediaDigestable<'a, O> {
    size_digest: SizeDigestable,
    sha256_digest: Sha256Digestable,
    sha1_digest: Sha1Digestable,
    md5_digest: MD5Digestable,
    metadata_digest: MetadataDigest<'a>,
    mime: MediaTypeBuf,
    location: Uuid,
    output: O,
}

impl<'a, O> MediaDigestable<'a, O>
where
    O: AsyncWrite + Unpin,
{
    async fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.size_digest.write(data);
        self.sha256_digest.write(data);
        self.sha1_digest.write(data);
        self.md5_digest.write(data);

        self.metadata_digest.write(data).await?;
        self.output.write_all(data).await?;

        Ok(())
    }

    async fn digest(mut self) -> Result<Media, MediaImportError> {
        self.output.flush().await?;

        Ok(Media {
            id: 0,
            metadata: self.metadata_digest.digest().await?,
            file_id: self.location,
            file_size: self.size_digest.digest(),
            sha1: self.sha1_digest.digest(),
            sha256: self.sha256_digest.digest(),
            md5: self.md5_digest.digest(),
            mime: self.mime,
        })
    }
}

async fn pipe_commands(source: &mut Child, target: &mut Child) -> Result<(), MediaImportError> {
    let stdout = source
        .stdout
        .as_mut()
        .ok_or(MediaImportError::UnexpectedOutput)?;

    let stdin = target
        .stdin
        .as_mut()
        .ok_or(MediaImportError::UnexpectedOutput)?;

    let mut buffer = [0_u8; 1024];

    loop {
        let size = stdout.read(&mut buffer).await?;

        if size == 0 {
            break;
        }

        stdin.write_all(&buffer[0..size]).await?;
    }

    Ok(())
}

enum MetadataDigest<'a> {
    ImageFile { path: &'a Path, mime: MediaType<'a> },
    PdfFile { path: &'a Path, mime: MediaType<'a> },
    MiscDoc { path: &'a Path, mime: MediaType<'a> },
    ImageStream { process: Child },
    VideoFile { path: &'a Path, mime: MediaType<'a> },
}

impl<'a> MetadataDigest<'a> {
    fn from_file(
        mime: MediaType<'a>,
        path: &'a Path,
    ) -> Result<MetadataDigest<'a>, MediaImportError> {
        if mime.subty == "pdf" {
            return Ok(Self::PdfFile { path, mime });
        }

        match (mime.ty.as_str(), mime.subty.as_str()) {
            ("image", _) => Ok(Self::ImageFile { path, mime }),
            ("video", _) => Ok(Self::VideoFile { path, mime }),
            ("document", _)
            | ("application", "vnd.openxmlformats-officedocument.wordprocessingml.document") => {
                Ok(Self::MiscDoc { path, mime })
            }
            _ => Err(MediaImportError::UnsupportedMimeType),
        }
    }

    fn new_image_stream(mime: MediaType<'_>) -> Result<MetadataDigest<'a>, MediaImportError> {
        Ok(Self::ImageStream {
            process: Command::new("identify")
                .arg("-ping")
                .arg("-format")
                .arg("%wx%h")
                .arg(format!("{}:-[0]", mime.subty))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit())
                .spawn()?,
        })
    }

    async fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        if let Self::ImageStream { process } = self {
            if let Some(stdin) = process.stdin.as_mut() {
                stdin.write_all(data).await?;
            }
        }

        Ok(())
    }

    fn parse_output(output: Vec<u8>) -> Result<Dimensions, MediaImportError> {
        let output_str =
            String::from_utf8(output).map_err(|_| MediaImportError::UnexpectedOutput)?;

        let split: Vec<&str> = output_str.split('x').collect();

        if split.len() != 2 {
            return Err(MediaImportError::UnexpectedOutput);
        }

        Ok(Dimensions {
            width: split[0]
                .trim()
                .parse()
                .map_err(|_| MediaImportError::UnexpectedOutput)?,
            height: split[1]
                .trim()
                .parse()
                .map_err(|_| MediaImportError::UnexpectedOutput)?,
        })
    }

    async fn digest(self) -> Result<MediaMetadata, MediaImportError> {
        match self {
            MetadataDigest::ImageFile { mime, path } => {
                let command = Command::new("identify")
                    .arg("-ping")
                    .arg("-format")
                    .arg("%wx%h")
                    .arg(format!("{}:{}[0]", mime.subty, path.display()))
                    .stdout(Stdio::piped())
                    .stderr(Stdio::inherit())
                    .spawn()?;

                let output = command.wait_with_output().await?;

                let dims = Self::parse_output(output.stdout)?;
                Ok(MediaMetadata::Image { dims })
            }
            MetadataDigest::ImageStream { mut process } => {
                if let Some(mut stdout) = process.stdin.take() {
                    stdout.flush().await?;
                }

                let output = process.wait_with_output().await?;

                let dims = Self::parse_output(output.stdout)?;
                Ok(MediaMetadata::Image { dims })
            }
            MetadataDigest::VideoFile { mime, path } => {
                let duration = get_video_duration(path).await?;

                let dism_probe = Command::new("ffprobe")
                    .arg("-v")
                    .arg("error")
                    .arg("-f")
                    .arg(map_ffmpeg_mime(&mime))
                    .arg("-select_streams")
                    .arg("v:0")
                    .arg("-show_entries")
                    .arg("stream=width,height")
                    .arg("-of")
                    .arg("csv=s=x:p=0")
                    .arg("-loglevel")
                    .arg("quiet")
                    .arg(path)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::inherit())
                    .spawn()?;

                let dims_output = dism_probe.wait_with_output().await?;

                let dims = Self::parse_output(dims_output.stdout)?;
                Ok(MediaMetadata::Video {
                    dims,
                    duration: duration as i32,
                    video_encoding: get_video_codec(path).await?,
                })
            }
            MetadataDigest::PdfFile { mime, path } => {
                let command = Command::new("pdfinfo")
                    .arg(path)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::inherit())
                    .spawn()?;

                let output = command.wait_with_output().await?;
                let output_str = String::from_utf8(output.stdout)
                    .map_err(|_| MediaImportError::UnexpectedOutput)?;

                Self::parse_pdf_info(output_str.as_str())
            }
            MetadataDigest::MiscDoc { mime, path } => {
                let mut unoconv = Command::new("unoconv")
                    .arg("--stdout")
                    .arg("-f")
                    .arg("pdf")
                    .arg(path)
                    .stderr(Stdio::inherit())
                    .stdout(Stdio::piped())
                    .spawn()?;

                let mut pdfinfo = Command::new("pdfinfo")
                    .arg("-")
                    .stderr(Stdio::inherit())
                    .stdout(Stdio::piped())
                    .stdin(Stdio::piped())
                    .spawn()?;

                pipe_commands(&mut unoconv, &mut pdfinfo).await?;

                let output = pdfinfo.wait_with_output().await?;
                unoconv.wait().await?;

                let output_str = String::from_utf8(output.stdout)
                    .map_err(|_| MediaImportError::UnexpectedOutput)?;

                Self::parse_pdf_info(output_str.as_str())
            }
        }
    }

    fn parse_pdf_info(output_str: &str) -> Result<MediaMetadata, MediaImportError> {
        let mut pages = None;
        let mut author = None;
        let mut title = None;
        let mut page_size = None;

        for line in output_str.lines() {
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "Title" => title = Some(String::from(value)),
                    "Author" => author = Some(String::from(value)),
                    "Pages" => pages = value.parse::<i32>().ok(),
                    "Page size" => {
                        if let Some((mut w, mut h)) = value.split_once('x') {
                            w = w.trim();
                            h = h.trim();

                            if let Some(mut h) = h.split(' ').next() {
                                h = h.trim();

                                if let (Some(w), Some(h)) =
                                    (w.parse::<f32>().ok(), h.parse::<f32>().ok())
                                {
                                    page_size = Some(Dimensions {
                                        width: w as i32,
                                        height: h as i32,
                                    })
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        match (pages, page_size) {
            (Some(pages), Some(page_size)) => Ok(MediaMetadata::Document {
                author,
                pages,
                title,
                page_size,
            }),
            _ => Err(MediaImportError::UnexpectedOutput),
        }
    }
}

#[derive(Default)]
struct Sha256Digestable {
    context: sha2::Sha256,
}

impl Sha256Digestable {
    fn write(&mut self, data: &[u8]) {
        use sha2::Digest;
        self.context.update(data);
    }

    fn digest(self) -> String {
        use sha2::Digest;
        format!("{:x}", self.context.finalize())
    }
}

#[derive(Default)]
struct Sha1Digestable {
    context: sha1::Sha1,
}

impl Sha1Digestable {
    fn write(&mut self, data: &[u8]) {
        use sha1::Digest;
        self.context.update(data);
    }

    fn digest(self) -> String {
        use sha1::Digest;
        format!("{:x}", self.context.finalize())
    }
}

struct MD5Digestable {
    context: md5::Context,
}

impl Default for MD5Digestable {
    fn default() -> Self {
        Self {
            context: md5::Context::new(),
        }
    }
}

impl MD5Digestable {
    fn write(&mut self, data: &[u8]) {
        self.context.consume(data);
    }

    fn digest(self) -> String {
        format!("{:x}", self.context.compute())
    }
}

#[derive(Default)]
struct SizeDigestable {
    size: usize,
}

impl SizeDigestable {
    fn write(&mut self, data: &[u8]) {
        self.size += data.len();
    }

    fn digest(self) -> usize {
        self.size
    }
}

async fn get_video_duration(path: &Path) -> Result<f32, MediaImportError> {
    let duration_probe = Command::new("ffprobe")
        .arg("-loglevel")
        .arg("error")
        .arg("-show_entries")
        .arg("format=duration")
        .arg("-of")
        .arg("default=nk=1:nw=1")
        .arg(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let duration_output = duration_probe.wait_with_output().await?;

    let duration_output_str = String::from_utf8(duration_output.stdout)
        .map_err(|_| MediaImportError::UnexpectedOutput)?;

    let duration = duration_output_str
        .trim()
        .parse::<f32>()
        .map_err(|_| MediaImportError::UnexpectedOutput)?;

    Ok(duration)
}
