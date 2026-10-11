use masjid_app_api_library::features::feedback::models::feedback_type::FeedbackType;
use serde::Deserialize;

use crate::features::feedback::models::filters::delete_feedback_filter::DeleteFeedbackFilter;

#[derive(Debug, Deserialize, Clone)]
pub struct DeleteFeedbackRequest {
    pub feedback_type: FeedbackType,
    pub date_of_feedback: Option<chrono::DateTime<chrono::Utc>>,
}

impl Into<DeleteFeedbackFilter> for DeleteFeedbackRequest {
    fn into(self) -> DeleteFeedbackFilter {
        DeleteFeedbackFilter {
            feedback_type: self.feedback_type,
            date_of_feedback: self.date_of_feedback,
        }
    }
}
