use std::sync::Arc;

use async_trait::async_trait;
use axum::body::Body;
use axum::Router;
use axum::routing::{get, patch, post};
use crate::features::announcements;
use crate::features::announcements::{new_announcement_repository, AnnouncementRepository};
use crate::features::{prayer_times, user_authentication};
use crate::features::prayer_times::new_prayer_times_repository;
use crate::features::user_authentication::{new_user_repository, UserRepository};
use crate::shared::app_state::{InnerAppState, OuterAppState};
use crate::shared::repository_manager::RepositoryMode;

#[async_trait]
trait ControllerMapper {
    fn map_user_authentication(&self) -> Router;
    fn map_announcements(&self) -> Router<Body>;
    fn map_prayer_times(&self) -> Router<Body>;
    fn map_donation(&self) -> Router<Body>;
    fn map_events(&self) -> Router<Body>;
    fn map_classes(&self) -> Router<Body>;

}

    async fn map_user_authentication() -> Router {
        let state = InnerAppState {
            repositories: vec![new_user_repository(RepositoryMode::Normal).await],
        
        };
         Router::new()
             .route("/authentication/login", post(user_authentication::login))
             .route("/authentication/register-user", post(user_authentication::register_user))
             .route("/authentication/reset-password", patch(user_authentication::reset_user_password))
             .with_state(state)
    }
    async fn map_announcements() -> Router {
        let state = InnerAppState {
            repositories: vec![new_announcement_repository(RepositoryMode::Normal).await],
        };
        Router::new()
            .route("/announcements", get(announcements::get_announcements))
            .route("/announcements", post(announcements::post_announcement))
            .route("/announcements", patch(announcements::edit_announcement))
            .with_state(state)
    }
    async fn map_prayer_times() -> Router {
        let state = InnerAppState {
            repositories: vec![new_prayer_times_repository(RepositoryMode::Normal).await],
        };
        Router::new()
            .route("/prayer-times", get(prayer_times::get_prayer_times))
            .route("/prayer-times", patch(prayer_times::update_prayer_times))
            .with_state(state)
    }
    fn map_donation() -> Router {
        panic!("Implement donation controller")
    }
    fn map_events() -> Router {
        panic!("Implement events controller")
    }
    fn map_classes() -> Router {
        panic!("Implement classes controller")
    }


pub async fn map_endpoints() -> Router<Body> {

    let authentication_routes = map_user_authentication().await;
    let prayer_times_routes = map_prayer_times().await;
    let announcements_routes = map_donation().await;
    let router = Router::new();
    router.merge(authentication_routes)
        .merge(prayer_times_routes)
        .merge(announcements_routes)
        .with_state(OuterAppState{
            user_authentication_state: InnerAppState {},
            prayer_times_state: InnerAppState {},
            announcements_state: InnerAppState {},
        })
}


pub async fn run() {
    let app = map_endpoints().await;
}