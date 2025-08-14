use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, RequestExt};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, PartialEq)]
pub struct ValidatedJsonRequest<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJsonRequest<T>
where
    T: DeserializeOwned + Validate + 'static,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(request) = req
            .extract::<Json<T>, _>()
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
        request
            .validate()
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
        Ok(Self(request))
    }
}

mod test {
    use super::*;
    use axum::body::Body;
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[tokio::test]
    async fn test_validated_json_request_from_request() {
        #[derive(Debug, Deserialize, Serialize, Validate, Default, PartialEq)]
        struct MockRequest {
            #[validate(length(min = 1, message = "missing"))]
            field1: String,
        }
        struct TestCase {
            http_request: Request<Body>,
            expected_result: Result<ValidatedJsonRequest<MockRequest>, (StatusCode, String)>,
        }
        let test_cases = [
            TestCase {
                http_request: Request::new(Body::empty()),
                expected_result: Err((
                    StatusCode::BAD_REQUEST,
                    "Expected request with `Content-Type: application/json`".to_owned(),
                )),
            },
            TestCase {
                http_request: Request::builder()
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&MockRequest::default()).unwrap(),
                    ))
                    .unwrap(),
                expected_result: Err((StatusCode::BAD_REQUEST, "field1: missing".to_owned())),
            },
            TestCase {
                http_request: Request::builder()
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&MockRequest {
                            field1: "this is a valid field".to_string(),
                        })
                        .unwrap(),
                    ))
                    .unwrap(),
                expected_result: Ok(ValidatedJsonRequest(MockRequest {
                    field1: "this is a valid field".to_string(),
                })),
            },
        ];

        for case in test_cases {
            let actual_result: Result<ValidatedJsonRequest<MockRequest>, (StatusCode, String)> =
                ValidatedJsonRequest::from_request(case.http_request, &()).await;
            match case.expected_result {
                Ok(expected_happy_path) => {
                    assert_eq!(expected_happy_path, actual_result.unwrap());
                }
                Err(expected_error_path) => {
                    let actual_error = actual_result.unwrap_err();
                    assert_eq!(expected_error_path.0, actual_error.clone().0);
                    assert_eq!(expected_error_path.1, actual_error.1);
                }
            }
        }
    }
}
