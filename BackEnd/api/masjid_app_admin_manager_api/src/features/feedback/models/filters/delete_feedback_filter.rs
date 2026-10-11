use masjid_app_api_library::features::feedback::models::feedback_type::FeedbackType;

#[derive(Debug)]
pub struct DeleteFeedbackFilter {
    pub feedback_type: FeedbackType,
    pub date_of_feedback: Option<chrono::DateTime<chrono::Utc>>,    
}
