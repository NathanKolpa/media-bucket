use crate::data_source::MediaImportError;
use crate::media_import::digest::Digest;

#[derive(Default)]
struct Sha256Digest {
    context: sha2::Sha256,
}

impl Digest for Sha256Digest {
    type Output = String;

    async fn write(&mut self, data: &[u8]) -> Result<(), MediaImportError> {
        use sha2::Digest;
        self.context.update(data);
        Ok(())
    }

    async fn digest(self) -> Result<Self::Output, MediaImportError> {
        use sha2::Digest;
        Ok(format!("{:x}", self.context.finalize()))
    }
}

#[derive(Default)]
struct Sha1Digest {
    context: sha1::Sha1,
}

impl Digest for Sha1Digest {
    type Output = String;

    async fn write(&mut self, data: &[u8]) -> Result<(), MediaImportError> {
        use sha1::Digest;
        self.context.update(data);
        Ok(())
    }

    async fn digest(self) -> Result<Self::Output, MediaImportError> {
        use sha1::Digest;
        Ok(format!("{:x}", self.context.finalize()))
    }
}

struct MD5Digest {
    context: md5::Context,
}

impl Default for MD5Digest {
    fn default() -> Self {
        Self {
            context: md5::Context::new(),
        }
    }
}

impl Digest for MD5Digest {
    type Output = String;

    async fn write(&mut self, data: &[u8]) -> Result<(), MediaImportError> {
        self.context.consume(data);
        Ok(())
    }

    async fn digest(self) -> Result<Self::Output, MediaImportError> {
        Ok(format!("{:x}", self.context.compute()))
    }
}

#[derive(Default)]
struct SizeDigest {
    size: usize,
}

impl Digest for SizeDigest {
    type Output = usize;

    async fn write(&mut self, data: &[u8]) -> Result<(), MediaImportError> {
        self.size += data.len();
        Ok(())
    }

    async fn digest(self) -> Result<Self::Output, MediaImportError> {
        Ok(self.size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_size_without_write() {
        let size_digest = SizeDigest::default();

        let size = size_digest.digest().await.unwrap();

        assert_eq!(0, size)
    }


    #[tokio::test]
    async fn test_size() {
        let mut size_digest = SizeDigest::default();
        let test_data = [0; 21];

        size_digest.write(&test_data).await.unwrap();
        let size = size_digest.digest().await.unwrap();

        assert_eq!(21, size)
    }

    #[tokio::test]
    async fn test_sha256() {
        let mut size_digest = Sha256Digest::default();
        let test_data = "Hello World".as_bytes();

        size_digest.write(test_data).await.unwrap();
        let size = size_digest.digest().await.unwrap();

        assert_eq!("a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e", size)
    }

    #[tokio::test]
    async fn test_sha1() {
        let mut size_digest = Sha1Digest::default();
        let test_data = "Hello World".as_bytes();

        size_digest.write(test_data).await.unwrap();
        let size = size_digest.digest().await.unwrap();

        assert_eq!("0a4d55a8d778e5022fab701977c5d840bbc486d0", size)
    }

    #[tokio::test]
    async fn test_md5() {
        let mut size_digest = MD5Digest::default();
        let test_data = "Hello World".as_bytes();

        size_digest.write(test_data).await.unwrap();
        let size = size_digest.digest().await.unwrap();

        assert_eq!("b10a8db164e0754105b7a99be72e3fe5", size)
    }
}