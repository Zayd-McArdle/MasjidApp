use crate::shared::extractors::file_handler::FileHandler;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use std::fmt::Display;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub enum UploadError {
    EmptyFile,
    NoFileName,
    InvalidFileName,
    UnsupportedFileType(String),
    SystemIOError,
}
const NO_FILE_EXTENSION: &'static str = "";
impl Display for UploadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            UploadError::EmptyFile => "Empty file".to_owned(),
            UploadError::NoFileName => "No filename".to_owned(),
            UploadError::InvalidFileName => "Invalid filename".to_owned(),
            UploadError::UnsupportedFileType(e) => {
                if e == NO_FILE_EXTENSION {
                    return write!(f, "{}", "File extension not found".to_owned());
                }
                format!("Unsupported type: {}", e)
            }
            UploadError::SystemIOError => "System IO error".to_owned(),
        };
        write!(f, "{}", str)
    }
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
impl FileHandler {
    pub async fn save_file(&self, data: &[u8], file_name: String) -> Result<String, UploadError> {
        if file_name.is_empty() {
            return Err(UploadError::NoFileName);
        } else if data.is_empty() {
            return Err(UploadError::EmptyFile);
        } else if !file_name.contains('.') {
            return Err(UploadError::UnsupportedFileType(
                NO_FILE_EXTENSION.to_owned(),
            ));
        } else if file_name.chars().nth(file_name.len() - 1) == Some('.') {
            tracing::error!("invalid file name: {}", file_name);
            return Err(UploadError::InvalidFileName);
        } else if let Some(extension) = file_name.split('.').last() {
            if !is_supported_file_extension(extension) {
                tracing::error!("invalid file extension: {}", file_name);
                return Err(UploadError::UnsupportedFileType(extension.to_owned()));
            }
        }
        let path = self.upload_dir.join(&file_name);
        tokio::fs::write(&path, data).await.map_err(|e| {
            tracing::error!("unable to upload file: {}\nerror: {}", &file_name, &e);
            UploadError::SystemIOError
        })?;
        Ok(format!("/{}/{}", &self.endpoint, file_name))
    }
}

mod test {
    use crate::shared::extractors::file_handler::file_uploader::{UploadError, NO_FILE_EXTENSION};
    use crate::shared::extractors::file_handler::FileHandler;

    #[tokio::test]
    async fn test_file_uploader_save_file() {
        struct TestCase {
            file_name: String,
            file_data: Vec<u8>,
            expected_result: Result<String, UploadError>,
        }
        let test_cases = vec![
            //When there is an empty file, I should get an empty file error
            TestCase {
                file_name: "filename".to_owned(),
                file_data: vec![],
                expected_result: Err(UploadError::EmptyFile),
            },
            // When there is no file name, I should get a no file name error
            TestCase {
                file_name: "".to_owned(),
                file_data: vec![1],
                expected_result: Err(UploadError::NoFileName),
            },
            // When there is an invalid file name, I should get an invalid file name error
            TestCase {
                file_name: ".".to_owned(),
                file_data: vec![1],
                expected_result: Err(UploadError::InvalidFileName),
            },
            // When there is no file extension, I should get an unable to locate file extension error
            TestCase {
                file_name: "filename".to_owned(),
                file_data: vec![1],
                expected_result: Err(UploadError::UnsupportedFileType(
                    NO_FILE_EXTENSION.to_owned(),
                )),
            },
            // When I input an unsupported file exception, I should get an unsupported file extension error
            TestCase {
                file_name: "filename.txt".to_owned(),
                file_data: vec![1],
                expected_result: Err(UploadError::UnsupportedFileType(String::from("txt"))),
            },
            // When I input a supported file, I should get no error
            TestCase {
                file_name: "filename.json".to_owned(),
                file_data: vec![1],
                expected_result: Ok("/uploads/filename.json".to_owned()),
            },
        ];
        for test_case in test_cases {
            let file_uploader = FileHandler::new(std::env::temp_dir(), "uploads".to_owned());
            let actual_result = file_uploader
                .save_file(&test_case.file_data, test_case.file_name)
                .await;
            assert_eq!(test_case.expected_result, actual_result)
        }
    }
}
