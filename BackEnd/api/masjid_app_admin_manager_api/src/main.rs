mod features;
mod shared;

use crate::features::events;
use crate::features::user_authentication::repository::new_user_repository;
use axum::routing::{delete, get, patch, post, put};
use axum::Router;
use features::prayer_times::new_prayer_times_admin_repository;
use features::user_authentication::repository::UserRepository;
use features::user_authentication::endpoints;
use features::{announcements, prayer_times, user_authentication};
use masjid_app_api_library::shared::data_access::db_type::DbType;
use masjid_app_api_library::shared::types::app_state::AppState;
use std::collections::HashMap;
use crate::features::events::endpoints::{delete_event, get_events, upsert_events};
use crate::features::events::repository::new_events_admin_repository;

async fn map_user_authentication() -> Router {
    let state = AppState {
        repository_map: HashMap::from([(DbType::MySql, new_user_repository().await)]),
    };

    Router::new()
        .route("/login", post(user_authentication::endpoints::login))
        .route("/register-user", post(user_authentication::endpoints::register_user))
        .route(
            "/reset-password",
            patch(user_authentication::endpoints::reset_user_password),
        )
        .with_state(state)
}
/*async fn map_announcements() -> Router {
    let state = AppState {
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
        .with_state(state)
}*/
async fn map_prayer_times() -> Router {
    let state = AppState {
        repository_map: HashMap::from([
            (
                DbType::InMemory,
                new_prayer_times_admin_repository(DbType::InMemory).await,
            ),
            (
                DbType::MySql,
                new_prayer_times_admin_repository(DbType::MySql).await,
            ),
        ]),
    };
    Router::new()
        .route("/", get(prayer_times::get_prayer_times))
        .route("/", patch(prayer_times::update_prayer_times))
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
                new_events_admin_repository(DbType::InMemory).await,
            ),
            (
                DbType::MySql,
                new_events_admin_repository(DbType::MySql).await,
            ),
        ]),
    };
    Router::new()
        .route("/", get(get_events))
        .route("/", put(upsert_events))
        .route("/{id}", delete(delete_event))
        .with_state(state)
}
async fn map_endpoints() -> Router {
    let authentication_routes = map_user_authentication().await;
    tracing::info!("Mapped User Authentication Endpoints");
    let prayer_times_routes = map_prayer_times().await;
    tracing::info!("Mapped Prayer Times Endpoints");
    let events_routes = map_events().await;
    tracing::info!("Mapped Events Routes");
    let router = Router::new();
    router
        .nest("/authentication", authentication_routes)
        .nest("/prayer-times", prayer_times_routes)
        .nest("/events", events_routes)
    //.nest("/announcements", announcements_routes)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt().json().init();

    tracing::info!("MasjidApp Admin Manager API starting up");
    let app = map_endpoints().await;
    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
