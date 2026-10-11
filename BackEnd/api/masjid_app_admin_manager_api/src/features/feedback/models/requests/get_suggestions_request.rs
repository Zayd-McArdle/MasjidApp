use crate::features::feedback::models::filters::get_feedback_filter::GetFeedbackFilter;
use masjid_app_api_library::features::feedback::models::feedback_type::FeedbackType;
use masjid_app_api_library::features::feedback::models::suggestion_type::SuggestionType;

pub struct GetSuggestionsRequest {
    pub title: Option<String>,
    pub suggestion_type: Option<SuggestionType>,
    pub date_of_feedback: Option<chrono::DateTime<chrono::Utc>>,
    pub has_images: bool,
}

impl Into<GetFeedbackFilter> for GetSuggestionsRequest {
    #[inline]
    fn into(self) -> GetFeedbackFilter {
        GetFeedbackFilter {
            title: self.title,
            feedback_type: self
                .suggestion_type
                .map(|suggestion_type| FeedbackType::Suggestion(suggestion_type)),
            date_of_feedback: self.date_of_feedback,
            has_images: self.has_images,
        }
    }
}
