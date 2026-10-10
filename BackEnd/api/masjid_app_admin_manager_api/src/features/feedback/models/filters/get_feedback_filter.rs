use masjid_app_api_library::features::feedback::models::feedback_type::FeedbackType;

pub struct GetFeedbackFilter {
    pub title: Option<String>,
    pub feedback_type: Option<FeedbackType>,
    pub date_of_feedback: Option<chrono::DateTime<chrono::Utc>>,
    pub has_images: bool,
}