use crate::features::ask_imam::models::get_imam_question_admin_request::GetImamQuestionsAdminRequest;
use crate::features::ask_imam::models::question_status::QuestionStatus;
use crate::features::ask_imam::services::AskImamAdminService;
use crate::shared::jwt::Claims;
use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use masjid_app_api_library::features::ask_imam::models::imam_question_dto::ImamQuestionDTO;
use masjid_app_api_library::features::ask_imam::models::school_of_thought::SchoolOfThought;
use masjid_app_api_library::features::ask_imam::utils::send_response_for_get_imam_questions;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::str::FromStr;
use std::sync::Arc;
use validator::Validate;

pub async fn get_imam_questions(
    State(state): State<ServiceAppState<Arc<dyn AskImamAdminService>>>,
    _claims: Claims,
    Query(request): Query<GetImamQuestionsAdminRequest>,
) -> Result<Json<Vec<ImamQuestionDTO>>, StatusCode> {
    request.validate().map_err(|_| StatusCode::BAD_REQUEST)?;
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
