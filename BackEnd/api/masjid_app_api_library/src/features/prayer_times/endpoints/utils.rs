use crate::features::prayer_times::models::prayer_times_dto::PrayerTimesDTO;
use axum::body::Body;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};

pub fn build_prayer_times_response(prayer_times: PrayerTimesDTO, hash: Option<&str>) -> Response {
    if let Some(hash_value) = hash {
        if prayer_times.hash == hash_value.to_owned() {
            return StatusCode::CONFLICT.into_response();
        }
    }
    if let Some(data) = prayer_times.data {
        // Create response_body_result with hash in a custom header
        let response_body_result = Response::builder()
            .status(StatusCode::OK)
            .header("X-File-Hash", prayer_times.hash)
            .header(header::CONTENT_TYPE, "application/octet-stream")
            .body(Body::from(data));
        return match response_body_result {
            Ok(response) => response,
            Err(err) => {
                tracing::error!("unable to build response: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        };
    }
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
