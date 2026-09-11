use crate::features::prayer_times::models::prayer_times_dto::PrayerTimesDTO;
use axum::body::Body;
use axum::http::{StatusCode, header};
use axum::response::Response;

pub fn build_prayer_times_response(
    prayer_times: PrayerTimesDTO,
    hash: Option<&str>,
) -> Result<Response<Body>, StatusCode> {
    if let Some(hash_value) = hash {
        if prayer_times.hash == hash_value.to_owned() {
            return Err(StatusCode::CONFLICT);
        }
    }
    if let Some(data) = prayer_times.data {
        // Create response_body_result with hash in a custom header
        return Response::builder()
            .status(StatusCode::OK)
            .header("X-File-Hash", prayer_times.hash)
            .header(header::CONTENT_TYPE, "application/octet-stream")
            .body(Body::from(data))
            .map_err(move |err| {
                tracing::error!(err = ?err, "error building prayer times response");
                StatusCode::INTERNAL_SERVER_ERROR
            });
    }
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
