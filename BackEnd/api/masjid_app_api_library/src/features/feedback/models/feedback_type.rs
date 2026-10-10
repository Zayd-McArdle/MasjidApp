use serde::{Deserialize, Serialize};

use crate::features::feedback::models::complaint_type::ComplaintType;
use crate::features::feedback::models::suggestion_type::SuggestionType;

#[derive(Deserialize, Serialize)]
pub enum FeedbackType {
    Suggestion(SuggestionType),
    Complaint(ComplaintType),
}
