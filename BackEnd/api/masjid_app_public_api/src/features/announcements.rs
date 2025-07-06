/*use async_trait::async_trait;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{DateTime, Utc};
use masjid_app_api_library::shared::app_state::{AppState, DbType};
use masjid_app_api_library::shared::repository_manager::{
    InMemoryRepository, MySqlRepository, RepositoryType,
};
use mockall::automock;
use serde::{Deserialize, Serialize};
use sqlx::{Error, Row};
use std::borrow::Cow::Borrowed;
use std::sync::Arc;
use validator::Validate;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AnnouncementDTO {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub last_updated: DateTime<Utc>,
    pub image: Option<Vec<u8>>,
    pub author: String,
}
#[derive(Clone, Debug, PartialEq)]
pub enum GetAnnouncementsError {
    //Used for when a database returns no rows.
    AnnouncementsNotFound,
    //Used for when there is some database operational failure
    UnableToFetchAnnouncements,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct PostAnnouncementRequest {
    #[validate(length(min = 2))]
    title: String,
    description: Option<String>,
    image: Option<Vec<u8>>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum PostAnnouncementError {
    FailedToPostAnnouncement,
    AnnouncementAlreadyExists,
}
#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct EditAnnouncementRequest {
    #[validate(range(min = 1))]
    #[serde(rename(deserialize = "announcementId"))]
    announcement_id: i32,
    #[validate(length(min = 2))]
    title: String,
    description: Option<String>,
    image: Option<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EditAnnouncementError {
    AnnouncementDoesNotExist,
    UserDoesNotExist,
    FailedToEditAnnouncement,
}

#[automock]
#[async_trait]
pub trait AnnouncementRepository: Send + Sync {
    async fn get_announcements(&self) -> Result<Vec<AnnouncementDTO>, GetAnnouncementsError>;
    async fn post_announcement(
        &self,
        announcement: AnnouncementDTO,
    ) -> Result<(), PostAnnouncementError>;
    async fn edit_announcement(
        &self,
        announcement: AnnouncementDTO,
    ) -> Result<(), EditAnnouncementError>;
}
pub async fn new_announcement_repository(db_type: DbType) -> Arc<dyn AnnouncementRepository> {
    match db_type {
        DbType::InMemory => Arc::new(InMemoryRepository::new(RepositoryType::Announcement).await),
        DbType::MySql => Arc::new(MySqlRepository::new(RepositoryType::Announcement).await),
    }
}

#[async_trait]
impl AnnouncementRepository for InMemoryRepository {
    async fn get_announcements(&self) -> Result<Vec<AnnouncementDTO>, GetAnnouncementsError> {
        tracing::warn!("Announcement In-Memory Repository not implemented");
        Err(GetAnnouncementsError::UnableToFetchAnnouncements)
    }

    async fn post_announcement(
        &self,
        announcement: AnnouncementDTO,
    ) -> Result<(), PostAnnouncementError> {
        tracing::warn!("Announcement In-Memory Repository not implemented");
        Err(PostAnnouncementError::FailedToPostAnnouncement)
    }

    async fn edit_announcement(
        &self,
        announcement: AnnouncementDTO,
    ) -> Result<(), EditAnnouncementError> {
        tracing::warn!("Announcement In-Memory Repository not implemented");
        Err(EditAnnouncementError::FailedToEditAnnouncement)
    }
}
#[async_trait]
impl AnnouncementRepository for MySqlRepository {
    async fn get_announcements(&self) -> Result<Vec<AnnouncementDTO>, GetAnnouncementsError> {
        let db_connection = self.db_connection.clone();
        let query_response: Result<Vec<AnnouncementDTO>, Error> =
            sqlx::query("CALL get_announcements()")
                .map(|row: sqlx::mysql::MySqlRow| AnnouncementDTO {
                    id: row.get(0),
                    title: row.get(1),
                    description: row.get(2),
                    last_updated: row.get(3),
                    image: row.get(4),
                    author: row.get(5),
                })
                .fetch_all(&*db_connection)
                .await;
        match query_response {
            Ok(announcements) => {
                if announcements.is_empty() {
                    return Err(GetAnnouncementsError::AnnouncementsNotFound);
                }
                Ok(announcements)
            }
            Err(Error::RowNotFound) => Err(GetAnnouncementsError::AnnouncementsNotFound),
            Err(err) => {
                tracing::error!("failed to get announcements: {err}");
                Err(GetAnnouncementsError::UnableToFetchAnnouncements)
            }
        }
    }

    async fn post_announcement(
        &self,
        announcement: AnnouncementDTO,
    ) -> Result<(), PostAnnouncementError> {
        let db_connection = self.db_connection.clone();
        let query_result = sqlx::query("CALL post_announcement(?, ?, ?, ?);")
            .bind(&announcement.title)
            .bind(&announcement.description)
            .bind(announcement.image.clone())
            .bind(&announcement.author)
            .execute(&*db_connection)
            .await;
        match query_result {
            Ok(_) => Ok(()),
            Err(err) => {
                if let Error::Database(database_err) = &err {
                    if database_err.is_unique_violation() {
                        return Err(PostAnnouncementError::AnnouncementAlreadyExists);
                    }
                }
                tracing::error!("unable to post announcement due to the following error: {err}");
                Err(PostAnnouncementError::FailedToPostAnnouncement)
            }
        }
    }
    async fn edit_announcement(
        &self,
        announcement: AnnouncementDTO,
    ) -> Result<(), EditAnnouncementError> {
        let db_connection = self.db_connection.clone();
        let query_result = sqlx::query("CALL edit_announcement(?, ?, ?, ?, ?);")
            .bind(announcement.id)
            .bind(announcement.author)
            .bind(announcement.title)
            .bind(announcement.description)
            .bind(announcement.image)
            .execute(&*db_connection)
            .await;
        match query_result {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    return Err(EditAnnouncementError::AnnouncementDoesNotExist);
                }
                Ok(())
            }
            Err(err) => {
                if let Error::Database(database_err) = &err {
                    if database_err.code() == Some(Borrowed("1644")) {
                        return Err(EditAnnouncementError::UserDoesNotExist);
                    }
                }
                tracing::error!("unable to edit announcement due to the following error {err}");
                Err(EditAnnouncementError::FailedToEditAnnouncement)
            }
        }
    }
}

pub async fn get_announcements(
    State(state): State<AppState<Arc<dyn AnnouncementRepository>>>,
) -> Response {
    let mut get_announcements_result = state
        .repository_map
        .get(&DbType::InMemory)
        .unwrap()
        .get_announcements()
        .await;
    if get_announcements_result.is_err() {
        get_announcements_result = state
            .repository_map
            .get(&DbType::MySql)
            .unwrap()
            .get_announcements()
            .await;
    }

    match get_announcements_result {
        Ok(announcements) => (StatusCode::OK, Json(announcements)).into_response(),
        Err(GetAnnouncementsError::AnnouncementsNotFound) => StatusCode::NO_CONTENT.into_response(),
        Err(GetAnnouncementsError::UnableToFetchAnnouncements) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
pub async fn post_announcement(
    State(state): State<AppState<Arc<dyn AnnouncementRepository>>>,
    claims: Claims,
    Json(request): Json<PostAnnouncementRequest>,
) -> Response {
    if request.validate().is_err() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let announcement = AnnouncementDTO {
        id: 0,
        title: request.title,
        description: request.description,
        last_updated: DateTime::default(),
        image: request.image,
        //Get the username from the authenticated user
        author: claims.sub,
    };
    let mut post_announcement_result = state
        .repository_map
        .get(&DbType::InMemory)
        .unwrap()
        .post_announcement(announcement.clone())
        .await;
    if post_announcement_result.is_err() {
        post_announcement_result = state
            .repository_map
            .get(&DbType::MySql)
            .unwrap()
            .post_announcement(announcement)
            .await;
    }
    match post_announcement_result {
        Ok(()) => StatusCode::CREATED.into_response(),
        Err(PostAnnouncementError::AnnouncementAlreadyExists) => {
            StatusCode::CONFLICT.into_response()
        }
        Err(PostAnnouncementError::FailedToPostAnnouncement) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn edit_announcement(
    State(state): State<AppState<Arc<dyn AnnouncementRepository>>>,
    claims: Claims,
    Json(request): Json<EditAnnouncementRequest>,
) -> Response {
    if request.validate().is_err() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let edited_announcement = AnnouncementDTO {
        id: request.announcement_id,
        title: request.title,
        description: request.description,
        last_updated: Default::default(),
        image: request.image,
        author: claims.sub,
    };
    let mut edit_announcement_result = state
        .repository_map
        .get(&DbType::InMemory)
        .unwrap()
        .edit_announcement(edited_announcement.clone())
        .await;
    if edit_announcement_result.is_err() {
        edit_announcement_result = state
            .repository_map
            .get(&DbType::MySql)
            .unwrap()
            .edit_announcement(edited_announcement)
            .await;
    }
    match edit_announcement_result {
        Ok(_) => StatusCode::ACCEPTED.into_response(),
        Err(EditAnnouncementError::AnnouncementDoesNotExist) => {
            StatusCode::NOT_FOUND.into_response()
        }
        Err(
            EditAnnouncementError::FailedToEditAnnouncement
            | EditAnnouncementError::UserDoesNotExist,
        ) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    fn get_valid_claims() -> Claims {
        Claims {
            iss: "MasjidApp".to_string(),
            sub: "JohnSmith".to_string(),
            aud: "Admin".to_string(),
            iat: 1234234,
            exp: 1294234,
        }
    }
    #[tokio::test]
    async fn test_get_announcements() {
        #[derive(Clone)]
        struct TestCase {
            in_memory_announcements: Result<Vec<AnnouncementDTO>, GetAnnouncementsError>,
            announcements: Result<Vec<AnnouncementDTO>, GetAnnouncementsError>,
            expected_response_code: StatusCode,
        }
        let test_cases = vec![
            TestCase {
                in_memory_announcements: Err(GetAnnouncementsError::AnnouncementsNotFound),
                announcements: Err(GetAnnouncementsError::AnnouncementsNotFound),
                expected_response_code: StatusCode::NO_CONTENT,
            },
            TestCase {
                in_memory_announcements: Err(GetAnnouncementsError::UnableToFetchAnnouncements),
                announcements: Err(GetAnnouncementsError::UnableToFetchAnnouncements),
                expected_response_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            TestCase {
                in_memory_announcements: Ok(vec![AnnouncementDTO {
                    id: 1,
                    title: "Test announcement".to_string(),
                    description: Some("This test contains a description".to_string()),
                    last_updated: DateTime::default(),
                    image: None,
                    author: "John Smith".to_string(),
                }]),
                announcements: Ok(vec![AnnouncementDTO {
                    id: 1,
                    title: "Test announcement".to_string(),
                    description: Some("This test contains a description".to_string()),
                    last_updated: DateTime::default(),
                    image: None,
                    author: "John Smith".to_string(),
                }]),
                expected_response_code: StatusCode::OK,
            },
        ];
        for case in test_cases {
            let mut mock_announcements_in_memory_repository = MockAnnouncementRepository::new();
            let mut mock_announcements_repository = MockAnnouncementRepository::new();
            mock_announcements_in_memory_repository
                .expect_get_announcements()
                .returning(move || case.in_memory_announcements.clone());
            mock_announcements_repository
                .expect_get_announcements()
                .returning(move || case.announcements.clone());
            let arc_in_memory_repository: Arc<dyn AnnouncementRepository> =
                Arc::new(mock_announcements_in_memory_repository);
            let arc_repository = Arc::new(mock_announcements_repository);
            let app_state: AppState<Arc<dyn AnnouncementRepository>> = AppState {
                repository_map: HashMap::from([
                    (DbType::InMemory, arc_in_memory_repository),
                    (DbType::MySql, arc_repository),
                ]),
            };
            let actual_response = get_announcements(State(app_state)).await;
            assert_eq!(case.expected_response_code, actual_response.status())
        }
    }
    #[tokio::test]
    async fn test_post_announcement() {
        let valid_announcement_request = PostAnnouncementRequest {
            title: "Test announcement".to_string(),
            description: Some("This test contains a description".to_string()),
            image: None,
        };
        #[derive(Clone)]
        struct TestCase {
            announcement_request: PostAnnouncementRequest,
            claims: Claims,
            expected_in_memory_db_response: Option<Result<(), PostAnnouncementError>>,
            expected_db_response: Option<Result<(), PostAnnouncementError>>,
            expected_response_code: StatusCode,
        }
        let test_cases = vec![
            //Given the request is not valid
            TestCase {
                announcement_request: PostAnnouncementRequest {
                    title: "".to_string(),
                    description: None,
                    image: None,
                },
                claims: Claims::default(),
                expected_in_memory_db_response: None,
                expected_db_response: None,
                expected_response_code: StatusCode::BAD_REQUEST,
            },
            //Given the request is valid, but cannot insert into the database
            TestCase {
                announcement_request: valid_announcement_request.clone(),
                claims: get_valid_claims(),
                expected_in_memory_db_response: Some(Err(
                    PostAnnouncementError::FailedToPostAnnouncement,
                )),
                expected_db_response: Some(Err(PostAnnouncementError::FailedToPostAnnouncement)),
                expected_response_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            //Given the request is valid and the announcement was successfully posted
            TestCase {
                announcement_request: valid_announcement_request.clone(),
                claims: get_valid_claims(),
                expected_in_memory_db_response: Some(Ok(())),
                expected_db_response: Some(Ok(())),
                expected_response_code: StatusCode::CREATED,
            },
        ];
        for case in test_cases {
            let mut mock_announcements_in_memory_repository = MockAnnouncementRepository::new();
            let mut mock_announcements_repository = MockAnnouncementRepository::new();

            if let Some(expected_in_memory_db_response) = case.expected_in_memory_db_response {
                mock_announcements_in_memory_repository
                    .expect_post_announcement()
                    .returning(move |announcement| expected_in_memory_db_response.clone());
            }
            if let Some(expected_db_response) = case.expected_db_response {
                mock_announcements_repository
                    .expect_post_announcement()
                    .returning(move |announcement| expected_db_response.clone());
            }

            let arc_in_memory_repository: Arc<dyn AnnouncementRepository> =
                Arc::new(mock_announcements_in_memory_repository);
            let arc_repository: Arc<dyn AnnouncementRepository> =
                Arc::new(mock_announcements_repository);
            let app_state: AppState<Arc<dyn AnnouncementRepository>> = AppState {
                repository_map: HashMap::from([
                    (DbType::InMemory, arc_in_memory_repository),
                    (DbType::MySql, arc_repository),
                ]),
            };
            let actual_response = post_announcement(
                State(app_state),
                case.claims,
                Json(case.announcement_request),
            );
        }
    }

    #[tokio::test]
    async fn edit_announcement_error() {
        let valid_request = EditAnnouncementRequest {
            announcement_id: 4,
            title: "My edited Announcement".to_owned(),
            description: Some("My edited description".to_owned()),
            image: None,
        };
        #[derive(Clone)]
        struct TestCase {
            announcement_request: EditAnnouncementRequest,
            claims: Claims,
            expected_in_memory_db_response: Option<Result<(), EditAnnouncementError>>,
            expected_db_response: Option<Result<(), EditAnnouncementError>>,
            expected_response_code: StatusCode,
        }
        let test_cases = vec![
            //Given the request is not valid
            TestCase {
                announcement_request: EditAnnouncementRequest {
                    announcement_id: 0,
                    title: "".to_string(),
                    description: None,
                    image: None,
                },
                claims: Claims::default(),
                expected_in_memory_db_response: None,
                expected_db_response: None,
                expected_response_code: StatusCode::BAD_REQUEST,
            },
            //Given the request is valid but cannot inserted edited announcement into database
            TestCase {
                announcement_request: valid_request.clone(),
                claims: get_valid_claims(),
                expected_in_memory_db_response: Some(Err(
                    EditAnnouncementError::FailedToEditAnnouncement,
                )),
                expected_db_response: Some(Err(EditAnnouncementError::FailedToEditAnnouncement)),
                expected_response_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            //Given the request is valid, but post does not exists
            TestCase {
                announcement_request: valid_request.clone(),
                claims: get_valid_claims(),
                expected_in_memory_db_response: Some(Err(
                    EditAnnouncementError::AnnouncementDoesNotExist,
                )),
                expected_db_response: Some(Err(EditAnnouncementError::AnnouncementDoesNotExist)),
                expected_response_code: StatusCode::NOT_FOUND,
            },
            //Given the request is valid, I should get a successful response
            TestCase {
                announcement_request: valid_request.clone(),
                claims: get_valid_claims(),
                expected_in_memory_db_response: Some(Ok(())),
                expected_db_response: Some(Ok(())),
                expected_response_code: StatusCode::CREATED,
            },
        ];
    }
}
*/
