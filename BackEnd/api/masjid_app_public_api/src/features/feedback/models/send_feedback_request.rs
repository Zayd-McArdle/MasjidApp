use masjid_app_api_library::features::feedback::models::feedback::Feedback;
use masjid_app_api_library::features::feedback::models::feedback_type::FeedbackType;
use serde::Deserialize;
use sqlx::types::chrono;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct SendFeedbackRequest {
    #[validate(length(min = 5, max = 50))]
    pub title: String,
    pub feedback_type: FeedbackType,
    #[validate(length(min = 4, max = 500))]
    pub details: String,
    pub images: Option<Box<[String]>>,
}

impl Into<Feedback> for SendFeedbackRequest {
    fn into(self) -> Feedback {
        Feedback {
            id: 0,
            title: self.title,
            feedback_type: self.feedback_type.to_string(),
            details: self.details,
            date: chrono::DateTime::default(),
            images: self.images.and_then(|images| {
                if images.is_empty() {
                    return None;
                }
                Some(images.join(";"))
            }),
        }
    }
}
