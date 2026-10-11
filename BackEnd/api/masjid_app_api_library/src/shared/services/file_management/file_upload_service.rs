use async_trait::async_trait;

#[async_trait]
pub trait FileUploadService {
    async fn upload(&self, files: Vec<&[u8]>) -> Result<(), ()>;
}
