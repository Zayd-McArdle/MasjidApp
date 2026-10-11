use crate::features::feedback::models::feedback_dto::FeedbackDTO;
use crate::features::feedback::models::feedback_type::FeedbackType;
use crate::features::feedback::models::feedback_type_error::FeedbackTypeError;
use std::str::FromStr;

pub struct Feedback {
    pub id: u64,
    pub title: String,
    pub feedback_type: String,
    pub details: String,
    pub date: chrono::DateTime<chrono::offset::Utc>,
    pub images: Option<String>,
}

impl TryInto<FeedbackDTO> for Feedback {
    type Error = FeedbackTypeError;
    #[inline]
    fn try_into(self) -> Result<FeedbackDTO, Self::Error> {
        let feedback_type = FeedbackType::from_str(&self.feedback_type)?;
        Ok(FeedbackDTO {
            id: self.id,
            title: self.title,
            feedback_type,
            details: self.details,
            date: self.date,
            images: self
                .images
                .map(move |images| images.split(';').map(str::to_owned).collect()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::feedback::models::suggestion_type::SuggestionType;

    #[test]
    fn test_feedback_try_into() {
        struct TestCase {
            description: &'static str,
            feedback: Feedback,
            expected_result: Result<FeedbackDTO, FeedbackTypeError>,
        }
        let test_cases = [
            TestCase {
                description: "When entering a bad feedback_type string, I should get an error",
                feedback: Feedback {
                    id: 0,
                    title: "".to_owned(),
                    feedback_type: "bad string".to_owned(),
                    details: "".to_owned(),
                    date: Default::default(),
                    images: None,
                },
                expected_result: Err(FeedbackTypeError::InvalidType("bad string".to_owned())),
            },
            TestCase {
                description: "When entering adding a single image, I should get one image",
                feedback: Feedback {
                    id: 0,
                    title: "".to_owned(),
                    feedback_type: "Suggestion: Management".to_owned(),
                    details: "".to_owned(),
                    date: Default::default(),
                    images: Some("google.com".to_owned()),
                },
                expected_result: Ok(FeedbackDTO {
                    id: 0,
                    title: "".to_owned(),
                    feedback_type: FeedbackType::Suggestion(SuggestionType::Management),
                    details: "".to_owned(),
                    images: Some(Box::new(["google.com".to_owned()])),
                    date: Default::default(),
                }),
            },
            TestCase {
                description: "When entering adding multiple images, I should get multiple images",
                feedback: Feedback {
                    id: 0,
                    title: "".to_owned(),
                    feedback_type: "Suggestion: Management".to_owned(),
                    details: "".to_owned(),
                    date: Default::default(),
                    images: Some("google.com;bing.com;youtube.com".to_owned()),
                },
                expected_result: Ok(FeedbackDTO {
                    id: 0,
                    title: "".to_owned(),
                    feedback_type: FeedbackType::Suggestion(SuggestionType::Management),
                    details: "".to_owned(),
                    images: Some(Box::new([
                        "google.com".to_owned(),
                        "bing.com".to_owned(),
                        "youtube.com".to_owned(),
                    ])),
                    date: Default::default(),
                }),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let actual_result = test_case.feedback.try_into();
            assert_eq!(test_case.expected_result, actual_result);
        }
    }
}
