use crate::features::ask_imam::models::get_imam_questions_admin_request::GetImamQuestionsAdminRequest;
use crate::features::ask_imam::models::question_status::{self, QuestionStatus};
use crate::features::ask_imam::services::AskImamAdminService;
use crate::shared::jwt::Claims;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use masjid_app_api_library::features::ask_imam::utils::send_response_for_get_imam_questions;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
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
    let get_questions_result = state.service.get_questions(request.into()).await;
    send_response_for_get_imam_questions(get_questions_result)
}
