use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use std::path::{Path, PathBuf};

pub mod file_deleter;
pub mod file_uploader;
#[derive(Debug, Eq, PartialEq)]
enum FilePathSafetyError {
    EmptyPath,
    PathTraversalAttack,
}
fn file_path_is_safe(path: impl AsRef<Path>) -> Result<(), FilePathSafetyError> {
    let path_string_option = path.as_ref().to_str();
    if let Some(path_string_value) = path_string_option {
        if path_string_value.is_empty() {
            return Err(FilePathSafetyError::EmptyPath);
        } else if path_string_value.contains('\\') || path_string_value.contains("..") {
            return Err(FilePathSafetyError::PathTraversalAttack);
        }
        return Ok(());
    }
    Err(FilePathSafetyError::EmptyPath)
}
// FileHandler is responsible for uploading and deleting files from the web server
#[derive(Clone, Default)]
pub struct FileHandler {
    upload_dir: PathBuf,
    endpoint: String,
}
impl FileHandler {
    pub fn new(upload_directory: impl AsRef<Path>, endpoint: String) -> Self {
        Self {
            upload_dir: upload_directory.as_ref().to_path_buf(),
            endpoint: endpoint,
        }
    }
}
impl<S> FromRequestParts<S> for FileHandler
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::default())
        // TODO - Fix file handler implementation
        /*parts.extensions.get::<FileHandler>().cloned().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "FileUploader not configured".into(),
        ))*/
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;
    #[test]
    fn test_file_is_safe() {
        struct TestCases<T: AsRef<Path>> {
            test_path: T,
            expected_result: Result<(), FilePathSafetyError>,
        }
        let test_cases = [
            // When there is an empty file path, I should get an empty path error
            TestCases {
                test_path: Path::new(""),
                expected_result: Err(FilePathSafetyError::EmptyPath),
            },
            // When there is dangerous characters in the path, I should get a path traversal error
            TestCases {
                test_path: Path::new("/\\.."),
                expected_result: Err(FilePathSafetyError::PathTraversalAttack),
            },
            // When there is a valid path, there should be an ok response
            TestCases {
                test_path: Path::new("/uploads/somefile.txt"),
                expected_result: Ok(()),
            },
        ];
        for test_case in test_cases {
            let actual_result = file_path_is_safe(test_case.test_path);
            assert_eq!(test_case.expected_result, actual_result,)
        }
    }
}
