mod features;
mod shared;

use crate::features::ask_imam::endpoints::get_answered_questions;
use crate::features::ask_imam::repositories::new_imam_questions_public_repository;
use crate::features::ask_imam::services::new_ask_imam_public_service;
use crate::features::events::repositories::new_events_public_repository;
use crate::features::{ask_imam, events};
use axum::routing::{get, post, put};
use axum::Router;
use features::prayer_times;
use features::prayer_times::repositories::new_prayer_times_public_repository;
use masjid_app_api_library::shared::data_access::db_type::DbType;
use masjid_app_api_library::shared::data_access::repository_manager::RepositoryMode;
use masjid_app_api_library::shared::logging::logging;
use masjid_app_api_library::shared::types::app_state::{AppState, ServiceAppState};
use std::collections::HashMap;

async fn map_prayer_times() -> Router {
    let state = AppState {
        repository_map: HashMap::from([
            (
                DbType::InMemory,
                new_prayer_times_public_repository(DbType::InMemory).await,
            ),
            (
                DbType::MySql,
                new_prayer_times_public_repository(DbType::MySql).await,
            ),
        ]),
    };
    Router::new()
        .route("/", get(prayer_times::endpoints::get_prayer_times))
        .route(
            "/update",
            get(prayer_times::endpoints::get_updated_prayer_times),
        )
        .with_state(state)
}
async fn map_donation() -> Router {
    panic!("Implement donation controller")
}
async fn map_events() -> Router {
    let state = AppState {
        repository_map: HashMap::from([
            (
                DbType::InMemory,
                new_events_public_repository(DbType::InMemory).await,
            ),
            (
                DbType::MySql,
                new_events_public_repository(DbType::MySql).await,
            ),
        ]),
    };
    Router::new()
        .route("/", get(events::endpoints::get_events))
        .with_state(state)
}
async fn map_ask_imam() -> Router {
    let state = ServiceAppState {
        service: new_ask_imam_public_service(
            new_imam_questions_public_repository(RepositoryMode::Normal).await,
            new_imam_questions_public_repository(RepositoryMode::InMemory).await,
        ),
    };
    Router::new()
        .route("/", get(get_answered_questions))
        .route("/", post(ask_imam::endpoints::ask_question_for_imam))
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
