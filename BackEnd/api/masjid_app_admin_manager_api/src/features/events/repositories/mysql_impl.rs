use crate::features::events::repositories::EventsAdminRepository;
use crate::features::events::repositories::errors::delete_event_error::DeleteEventError;
use crate::features::events::repositories::errors::insert_event_error::InsertEventError;
use crate::features::events::repositories::errors::update_event_error::UpdateEventError;
use crate::features::events::repositories::errors::upsert_event_error::UpsertEventError;
use async_trait::async_trait;
use masjid_app_api_library::features::events::models::event::Event;
use masjid_app_api_library::shared::data_access::repository_management::mysql_repository::MySqlRepository;
use sqlx::Row;

#[async_trait]
impl EventsAdminRepository for MySqlRepository {
    async fn upsert_event(&self, event: &Event) -> Result<(), UpsertEventError> {
        let db_connection = self.db_connection.clone();
        let result = sqlx::query("CALL upsert_event(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(&event.id)
            .bind(&event.title)
            .bind(&event.description)
            .bind(&event.date)
            .bind(&event.r#type)
            .bind(&event.recurrence)
            .bind(&event.status)
            .bind(&event.minimum_age)
            .bind(&event.maximum_age)
            .bind(&event.image_url)
            .bind(&event.full_name)
            .bind(&event.phone_number)
            .bind(&event.email)
            .execute(&*db_connection)
            .await
            .map_err(|err| {
                if let sqlx::Error::Database(ref database_error) = err {
                    if database_error.message() == "Event already exists" {
                        return UpsertEventError::InsertError(InsertEventError::EventAlreadyExists);
                    }
                }
                tracing::error!("Unable to upsert event due to the following error: {}", err);
                UpsertEventError::InsertError(InsertEventError::UnableToInsertEvent)
            })?;
        if result.rows_affected() == 0 {
            tracing::debug!(id = &event.id, "record not found");
            return Err(UpsertEventError::UpdateError(
                UpdateEventError::EventNotFound,
            ));
        }
        Ok(())
    }

    async fn delete_event_by_id(&self, event_id: &i32) -> Result<Option<String>, DeleteEventError> {
        let db_connection = self.db_connection.clone();
        let mut image_url: Option<String> = None;

        match sqlx::query("CALL retrieve_image_url_by_event_id(?)")
            .bind(&event_id)
            .fetch_optional(&*db_connection)
            .await
        {
            Ok(url) => image_url = url.and_then(|row| row.get(0)),
            Err(err) => {
                tracing::error!(
                    "unable to retrieve image url for event id {}, due to the following error: {}",
                    event_id,
                    err
                )
            }
        }

        let query_result = sqlx::query("CALL delete_event_by_id(?)")
            .bind(&event_id)
            .execute(&*db_connection)
            .await
            .map_err(|err| {
                tracing::error!("failed to delete event due to the following error: {}", err);
                DeleteEventError::UnableToDeleteEvent
            })?;
        if query_result.rows_affected() == 0 {
            tracing::debug!("event id {} not found in the database", event_id);
            return Err(DeleteEventError::EventNotFound);
        }
        Ok(image_url)
    }
}
