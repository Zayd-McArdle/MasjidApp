use axum::http::StatusCode;

#[inline]
pub fn bad_request(validation_errors: impl ToString) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, validation_errors.to_string())
}
