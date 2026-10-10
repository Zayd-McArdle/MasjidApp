use crate::features::feedback::models::complaint_type::ComplaintType;
use crate::features::feedback::models::feedback_dto::FeedbackDTO;
use crate::features::feedback::models::feedback_type::FeedbackType;
use crate::features::feedback::models::feedback_type_error::FeedbackTypeError;
use crate::features::feedback::models::suggestion_type::SuggestionType;
use std::str::FromStr;

const SUGGESTION: &'static str = "Suggestion";
const COMPLAINT: &'static str = "Complaint";

pub struct Feedback {
    pub id: u64,
    pub title: String,
    pub feedback_type: String,
    pub suggestion_type: Option<String>,
    pub complaint_type: Option<String>,
    pub details: String,
    pub date: chrono::DateTime<chrono::offset::Utc>,
    pub images: Option<Vec<String>>,
}

impl TryInto<FeedbackDTO> for Feedback {
    type Error = FeedbackTypeError;

    fn try_into(self) -> Result<FeedbackDTO, Self::Error> {
        let feedback_type = match self.feedback_type.as_str() {
            SUGGESTION => Ok(FeedbackType::Suggestion(
                SuggestionType::from_str(&self.suggestion_type.unwrap()).unwrap(),
            )),
            COMPLAINT => Ok(FeedbackType::Complaint(
                ComplaintType::from_str(&self.complaint_type.unwrap()).unwrap(),
            )),
            "" => Err(FeedbackTypeError::Empty),
            _ => Err(FeedbackTypeError::InvalidType(self.feedback_type)),
        }?;
        Ok(FeedbackDTO {
            id: self.id,
            title: self.title,
            feedback_type,
            details: self.details,
            date: self.date,
            images: self.images,
        })
    }
}
