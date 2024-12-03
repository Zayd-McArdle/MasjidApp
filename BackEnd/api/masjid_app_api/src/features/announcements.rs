use std::future::Future;
use crate::shared::repository_manager::{MainRepository, ConnectionString, RepositoryMode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use async_trait::async_trait;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use sqlx::Error;
use sqlx::mysql::MySqlQueryResult;
use validator::Validate;
use crate::shared::app_state::InnerAppState;
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
pub enum PostAnnouncementError {
    FailedToPostAnnouncement,
}
pub enum EditAnnouncementError {
    FailedToEditAnnouncement,
}
#[derive(Serialize, Deserialize, Validate)]
pub struct AnnouncementRequest {
    #[validate(length(min = 2))]
    title: String,
    description: Option<String>,
    image: Option<Vec<u8>>,
    #[validate(length(min = 2))]
    author: String
}

#[async_trait]
pub trait AnnouncementRepository {
    async fn get_announcements(&self) -> Result<Vec<AnnouncementDTO>, GetAnnouncementsError>;
    async fn post_announcement(&self, announcement: AnnouncementDTO) -> Result<(), PostAnnouncementError>;
    async fn edit_announcement(&self, announcement: AnnouncementDTO) -> Result<(), EditAnnouncementError>;
}
pub async fn new_announcement_repository(mode: RepositoryMode) -> Arc<dyn AnnouncementRepository> {
    Arc::new(MainRepository::new(ConnectionString::AnnouncementConnection).await)
}
#[async_trait]
impl AnnouncementRepository for MainRepository {
    async fn get_announcements(&self) -> Result<Vec<AnnouncementDTO>, GetAnnouncementsError> {
        let db_connection = self.db_connection.clone();
        let query_response: Result<Vec<AnnouncementDTO>, Error> = sqlx::query_as("CALL get_announcements();")
        .fetch_all(&*db_connection).await;
        match query_response {
            Ok(announcement) => Ok(announcement),
            Err(Error::RowNotFound) => Err(GetAnnouncementsError::AnnouncementsNotFound),
            Err(_) => Err(GetAnnouncementsError::UnableToFetchAnnouncements)
        }
    }

    async fn post_announcement(&self, announcement: AnnouncementDTO) -> Result<(), PostAnnouncementError> {
        let db_connection = self.db_connection.clone();
        let query_result = sqlx::query("CALL post_announcement($1, $2, $3, $4);")
            .execute(&*db_connection).await;
        match query_result {
            Ok(_) => Ok(()),
            Err(_RowNotFound) => Err(PostAnnouncementError::FailedToPostAnnouncement),
        }
    }

    async fn edit_announcement(&self, announcement: AnnouncementDTO) -> Result<(), EditAnnouncementError> {
        todo!()
    }
}

pub async fn get_announcements(State(state): State<InnerAppState<Arc<dyn AnnouncementRepository>>>) -> Response {
    match state.repositories[0].get_announcements().await {
        Ok(announcements) => (StatusCode::OK, announcements).into_response(),
        Err(GetAnnouncementsError::AnnouncementsNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(GetAnnouncementsError::UnableToFetchAnnouncements) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn post_announcement(Json(request): Json<AnnouncementRequest>, State(state): State<InnerAppState<Arc<dyn AnnouncementRepository>>>) -> Response {
    if let Err(_) = request.validate() {
        return StatusCode::BAD_REQUEST.into_response()
    }
    let announcement = AnnouncementDTO {
        title: request.title,
        description: request.description,
        image: request.image,
        author: request.author
    };
    match state.repositories[0].post_announcement(announcement).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(PostAnnouncementError::FailedToPostAnnouncement) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
pub async fn edit_announcement(Json(request): Json<AnnouncementRequest>, State(state): State<InnerAppState<Arc<dyn AnnouncementRepository>>>) -> Response {
    if let Err(_) = request.validate() {
        return StatusCode::BAD_REQUEST.into_response()
    }
    todo!()
}