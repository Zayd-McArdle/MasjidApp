mod features;
mod shared;

use crate::features::ask_imam::endpoints::ask_question_for_imam::ask_question_for_imam;
use crate::features::ask_imam::endpoints::get_answered_questions::get_answered_questions;
use crate::features::ask_imam::repositories::new_imam_questions_public_repository;
use crate::features::ask_imam::services::new_ask_imam_public_service;
use crate::features::events::events_public_repository::new_events_public_repository;
use crate::features::prayer_times::endpoints::get_prayer_times::get_prayer_times;
use crate::features::prayer_times::endpoints::get_updated_prayer_times::get_updated_prayer_times;
use crate::features::prayer_times::services::prayer_times_update_checking_service::{
    PrayerTimesUpdateCheckingService, new_prayer_times_update_checking_service,
};
use crate::features::{ask_imam, events};
use axum::Router;
use axum::routing::{get, post};
use features::prayer_times;
use features::prayer_times::repositories::new_prayer_times_public_repository;
use masjid_app_api_library::features::events::endpoints::get_events::get_events_common;
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
use masjid_app_api_library::shared::types::app_state::ServiceAppState;
use std::sync::Arc;

async fn map_prayer_times() -> Router {
    let get_prayer_times_app_state = ServiceAppState::<Arc<dyn PrayerTimesRetrievalService>> {
        service: new_prayer_times_retrieval_service(
            new_prayer_times_public_repository(RepositoryMode::Normal(NormalDbProvider::MySql))
                .await,
            new_prayer_times_public_repository(RepositoryMode::InMemory(InMemoryDbProvider::Redis))
                .await,
        ),
    };
    let get_updated_prayer_times_app_state = ServiceAppState::<
        Arc<dyn PrayerTimesUpdateCheckingService>,
    > {
        service: new_prayer_times_update_checking_service(
            new_prayer_times_public_repository(RepositoryMode::Normal(NormalDbProvider::MySql))
                .await,
            new_prayer_times_public_repository(RepositoryMode::InMemory(InMemoryDbProvider::Redis))
                .await,
        ),
    };
    Router::new()
        .route("/", get(get_prayer_times))
        .with_state(get_prayer_times_app_state)
        .route("/update", get(get_updated_prayer_times))
        .with_state(get_updated_prayer_times_app_state)
}
async fn map_donation() -> Router {
    panic!("Implement donation controller")
}
async fn map_events() -> Router {
    let state = ServiceAppState::<Arc<dyn EventRetrievalService>> {
        service: new_event_retrieval_service(
            new_events_public_repository(RepositoryMode::Normal(NormalDbProvider::MySql)).await,
            new_events_public_repository(RepositoryMode::InMemory(InMemoryDbProvider::Redis)).await,
        ),
    };
    Router::new()
        .route("/", get(get_events_common))
        .with_state(state)
}
async fn map_ask_imam() -> Router {
    let state = ServiceAppState {
        service: new_ask_imam_public_service(
            new_imam_questions_public_repository(RepositoryMode::Normal(NormalDbProvider::MySql))
                .await,
            new_imam_questions_public_repository(RepositoryMode::InMemory(
                InMemoryDbProvider::Redis,
            ))
            .await,
        ),
    };
    Router::new()
        .route("/", get(get_answered_questions))
        .route("/", post(ask_question_for_imam))
        .with_state(state)
}

async fn map_endpoints() -> Router {
    let prayer_times_routes = map_prayer_times().await;
    tracing::info!("Mapped Prayer Times Endpoints");
    let event_routes = map_events().await;
    tracing::info!("Mapped Events Endpoints");
    let ask_imam_routes = map_ask_imam().await;
    tracing::info!("Mapped Ask Imam Endpoints");

    let router = Router::new();
    router
        .nest("/prayer-times", prayer_times_routes)
        .nest("/events", event_routes)
        .nest("/ask-imam", ask_imam_routes)
}

#[tokio::main]
async fn main() {
    logging::setup();

    tracing::info!("MasjidApp Public API initialised");
    let app = map_endpoints().await;
    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
