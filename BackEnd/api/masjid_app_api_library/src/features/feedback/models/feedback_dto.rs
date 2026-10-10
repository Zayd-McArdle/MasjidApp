use crate::features::feedback::models::feedback_type::FeedbackType;
use serde::Serialize;
#[derive(Serialize)]
pub struct FeedbackDTO {
    pub id: u64,
    pub title: String,
    pub feedback_type: FeedbackType,
    pub details: String,
    pub images: Option<Vec<String>>,
    pub date: chrono::DateTime<chrono::Utc>,
}