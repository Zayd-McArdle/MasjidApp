use std::sync::Arc;

use async_trait::async_trait;
use axum::body::Body;
use axum::Router;
use axum::routing::{get, patch, post};
use crate::features::announcements::announcements;
use crate::features::announcements::announcements::{new_announcement_repository, AnnouncementRepository};
use crate::features::{prayer_times, user_authentication};
use crate::features::user_authentication::user_repository;
use crate::shared::app_state::AppState;

#[async_trait]
trait ControllerMapper {
    fn map_user_authentication(&self) -> Router;
    fn map_announcements(&self) -> Router<Body>;
    fn map_prayer_times(&self) -> Router<Body>;
    fn map_donation(&self) -> Router<Body>;
    fn map_events(&self) -> Router<Body>;
    fn map_classes(&self) -> Router<Body>;

}

impl ControllerMapper for Router {
     async fn map_user_authentication(self) -> Router<Body> {
        let app_state = AppState {
          user_repository: user_repository::new_user_repository().await,
            announcement_repository: new_announcement_repository()
        };
        self.route("/authentication/login", post(user_authentication::login::login))
            .route("/authentication/register-user", post(user_authentication::registration::register_user))
            .route("/authentication/reset-password", patch(user_authentication::reset_password::reset_user_password))
            .with_state::<AppState>(&app_state)
    }
    fn map_announcements(self) -> Router {
        self.route("/announcements", get(announcements::get_announcements))
            .route("/announcements", post(announcements::post_announcement))
            .route("/announcements", patch(announcements::edit_announcement))
    }
    fn map_prayer_times(self) -> Router {
        self.route("/prayer-times", get(prayer_times::get_prayer_times))
            .route("/prayer-times", patch(prayer_times::update_prayer_times_file))
    }
    fn map_donation(self) -> Router {
        panic!("Implement donation controller")
    }
    fn map_events(self) -> Router {
        panic!("Implement events controller")
    }
    fn map_classes(self) -> Router {
        panic!("Implement classes controller")
    }
}


pub async fn map_endpoints() -> Router<Body> {
    let router = Router::new();
    router.map_user_authentication()
        .map_announcements()
        .map_prayer_times()
        .map_donation()
        .map_donation()
        .map_events()
        .map_classes()
}


pub async fn run() {
    let app = map_endpoints().await;
}