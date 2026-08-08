mod features;
mod shared;

use crate::features::ask_imam::repositories::new_imam_questions_admin_repository;
use crate::features::events::endpoints::{delete_event, get_events, upsert_events};
use crate::features::events::repositories::new_events_admin_repository;
use crate::features::prayer_times::repositories::new_prayer_times_admin_repository;
use crate::features::user_authentication;
use crate::features::user_authentication::repositories::new_user_repository;

use crate::features::ask_imam::endpoints::delete_imam_question::delete_imam_question;
use crate::features::ask_imam::endpoints::get_imam_questions::get_imam_questions;
use crate::features::ask_imam::endpoints::provide_answer_for_imam_question::provide_answer_for_imam_question;
use crate::features::ask_imam::endpoints::{
    delete_imam_question, get_imam_questions, provide_answer_for_imam_question,
};
use crate::features::ask_imam::services::{AskImamAdminService, new_ask_imam_admin_service};
use crate::features::events::services::event_deletion_service::{
    EventDeletionService, new_event_deletion_service,
};
use crate::features::events::services::event_publishing_service::{
    EventPublishingService, new_event_publishing_service,
};
use crate::features::prayer_times::endpoints::get_prayer_times::get_prayer_times;
use crate::features::prayer_times::endpoints::update_prayer_times::update_prayer_times;
use crate::features::prayer_times::services::prayer_times_update_service::PrayerTimesUpdateService;
use crate::features::prayer_times::services::prayer_times_update_service::new_prayer_times_update_service;
use crate::features::user_authentication::services::login_service::new_login_service;
use crate::features::user_authentication::services::reset_password_service::new_reset_password_service;
use crate::features::user_authentication::services::user_registration_service::new_user_registration_service;
use axum::Router;
use axum::routing::{delete, get, patch, post, put};
use masjid_app_api_library::features::events::services::event_retrieval_service::{
    EventRetrievalService, new_event_retrieval_service,
};
use masjid_app_api_library::features::prayer_times::services::prayer_times_retrieval_service::{
    PrayerTimesRetrievalService, new_prayer_times_retrieval_service,
};
use masjid_app_api_library::shared::data_access::db_providers::in_memory_db_provider::InMemoryDbProvider;
use masjid_app_api_library::shared::data_access::db_providers::normal_db_provider::NormalDbProvider;
use masjid_app_api_library::shared::data_access::repository_management::repository_mode::RepositoryMode;
use masjid_app_api_library::shared::logging::logging;
use masjid_app_api_library::shared::services::hashing::providers::HashingProvider;
use masjid_app_api_library::shared::services::hashing::r#trait::new_hashing_service;
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;

async fn map_user_authentication() -> Router {
    let login_app_state = ServiceAppState {
        service: new_login_service(
            new_hashing_service(HashingProvider::Bcrypt),
            new_user_repository().await,
        ),
    };
    let user_registration_app_state = ServiceAppState {
        service: new_user_registration_service(
            new_hashing_service(HashingProvider::Bcrypt),
            new_user_repository().await,
        ),
    };
    let reset_password_app_state = ServiceAppState {
        service: new_reset_password_service(
            new_hashing_service(HashingProvider::Bcrypt),
            new_user_repository().await,
        ),
    };

    Router::new()
        .route("/login", post(user_authentication::endpoints::login))
        .with_state(login_app_state)
        .route(
            "/register-user",
            post(user_authentication::endpoints::register_user),
        )
        .with_state(user_registration_app_state)
        .route(
            "/reset-password",
            patch(user_authentication::endpoints::reset_user_password),
        )
        .with_state(reset_password_app_state)
}
async fn map_prayer_times() -> Router {
    let get_prayer_times_app_state = ServiceAppState::<Arc<dyn PrayerTimesRetrievalService>> {
        service: new_prayer_times_retrieval_service(
            new_prayer_times_admin_repository(RepositoryMode::Normal(NormalDbProvider::MySql))
                .await,
            new_prayer_times_admin_repository(RepositoryMode::InMemory(InMemoryDbProvider::Redis))
                .await,
        ),
    };
    let update_prayer_times_app_state = ServiceAppState::<Arc<dyn PrayerTimesUpdateService>> {
        service: new_prayer_times_update_service(
            new_prayer_times_admin_repository(RepositoryMode::Normal(NormalDbProvider::MySql))
                .await,
            new_prayer_times_admin_repository(RepositoryMode::InMemory(InMemoryDbProvider::Redis))
                .await,
        ),
    };
    Router::new()
        .route("/", get(get_prayer_times))
        .with_state(get_prayer_times_app_state)
        .route("/", patch(update_prayer_times))
        .with_state(update_prayer_times_app_state)
}
async fn map_donation() -> Router {
    panic!("Implement donation controller")
}
async fn map_events() -> Router {
    let get_events_app_state = ServiceAppState::<Arc<dyn EventRetrievalService>> {
        service: new_event_retrieval_service(
            new_events_admin_repository(RepositoryMode::Normal(NormalDbProvider::MySql)).await,
            new_events_admin_repository(RepositoryMode::InMemory(InMemoryDbProvider::Redis)).await,
        ),
    };

    let upsert_events_app_state = ServiceAppState::<Arc<dyn EventPublishingService>> {
        service: new_event_publishing_service(
            new_events_admin_repository(RepositoryMode::Normal(NormalDbProvider::MySql)).await,
            new_events_admin_repository(RepositoryMode::InMemory(InMemoryDbProvider::Redis)).await,
        ),
    };

    let delete_event_app_state = ServiceAppState::<Arc<dyn EventDeletionService>> {
        service: new_event_deletion_service(
            new_events_admin_repository(RepositoryMode::Normal(NormalDbProvider::MySql)).await,
            new_events_admin_repository(RepositoryMode::InMemory(InMemoryDbProvider::Redis)).await,
        ),
    };
    Router::new()
        .route("/", get(get_events))
        .with_state(get_events_app_state)
        .route("/", put(upsert_events))
        .with_state(upsert_events_app_state)
        .route("/{id}", delete(delete_event))
        .with_state(delete_event_app_state)
}
async fn map_ask_imam() -> Router {
    let state = ServiceAppState::<Arc<dyn AskImamAdminService>> {
        service: new_ask_imam_admin_service(
            new_imam_questions_admin_repository(RepositoryMode::Normal(NormalDbProvider::MySql))
                .await,
            new_imam_questions_admin_repository(RepositoryMode::InMemory(
                InMemoryDbProvider::Redis,
            ))
            .await,
        ),
    };
    Router::new()
        .route("/", get(get_imam_questions))
        .route("/", put(provide_answer_for_imam_question))
        .route("/{question_id}", delete(delete_imam_question))
        .with_state(state)
}
async fn map_endpoints() -> Router {
    let authentication_routes = map_user_authentication().await;
    tracing::info!("Mapped User Authentication Endpoints");
    let prayer_times_routes = map_prayer_times().await;
    tracing::info!("Mapped Prayer Times Endpoints");
    let events_routes = map_events().await;
    tracing::info!("Mapped Events Routes");
    let ask_imam_routes = map_ask_imam().await;
    tracing::info!("Mapped Ask Imam Routes");
    let router = Router::new();
    router
        .nest("/authentication", authentication_routes)
        .nest("/prayer-times", prayer_times_routes)
        .nest("/events", events_routes)
        .nest("/ask-imam", ask_imam_routes)
}

#[tokio::main]
async fn main() {
    logging::setup();
    tracing::info!("MasjidApp Admin Manager API starting up");
    let app = map_endpoints().await;
    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
