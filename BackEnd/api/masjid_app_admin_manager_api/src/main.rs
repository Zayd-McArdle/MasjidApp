mod features;
mod shared;

use crate::features::ask_imam::repositories::new_imam_questions_admin_repository;
use crate::features::events::endpoints::{delete_event, get_events, upsert_events};
use crate::features::events::repositories::new_events_admin_repository;
use crate::features::prayer_times::repositories::new_prayer_times_admin_repository;
use crate::features::user_authentication::repositories::new_user_repository;
use crate::features::{prayer_times, user_authentication};

use crate::features::ask_imam::endpoints::{
    delete_imam_question, get_imam_questions, provide_answer_for_imam_question,
};
use crate::features::ask_imam::services::{new_ask_imam_admin_service, AskImamAdminService};
use axum::routing::{delete, get, patch, post, put};
use axum::Router;
use masjid_app_api_library::shared::data_access::db_type::DbType;
use masjid_app_api_library::shared::data_access::repository_manager::RepositoryMode;
use masjid_app_api_library::shared::logging::logging;
use masjid_app_api_library::shared::types::app_state::{AppState, ServiceAppState};
use std::collections::HashMap;
use std::sync::Arc;

async fn map_user_authentication() -> Router {
    let state = AppState {
        repository_map: HashMap::from([(DbType::MySql, new_user_repository().await)]),
    };

    Router::new()
        .route("/login", post(user_authentication::endpoints::login))
        .route(
            "/register-user",
            post(user_authentication::endpoints::register_user),
        )
        .route(
            "/reset-password",
            patch(user_authentication::endpoints::reset_user_password),
        )
        .with_state(state)
}
async fn map_prayer_times() -> Router {
    let state = AppState {
        repository_map: HashMap::from([
            (DbType::InMemory, new_prayer_times_admin_repository().await),
            (DbType::MySql, new_prayer_times_admin_repository().await),
        ]),
    };
    Router::new()
        .route("/", get(prayer_times::endpoints::get_prayer_times))
        .route("/", patch(prayer_times::endpoints::update_prayer_times))
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
                new_events_admin_repository(RepositoryMode::InMemory).await,
            ),
            (
                DbType::MySql,
                new_events_admin_repository(RepositoryMode::Normal).await,
            ),
        ]),
    };
    Router::new()
        .route("/", get(get_events))
        .route("/", put(upsert_events))
        .route("/{id}", delete(delete_event))
        .with_state(state)
}
async fn map_ask_imam() -> Router {
    let state = ServiceAppState::<Arc<dyn AskImamAdminService>> {
        service: new_ask_imam_admin_service(
            new_imam_questions_admin_repository(RepositoryMode::Normal).await,
            new_imam_questions_admin_repository(RepositoryMode::InMemory).await,
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
