use serde::{Deserialize, Serialize};

use crate::features::feedback::models::concern_type::ConcernType;
use crate::features::feedback::models::suggestion_type::SuggestionType;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum FeedbackType {
    Suggestion(SuggestionType),
    Concern(ConcernType),
}
