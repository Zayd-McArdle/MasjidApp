mod features;
mod shared;

use crate::features::events;
use crate::features::events::repository::new_events_public_repository;
use axum::routing::{get, patch, post, put};
use axum::Router;
use features::prayer_times::new_prayer_times_public_repository;
use features::{announcements, prayer_times};
use masjid_app_api_library::shared::data_access::db_type::DbType;
use masjid_app_api_library::shared::types::app_state::AppState;
use std::collections::HashMap;

async fn map_announcements() -> Router {
    todo!();
    /*let state = AppState {
        repository_map: HashMap::from([
            (
                DbType::InMemory,
                new_announcement_repository(DbType::InMemory).await,
            ),
            (
                DbType::MySql,
                new_announcement_repository(DbType::MySql).await,
            ),
        ]),
    };
    Router::new()
        .route("/", get(announcements::get_announcements))
        .route("/", post(announcements::post_announcement))
        .route("/", put(announcements::edit_announcement))
        .with_state(state)*/
}
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
        .route("/", get(prayer_times::get_prayer_times))
        .route("/update", get(prayer_times::get_updated_prayer_times))
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
async fn map_classes() -> Router {
    panic!("Implement classes controller")
}

async fn map_endpoints() -> Router {
    let prayer_times_routes = map_prayer_times().await;
    tracing::info!("Mapped Prayer Times Endpoints");
    let event_routes = map_events().await;
    tracing::info!("Mapped Events Endpoints");
    //let announcements_routes = map_announcements().await;
    tracing::info!("Mapped Announcements Endpoints");
    let router = Router::new();
    router
        .nest("/prayer-times", prayer_times_routes)
        .nest("/events", event_routes)
    // .nest("/announcements", announcements_routes)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt().json().init();

    tracing::info!("MasjidApp Public API initialised");
    let app = map_endpoints().await;
    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
