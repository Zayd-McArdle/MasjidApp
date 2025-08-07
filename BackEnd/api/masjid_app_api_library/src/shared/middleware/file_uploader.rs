use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use std::path::{Path, PathBuf};
#[derive(Debug, PartialEq)]
pub enum FileUploaderError {
    EmptyFile,
    NoFileName,
    InvalidFile,
    UnableToLocateFileExtension,
    UnsupportedFileExtension(String),
    SystemIOError,
}
#[derive(Clone)]
pub struct FileUploader {
    upload_dir: PathBuf,
    endpoint: String,
}
fn is_supported_file_extension(extension: &str) -> bool {
    extension == "png"
        || extension == "jpg"
        || extension == "jpeg"
        || extension == "gif"
        || extension == "json"
        || extension == "csv"
        || extension == "json"
}

impl FileUploader {
    pub fn new(upload_directory: impl AsRef<Path>, endpoint: Option<String>) -> Self {
        Self {
            upload_dir: upload_directory.as_ref().to_path_buf(),
            // Default to uploads directory if no custom endpoint needed
            endpoint: endpoint.unwrap_or("uploads".to_owned()),
        }
    }

    pub async fn save_file(
        &self,
        data: &[u8],
        file_name: Option<String>,
    ) -> Result<String, FileUploaderError> {
        if data.len() == 0 {
            return Err(FileUploaderError::EmptyFile);
        }
        match file_name {
            None => Err(FileUploaderError::NoFileName),

            Some(name) => {
                if name.is_empty() {
                    return Err(FileUploaderError::NoFileName);
                } else if !name.contains('.') {
                    tracing::error!("unable to locate file extension: {}", name);
                    return Err(FileUploaderError::UnableToLocateFileExtension);
                } else if name.chars().nth(name.len() - 1) == Some('.') {
                    tracing::error!("invalid file name: {}", name);
                    return Err(FileUploaderError::InvalidFile);
                } else if let Some(extension) = name.split('.').last() {
                    if !is_supported_file_extension(extension) {
                        tracing::error!("unsupported file extension: .{}", extension);
                        return Err(FileUploaderError::UnsupportedFileExtension(
                            extension.to_owned(),
                        ));
                    }
                }
                let path = self.upload_dir.join(&name);
                tokio::fs::write(&path, data).await.map_err(|e| {
                    tracing::error!("unable to upload file: {}\nerror: {}", &name, &e);
                    FileUploaderError::SystemIOError
                })?;
                Ok(format!("/{}/{}", &self.endpoint, name))
            }
        }
    }
}

impl<S> FromRequestParts<S> for FileUploader
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<FileUploader>().cloned().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "FileUploader not configured".into(),
        ))
    }
}

mod test {
    use crate::shared::middleware::file_uploader::{FileUploader, FileUploaderError};
    use std::env::temp_dir;

    #[tokio::test]
    async fn test_file_uploader_save_file() {
        struct TestCase {
            file_name: Option<String>,
            file_data: Vec<u8>,
            endpoint: Option<String>,
            expected_result: Result<String, FileUploaderError>,
        }
        let test_cases = vec![
            //When there is an empty file, I should get an empty file error
            TestCase {
                file_name: None,
                file_data: vec![],
                endpoint: None,
                expected_result: Err(FileUploaderError::EmptyFile),
            },
            // When there is no file name, I should get a no file name error
            TestCase {
                file_name: None,
                file_data: vec![1],
                endpoint: None,
                expected_result: Err(FileUploaderError::NoFileName),
            },
            // When there is no file extension, I should get an unable to locate file extension error
            TestCase {
                file_name: Some(String::from("filename")),
                file_data: vec![1],
                endpoint: None,
                expected_result: Err(FileUploaderError::UnableToLocateFileExtension),
            },
            // When I input an unsupported file exception, I should get an unsupported file extension error
            TestCase {
                file_name: Some(String::from("filename.txt")),
                file_data: vec![1],
                endpoint: None,
                expected_result: Err(FileUploaderError::UnsupportedFileExtension(String::from(
                    "txt",
                ))),
            },
        ];
        for test_case in test_cases {
            let file_uploader = FileUploader::new(temp_dir(), test_case.endpoint);
            let actual_result = file_uploader
                .save_file(&test_case.file_data, test_case.file_name)
                .await;
            assert_eq!(test_case.expected_result, actual_result)
        }
    }
}
