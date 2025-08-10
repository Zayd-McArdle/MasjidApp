use async_trait::async_trait;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, RequestExt};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedJsonRequest<T>(pub T);

#[async_trait]
impl<T, S, R> FromRequest<R> for ValidatedJsonRequest<T>
where
    T: DeserializeOwned + Validate,
    R: Send + 'static,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(request) = request
            .extract::<Json<T>, _>()
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e).into_response())?;
        request
            .validate()
            .map_err(|e| (StatusCode::BAD_REQUEST, e).into_response())?;
    }
}
