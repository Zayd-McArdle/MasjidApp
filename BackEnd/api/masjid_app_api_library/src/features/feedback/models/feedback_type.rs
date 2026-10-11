use crate::features::feedback::models::concern_type::ConcernType;
use crate::features::feedback::models::feedback_type_error::FeedbackTypeError;
use crate::features::feedback::models::suggestion_type::SuggestionType;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
const SUGGESTION: &'static str = "Suggestion: ";
const CONCERN: &'static str = "Concern: ";
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum FeedbackType {
    Suggestion(SuggestionType),
    Concern(ConcernType),
}

impl Display for FeedbackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let feedback_type_str = match self {
            FeedbackType::Suggestion(suggestion) => {
                format!("{SUGGESTION}{}", suggestion.to_string())
            }
            FeedbackType::Concern(concern) => {
                format!("{CONCERN}{}", concern.to_string())
            }
        };
        write!(f, "{}", feedback_type_str)
    }
}

impl FromStr for FeedbackType {
    type Err = FeedbackTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with(SUGGESTION) {
            Ok(FeedbackType::Suggestion(
                SuggestionType::from_str(&s.replace(SUGGESTION, "")).unwrap(),
            ))
        } else if s.starts_with(CONCERN) {
            Ok(FeedbackType::Concern(
                ConcernType::from_str(&s.replace(CONCERN, "")).unwrap(),
            ))
        } else {
            Err(FeedbackTypeError::InvalidType(s.to_owned()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::feedback::models::concern_type::NEIGHBOUR_DISRUPTION;
    use crate::features::feedback::models::constants::MANAGEMENT;
    use crate::features::feedback::models::facility::Facility;
    use crate::features::feedback::models::facility::Facility::WudhuArea;
    use crate::features::feedback::models::facility::KITCHEN;
    use crate::features::feedback::models::facility::SINKS;

    #[test]
    fn test_feedback_type_display() {
        struct TestCase {
            feedback_type: FeedbackType,
            expected_str: String,
        }

        let test_cases = [
            TestCase {
                feedback_type: FeedbackType::Suggestion(SuggestionType::Management),
                expected_str: format!("{SUGGESTION}{MANAGEMENT}"),
            },
            TestCase {
                feedback_type: FeedbackType::Suggestion(SuggestionType::Facilities(
                    Facility::Sinks,
                )),
                expected_str: format!("{SUGGESTION}{SINKS}"),
            },
            TestCase {
                feedback_type: FeedbackType::Suggestion(SuggestionType::Other(
                    "risk-assessment".to_owned(),
                )),
                expected_str: format!("{SUGGESTION}risk-assessment"),
            },
            TestCase {
                feedback_type: FeedbackType::Concern(ConcernType::NeighbourDisruption),
                expected_str: format!("{CONCERN}{NEIGHBOUR_DISRUPTION}"),
            },
            TestCase {
                feedback_type: FeedbackType::Concern(ConcernType::Facilities(Facility::Kitchen)),
                expected_str: format!("{CONCERN}{KITCHEN}"),
            },
            TestCase {
                feedback_type: FeedbackType::Concern(ConcernType::Other("flooding".to_owned())),
                expected_str: format!("{CONCERN}flooding"),
            },
        ];
        for test_case in test_cases {
            let actual_str = test_case.feedback_type.to_string();
            assert_eq!(test_case.expected_str, actual_str);
        }
    }

    #[test]
    fn test_feedback_type_from_str() {
        struct TestCase {
            description: &'static str,
            feedback_type_str: &'static str,
            expected_result: Result<FeedbackType, FeedbackTypeError>,
        }
        let test_cases = [
            TestCase {
                description: "When entering an empty feedback_type string, I should get an error",
                feedback_type_str: "",
                expected_result: Err(FeedbackTypeError::InvalidType("".to_owned())),
            },
            TestCase {
                description: "When entering a bad feedback_type string, I should get an error",
                feedback_type_str: "bad string",
                expected_result: Err(FeedbackTypeError::InvalidType("bad string".to_owned())),
            },
            TestCase {
                description: "When entering a management suggestion in feedback_type, I should get no error",
                feedback_type_str: "Suggestion: Management",
                expected_result: Ok(FeedbackType::Suggestion(SuggestionType::Management)),
            },
            TestCase {
                description: "When entering a facilities suggestion in feedback_type, I should get no error",
                feedback_type_str: "Suggestion: Toilets",
                expected_result: Ok(FeedbackType::Suggestion(SuggestionType::Facilities(
                    Facility::Toilets,
                ))),
            },
            TestCase {
                description: "When entering a custom suggestion in feedback_type, I should get no error",
                feedback_type_str: "Suggestion: special suggestion",
                expected_result: Ok(FeedbackType::Suggestion(SuggestionType::Other(
                    "special suggestion".to_owned(),
                ))),
            },
            TestCase {
                description: "When entering a neighbour disruption concern in feedback_type, I should get no error",
                feedback_type_str: "Concern: Neighbour Disruption",
                expected_result: Ok(FeedbackType::Concern(ConcernType::NeighbourDisruption)),
            },
            TestCase {
                description: "When entering a wudhu area concern in feedback_type, I should get no error",
                feedback_type_str: "Concern: Wudhu Area",
                expected_result: Ok(FeedbackType::Concern(ConcernType::Facilities(WudhuArea))),
            },
            TestCase {
                description: "When entering a custom concern in feedback_type, I should get no error",
                feedback_type_str: "Concern: ceiling rupture",
                expected_result: Ok(FeedbackType::Concern(ConcernType::Other(
                    "ceiling rupture".to_owned(),
                ))),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let actual_result = FeedbackType::from_str(test_case.feedback_type_str);
            assert_eq!(test_case.expected_result, actual_result);
        }
    }
}
