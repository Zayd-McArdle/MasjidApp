use crate::features::ask_imam::errors::{DeleteQuestionError, UpsertAnswerToQuestionError};
use crate::features::ask_imam::models::{
    GetImamQuestionsAdminRequest, ProvideAnswerForImamQuestionRequest, QuestionStatus,
};
use crate::features::ask_imam::repositories::ImamQuestionsAdminRepository;
use crate::features::ask_imam::services::AskImamAdminService;
use crate::shared::jwt::Claims;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use masjid_app_api_library::features::ask_imam::endpoints::send_response_for_get_imam_questions;
use masjid_app_api_library::features::ask_imam::models::SchoolOfThought;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::str::FromStr;
use std::sync::Arc;
use validator::Validate;

pub async fn get_imam_questions(
    State(state): State<ServiceAppState<Arc<dyn AskImamAdminService>>>,
    claims: Claims,
    Query(request): Query<GetImamQuestionsAdminRequest>,
) -> Response {
    if request.validate().is_err() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let get_questions_result = state
        .service
        .get_questions(
            request
                .question_status
                .and_then(|status| QuestionStatus::from_str(&status).ok()),
            request.topic,
            request
                .school_of_thought
                .and_then(|school_of_thought| SchoolOfThought::from_str(&school_of_thought).ok()),
        )
        .await;
    send_response_for_get_imam_questions(get_questions_result)
}

pub async fn provide_answer_for_imam_question(
    State(state): State<ServiceAppState<Arc<dyn AskImamAdminService>>>,
    claims: Claims,
    Json(request): Json<ProvideAnswerForImamQuestionRequest>,
) -> Response {
    if request.validate().is_err() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    match state
        .service
        .provide_answer_to_question(request.question_id, request.into())
        .await
    {
        Ok(()) => StatusCode::OK.into_response(),
        Err(UpsertAnswerToQuestionError::QuestionNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(UpsertAnswerToQuestionError::UnableToUpsertAnswerToQuestion) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_imam_question(
    State(state): State<ServiceAppState<Arc<dyn AskImamAdminService>>>,
    claims: Claims,
    Path(questions_id): Path<i32>,
) -> Response {
    if questions_id == 0 {
        return (StatusCode::BAD_REQUEST, "question ids cannot be 0").into_response();
    }
    match state.service.delete_question(questions_id).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(DeleteQuestionError::QuestionNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(DeleteQuestionError::UnableToDeleteQuestion) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

mod test {
    use super::*;
    use crate::features::ask_imam::services::MockAskImamAdminService;

    #[tokio::test]
    async fn test_provide_answer_for_imam_question() {
        struct TestCase {
            description: &'static str,
            request: ProvideAnswerForImamQuestionRequest,
            expected_db_response: Option<Result<(), UpsertAnswerToQuestionError>>,
            expected_status_code: StatusCode,
        }
        let valid_request = ProvideAnswerForImamQuestionRequest {
            question_id: 1,
            imam_name: "Zayd".to_owned(),
            text: "This is a test answer".to_owned(),
        };
        let test_cases = [
            TestCase {
                description: "When the JSON request is invalid, I should get a BAD_REQUEST response",
                request: ProvideAnswerForImamQuestionRequest {
                    question_id: 0,
                    imam_name: "".to_owned(),
                    text: "".to_owned(),
                },
                expected_db_response: None,
                expected_status_code: StatusCode::BAD_REQUEST,
            },
            TestCase {
                description: "When upserting an answer to a non-existent question, I should get a NOT_FOUND response",
                request: valid_request.clone(),
                expected_db_response: Some(Err(UpsertAnswerToQuestionError::QuestionNotFound)),
                expected_status_code: StatusCode::NOT_FOUND,
            },
            TestCase {
                description: "When upsertion fails, I should get an INTERNAL_SERVER_ERROR response",
                request: valid_request.clone(),
                expected_db_response: Some(Err(
                    UpsertAnswerToQuestionError::UnableToUpsertAnswerToQuestion,
                )),
                expected_status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                description: "When upsertion succeeds, I should get an OK response",
                request: valid_request,
                expected_db_response: Some(Ok(())),
                expected_status_code: StatusCode::OK,
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_service = MockAskImamAdminService::new();
            if let Some(expected_db_response) = test_case.expected_db_response {
                mock_service
                    .expect_provide_answer_to_question()
                    .returning(move |_, _| expected_db_response);
            }
            let arc_service: Arc<dyn AskImamAdminService> = Arc::new(mock_service);
            let app_state = ServiceAppState {
                service: arc_service,
            };
            let actual_response = provide_answer_for_imam_question(
                State(app_state),
                Claims::default(),
                Json(test_case.request),
            )
            .await;
            assert_eq!(test_case.expected_status_code, actual_response.status());
        }
    }

    #[tokio::test]
    async fn test_delete_imam_question() {
        struct TestCase {
            description: &'static str,
            question_id: i32,
            expected_db_response: Option<Result<(), DeleteQuestionError>>,
            expected_status_code: StatusCode,
        }
        let test_cases = [
            TestCase {
                description: "When the JSON request is invalid, I should get a BAD_REQUEST response",
                question_id: 0,
                expected_db_response: None,
                expected_status_code: StatusCode::BAD_REQUEST,
            },
            TestCase {
                description: "When deleting a non-existent question, I should get a NOT_FOUND response",
                question_id: 1,
                expected_db_response: Some(Err(DeleteQuestionError::QuestionNotFound)),
                expected_status_code: StatusCode::NOT_FOUND,
            },
            TestCase {
                description: "When deletion fails, I should get an INTERNAL_SERVER_ERROR response",
                question_id: 1,
                expected_db_response: Some(Err(DeleteQuestionError::UnableToDeleteQuestion)),
                expected_status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                description: "When deletion succeeds, I should get an OK response",
                question_id: 1,
                expected_db_response: Some(Ok(())),
                expected_status_code: StatusCode::OK,
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_service = MockAskImamAdminService::new();
            if let Some(expected_db_response) = test_case.expected_db_response {
                mock_service
                    .expect_delete_question()
                    .returning(move |_| expected_db_response);
            }
            let arc_respository: Arc<dyn AskImamAdminService> = Arc::new(mock_service);
            let app_state = ServiceAppState {
                service: arc_respository,
            };
            let actual_response = delete_imam_question(
                State(app_state),
                Claims::default(),
                Path(test_case.question_id),
            )
            .await;
            assert_eq!(test_case.expected_status_code, actual_response.status());
        }
    }
}
