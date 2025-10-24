use crate::shared::extractors::file_handler::{
    file_path_is_safe, FileHandler, FilePathSafetyError,
};
use std::io::ErrorKind;
use std::path::Path;

#[derive(Debug)]
pub enum DeleteError {
    EmptyPath,
    PathIsTraversal,
    FileNotFound,
    DirectoryNotFound,
    PermissionDenied,
    DirectoryMistookForFile,
    UnableToDeleteFileDueToReadOnlyAccess,
    UnableToDeleteFileDueToBeingInUse,
    IOError(std::io::Error),
}
impl FileHandler {
    pub async fn delete_file(&self, file_path: impl AsRef<Path>) -> Result<(), DeleteError> {
        let path = file_path.as_ref();
        file_path_is_safe(&path).map_err(|err| match err {
            FilePathSafetyError::EmptyPath => DeleteError::EmptyPath,
            FilePathSafetyError::PathTraversalAttack => {
                tracing::warn!("user attempted path traversal attack: {}", path.display());
                DeleteError::PathIsTraversal
            }
        })?;

        // Check if parent directory exists
        if let Some(parent) = path.parent()
            && !parent.exists()
        {
            return Err(DeleteError::DirectoryNotFound);
        }

        // Check if path exists and is a file
        match tokio::fs::metadata(path).await {
            Ok(metadata) => {
                if !metadata.is_file() {
                    return Err(DeleteError::DirectoryMistookForFile);
                }
            }
            Err(e) => {
                if e.kind() != ErrorKind::NotFound {
                    return Err(DeleteError::FileNotFound);
                }
                tracing::error!("Failed to get metadata for {}: {}", path.display(), e);
                return Err(DeleteError::IOError(e));
            }
        };

        // Attempt to delete the file
        tokio::fs::remove_file(path)
            .await
            .map_err(|e| match e.kind() {
                ErrorKind::PermissionDenied => DeleteError::PermissionDenied,
                ErrorKind::ReadOnlyFilesystem => DeleteError::UnableToDeleteFileDueToReadOnlyAccess,
                ErrorKind::ResourceBusy => DeleteError::UnableToDeleteFileDueToBeingInUse,
                _ => {
                    tracing::error!("Failed to delete file {}: {}", path.display(), e);
                    return DeleteError::IOError(e);
                }
            })
    }
} /*
#[cfg(test)]
mod tests {
use super::*;
use std::path::PathBuf;
use tempdir::TempDir;

// Test case struct to replace tuples
struct TestCase {
name: &'static str,
path: PathBuf,
path_exists: bool,
expected: Result<(), DeleteError>,
setup: Box<dyn Fn() -> (PathBuf, Result<(), DeleteError>)>,
}

#[tokio::test]
async fn test_delete_file() {
let tests: Vec<TestCase> = vec![
TestCase {
name: "parent_directory_not_found",
path: "non_existent_directory/file.txt".into(),
path_exists: false,
expected: Err(DeleteError::DirectoryNotFound),
}

TestCase {
name: "parent_directory_not_found",
setup: Box::new(|| {
(
"non_existent_directory/file.txt".into(),
Err(DeleteError::DirectoryNotFound),
)
}),
},
TestCase {
name: "directory_mistook_for_file",
setup: Box::new(|| {
(
TempDir::new("I_am_a_directory").unwrap().path().to_owned(),
Err(DeleteError::DirectoryMistookForFile),
)
}),
},
TestCase {
name: "file_not_found",
setup: Box::new(|| {
/*let dir = tempdir().unwrap();
                    let path = dir.path().join("nonexistent.txt");*/
                    ("nonexistent.txt".into(), Err(DeleteError::FileNotFound))
                }),
            },
            /*TestCase {
                name: "delete_permission_denied",
                setup: Box::new(|| {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let dir = tempdir().unwrap();
                        let path = dir.path().join("restricted.txt");

                        // Create the file synchronously since we're in a sync closure
                        std::fs::File::create(&path).unwrap();
                        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o000))
                            .unwrap();

                        (path, Err(DeleteError::PermissionDenied))
                    }
                    #[cfg(not(unix))]
                    {
                        // Skip test on non-Unix systems
                        let dir = tempdir().unwrap();
                        let path = dir.path().join("dummy.txt");
                        (path, Ok(()))
                    }
                }),
            },
            TestCase {
                name: "success",
                setup: Box::new(|| {
                    let file = NamedTempFile::new().unwrap();
                    let path = file.path().to_owned();
                    // Keep the file handle to prevent automatic deletion
                    (path, Ok(()))
                }),
            },*/
        ];

        for test in tests {
            if test.path_exists {
                std:::
            }
            let handler = FileHandler::default();
            let result = handler.delete_file(&path).await.map_err(|actual_err| {
                let expected_err = expected.as_ref().unwrap_err();
                match expected_err {
                    DeleteError::EmptyPath => assert!(matches!(actual_err, DeleteError::EmptyPath)),
                    DeleteError::PathIsTraversal => {
                        assert!(matches!(actual_err, DeleteError::PathIsTraversal))
                    }
                    DeleteError::FileNotFound => {
                        assert!(matches!(actual_err, DeleteError::FileNotFound))
                    }
                    DeleteError::DirectoryNotFound => {
                        assert!(matches!(actual_err, DeleteError::DirectoryNotFound))
                    }
                    DeleteError::PermissionDenied => {
                        assert!(matches!(actual_err, DeleteError::PermissionDenied))
                    }
                    DeleteError::DirectoryMistookForFile => {
                        assert!(
                            matches!(actual_err, DeleteError::DirectoryMistookForFile),
                            "Test '{}' failed: {:?}",
                            path.display(),
                            actual_err
                        );
                    }
                    DeleteError::UnableToDeleteFileDueToReadOnlyAccess => assert!(matches!(
                        actual_err,
                        DeleteError::UnableToDeleteFileDueToReadOnlyAccess
                    )),
                    DeleteError::UnableToDeleteFileDueToBeingInUse => assert!(matches!(
                        actual_err,
                        DeleteError::UnableToDeleteFileDueToBeingInUse
                    )),
                    DeleteError::IOError(_) => {
                        assert!(matches!(actual_err, DeleteError::IOError(_)))
                    }
                }
            });
            if !expected.is_ok() {
                continue;
            }
            assert!(result.is_ok(), "Test '{}' failed: {:?}", test.name, result)
        }

        /*// Test for IOError case (file not found during metadata check)
        let dir = tempdir().unwrap();
        let non_existent_path = dir.path().join("non_existent_file.txt");
        let handler = FileHandler::default();
        let result = handler.delete_file(&non_existent_path).await;

        // This should return FileNotFound, not IOError, based on the function logic
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DeleteError::FileNotFound));*/
    }
}
*/
