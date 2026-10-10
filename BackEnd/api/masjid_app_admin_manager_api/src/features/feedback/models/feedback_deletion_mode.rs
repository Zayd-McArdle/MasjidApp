use crate::features::feedback::models::filters::delete_feedback_filter::DeleteFeedbackFilter;

pub enum FeedbackDeletionMode {
    Single(u64),
    Multiple(DeleteFeedbackFilter)
}