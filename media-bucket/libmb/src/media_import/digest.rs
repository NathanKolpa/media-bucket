use crate::data_source::MediaImportError;


pub trait Digest {
    type Output;
    async fn write(&mut self, data: &[u8]) -> Result<(), MediaImportError>;
    async fn digest(self) -> Result<Self::Output, MediaImportError>;
}