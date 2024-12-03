use crate::shared::repository_manager::{Repository, ConnectionString};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use async_trait::async_trait;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use sqlx::Error;
use validator::Validate;
use crate::shared::app_state::AppState;
#[derive(sqlx::FromRow, Serialize)]
pub struct AnnouncementDTO {
    title: String,
    description: Option<String>,
    image: Option<Vec<u8>>,
    author: String
}
pub enum GetAnnouncementsError {
    //Used for when a database returns no rows.
    AnnouncementsNotFound,
    //Used for when there is some database operational failure
    UnableToFetchAnnouncements,
}
pub enum AnnouncementError {
    FailedToPostAnnouncement,
    FailedToEditAnnouncement,
    UnableToFetchAnnouncements,
    AnnouncementsNotFound,
    NewAnnouncementMatchesOldAnnouncement,
}
#[derive(Serialize, Deserialize, Validate)]
pub struct PostAnnouncementRequest {
    #[validate(length(min = 2))]
    title: String,
    description: String,
    image: Option<Vec<u8>>,
    #[validate(length(min = 2))]
    author: String
}

#[derive(Serialize, Deserialize, Validate)]
pub struct EditAnnouncementRequest {
    #[validate(length(min = 2))]
    title: String,
    description: String,
    image: Option<Vec<u8>>,
    #[validate(length(min = 2))]
    author: String
}


#[async_trait]
pub trait AnnouncementRepository {
    fn get_announcements(&self) -> Result<Vec<AnnouncementDTO>, GetAnnouncementsError>;
    fn post_announcement(&self, request: PostAnnouncementRequest) -> Result<(), AnnouncementError>;
    fn edit_announcement(&self) -> Result<(), AnnouncementError>;
}
pub fn new_announcement_repository() -> Arc<dyn AnnouncementRepository> {
    Arc::new(Repository::new(ConnectionString::AnnouncementConnection))
}
impl AnnouncementRepository for Repository {
    async fn get_announcements(&self) -> Result<Vec<AnnouncementDTO>, GetAnnouncementsError> {
        let query_response: Result<Vec<AnnouncementDTO>, Error> = sqlx::query_as("CALL get_announcements();").fetch_all(&self.db_connection).await;
        match query_response {
            Ok(announcement) => Ok(announcement),
            Err(_RowNotFound) => Err(GetAnnouncementsError::AnnouncementsNotFound),
            Err(_) => Err(GetAnnouncementsError::UnableToFetchAnnouncements)
        }
    }

    fn post_announcement(&self, request: PostAnnouncementRequest) -> Result<(), AnnouncementError> {
        todo!()
    }

    fn edit_announcement(&self) -> Result<(), AnnouncementError> {
        todo!()
    }
}

pub async fn get_announcements(State(state): State<AppState>) -> Response {
    match state.announcement_repository.get_announcements() {
        Ok(announcements) => (StatusCode::OK, announcements).into_response(),
        Err(GetAnnouncementsError::AnnouncementsNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(GetAnnouncementsError::UnableToFetchAnnouncements) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn post_announcement(Json(request): Json<PostAnnouncementRequest>, State(state): State<AppState>) -> Response {
    if let Err(_) = request.validate() {
        return StatusCode::BAD_REQUEST.into_response()
    }
    match state.announcement_repository.post_announcement(request) {
        Ok(()) => StatusCode::OK.into_response(),
        Err(AnnouncementError::FailedToPostAnnouncement) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
pub async fn edit_announcement(Json(request): Json<EditAnnouncementRequest>, State(state): State<AppState>) -> Response {
    if let Err(_) = request.validate() {
        return StatusCode::BAD_REQUEST.into_response()
    }
    todo!()
}