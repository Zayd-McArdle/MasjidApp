use async_trait::async_trait;
use axum::body::Bytes;
use axum::extract::{FromRequest, Multipart, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedMultipartRequest<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    pub metadata: T,              // Validated metadata (e.g., title, description)
    pub file_data: Bytes,         // Raw binary file content
    pub filename: Option<String>, // Original filename if provided
}

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedMultipartRequest<T>
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 3a. MULTIPART EXTRACTION
        let mut multipart = Multipart::from_request(req, state).await.map_err(|e| {
            tracing::error!("unable to get multi-part: {}", e);
            (StatusCode::BAD_REQUEST, format!("Multipart error: {}", e)).into_response()
        })?;

        // 3b. FIELD TRACKING
        let mut metadata_str = None;
        let mut file_data = None;
        let mut filename = None;

        // 4. FIELD PROCESSING LOOP
        while let Some(field) = multipart.next_field().await.map_err(|e| {
            tracing::error!("unable to get multipart field: {}", e);
            (StatusCode::BAD_REQUEST, format!("Field error: {}", e)).into_response()
        })? {
            match field.name() {
                // 4a. METADATA HANDLING
                Some("metadata") => {
                    metadata_str = Some(field.text().await.map_err(|e| {
                        (
                            StatusCode::BAD_REQUEST,
                            format!("Metadata text error: {}", e),
                        )
                            .into_response()
                    })?);
                }

                // 4b. FILE HANDLING
                Some("file") => {
                    filename = field.file_name().map(|s| s.to_string());
                    file_data = Some(field.bytes().await.map_err(|e| {
                        (StatusCode::BAD_REQUEST, format!("File read error: {}", e)).into_response()
                    })?);
                }

                // 4c. UNKNOWN FIELDS
                _ => continue,
            }
        }

        // 5. VALIDATION PIPELINE
        // 5a. Check metadata exists
        let metadata_str = metadata_str
            .ok_or((StatusCode::BAD_REQUEST, "Missing metadata field").into_response())?;

        // 5b. Deserialize JSON
        let metadata: T = serde_json::from_str(&metadata_str).map_err(|e| {
            (StatusCode::BAD_REQUEST, format!("JSON parse error: {}", e)).into_response()
        })?;

        // 5c. Validate struct
        metadata.validate().map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Validation failed: {:?}", e),
            )
                .into_response()
        })?;

        // 5d. Check file exists
        let file_data =
            file_data.ok_or((StatusCode::BAD_REQUEST, "Missing file field").into_response())?;

        // 6. SUCCESSFUL RESULT
        Ok(Self {
            metadata,
            file_data,
            filename,
        })
    }
}
