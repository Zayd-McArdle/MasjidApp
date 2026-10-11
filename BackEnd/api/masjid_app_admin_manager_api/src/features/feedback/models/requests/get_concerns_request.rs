use crate::features::feedback::models::filters::get_feedback_filter::GetFeedbackFilter;
use masjid_app_api_library::features::feedback::models::concern_type::ConcernType;
use masjid_app_api_library::features::feedback::models::feedback_type::FeedbackType;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetConcernsRequest {
    pub concern_type: Option<ConcernType>,
    pub date_of_feedback: Option<chrono::DateTime<chrono::Utc>>,
    pub has_images: Option<bool>,
}

impl Into<GetFeedbackFilter> for GetConcernsRequest {
    fn into(self) -> GetFeedbackFilter {
        GetFeedbackFilter {
            feedback_type: self
                .concern_type
                .map(move |concern_type| FeedbackType::Concern(concern_type)),
            date_of_feedback: self.date_of_feedback,
            has_images: self.has_images,
        }
    }
}
