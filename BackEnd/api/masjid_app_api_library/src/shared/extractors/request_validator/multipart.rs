use async_trait::async_trait;
use axum::body::{Body, Bytes};
use axum::extract::{FromRequest, Multipart, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedMultipartRequest<T> {
    pub metadata: T,      // Validated metadata (e.g., title, description)
    pub file_data: Bytes, // Raw binary file content
    pub filename: String, // Original filename if provided
}

impl<T, S> FromRequest<S> for ValidatedMultipartRequest<T>
where
    T: DeserializeOwned + Validate + 'static,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 3a. MULTIPART EXTRACTION
        let mut multipart = Multipart::from_request(req, state).await.map_err(|e| {
            tracing::error!("unable to get multi-part: {}", &e);
            (
                StatusCode::BAD_REQUEST,
                format!("Multipart error: {}", e.to_string()),
            )
        })?;

        // 3b. FIELD TRACKING
        let mut metadata_str = None;
        let mut file_data = None;
        let mut filename = None;

        // 4. FIELD PROCESSING LOOP
        while let Some(field) = multipart.next_field().await.map_err(|e| {
            tracing::error!("unable to get multipart field: {}", &e);
            (
                StatusCode::BAD_REQUEST,
                format!("Field error: {}", e.to_string()),
            )
        })? {
            match field.name() {
                // 4a. METADATA HANDLING
                Some("metadata") => {
                    metadata_str = Some(field.text().await.map_err(|e| {
                        (
                            StatusCode::BAD_REQUEST,
                            format!("Metadata text error: {}", e),
                        )
                    })?);
                }

                // 4b. FILE HANDLING
                Some("file") => {
                    filename = field.file_name().map(|s| s.to_string());
                    file_data = Some(field.bytes().await.map_err(|e| {
                        (
                            StatusCode::BAD_REQUEST,
                            format!("File read error: {}", e.to_string()),
                        )
                    })?);
                }

                // 4c. UNKNOWN FIELDS
                _ => continue,
            }
        }

        // 5. VALIDATION PIPELINE

        // 5a. Check metadata exists
        let metadata_str =
            metadata_str.ok_or((StatusCode::BAD_REQUEST, "Missing metadata field".to_owned()))?;
        let filename = filename.ok_or((StatusCode::BAD_REQUEST, "Missing filename".to_owned()))?;

        // 5b. Deserialize JSON
        let metadata: T = serde_json::from_str(&metadata_str)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("JSON parse error: {}", e)))?;

        // 5c. Validate struct
        metadata.validate().map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Validation failed: {:?}", e),
            )
        })?;

        // 5d. Check file exists
        let file_data =
            file_data.ok_or((StatusCode::BAD_REQUEST, "Missing file field".to_owned()))?;

        // 6. SUCCESSFUL RESULT
        Ok(Self {
            metadata,
            file_data,
            filename,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{Body, Bytes},
        http::{header, Request, StatusCode},
    };
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Debug, Serialize, Deserialize, Validate, PartialEq)]
    struct TestMetadata {
        #[validate(length(min = 1, message = "Name cannot be empty"))]
        name: String,
    }

    // Test case structure
    struct TestCase {
        name: &'static str,
        metadata: Option<&'static str>,
        file: Option<(&'static str, &'static [u8])>,
        expected_status: StatusCode,
        expected_error: Option<&'static str>,
    }

    #[tokio::test]
    async fn test_multipart_extraction() {
        let test_cases = vec![
            // Valid case
            TestCase {
                name: "valid request",
                metadata: Some(r#"{"name": "test"}"#),
                file: Some(("test.txt", b"content")),
                expected_status: StatusCode::OK,
                expected_error: None,
            },
            // Missing metadata (empty body)
            TestCase {
                name: "missing metadata - empty body",
                metadata: None,
                file: None,
                expected_status: StatusCode::BAD_REQUEST,
                expected_error: Some("Missing metadata field"),
            },
            // Has fields but no metadata
            TestCase {
                name: "missing metadata - has file",
                metadata: None,
                file: Some(("test.txt", b"content")),
                expected_status: StatusCode::BAD_REQUEST,
                expected_error: Some("Missing metadata field"),
            },
            // Has fields but no file
            TestCase {
                name: "missing file - has metadata",
                metadata: Some(r#"{"name": "test"}"#),
                file: None,
                expected_status: StatusCode::BAD_REQUEST,
                expected_error: Some("Missing filename"),
            },
            // Invalid JSON
            TestCase {
                name: "invalid json",
                metadata: Some("invalid json"),
                file: Some(("test.txt", b"content")),
                expected_status: StatusCode::BAD_REQUEST,
                expected_error: Some("JSON parse error: "),
            },
            // Validation failed
            TestCase {
                name: "validation failed",
                metadata: Some(r#"{"name": ""}"#),
                file: Some(("test.txt", b"content")),
                expected_status: StatusCode::BAD_REQUEST,
                expected_error: Some("Validation failed: "),
            },
        ];

        for case in test_cases {
            println!("Running test case: {}", case.name);
            // Create multipart body
            let boundary = "TESTBOUNDARY";
            let mut body = Vec::new();

            if let Some(meta) = case.metadata {
                body.extend_from_slice(
                    format!(
                        "--{boundary}\r\n\
                        Content-Disposition: form-data; name=\"metadata\"\r\n\r\n\
                        {meta}\r\n"
                    )
                    .as_bytes(),
                );
            }

            if let Some((filename, content)) = case.file {
                body.extend_from_slice(
                    format!(
                        "--{boundary}\r\n\
                        Content-Disposition: form-data; name=\"file\"; filename=\"{filename}\"\r\n\
                        Content-Type: application/octet-stream\r\n\r\n"
                    )
                    .as_bytes(),
                );
                body.extend_from_slice(content);
                body.extend_from_slice(b"\r\n");
            }

            body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());

            let req = Request::builder()
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .body(Body::from(body))
                .unwrap();

            let result = ValidatedMultipartRequest::<TestMetadata>::from_request(req, &()).await;

            match (result, case.expected_status, case.expected_error) {
                (Ok(actual), StatusCode::OK, None) => {
                    assert_eq!(actual.metadata.name, "test");
                    if case.file.is_some() {
                        assert_eq!(actual.file_data, Bytes::from("content"));
                        assert_eq!(actual.filename, "test.txt");
                    }
                }
                (Err((status, msg)), expected_status, Some(expected_error)) => {
                    assert_eq!(status, expected_status);
                    assert!(
                        msg.contains(expected_error),
                        "{}: Expected '{}' in '{}'",
                        case.name,
                        expected_error,
                        msg
                    );
                }
                _ => panic!("Unexpected result for case: {}", case.name),
            }
        }
    }
}
