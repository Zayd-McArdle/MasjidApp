use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, Clone, Eq, PartialEq)]
pub struct Answer {
    #[serde(rename = "imamName")]
    pub imam_name: String,

    pub text: String,
    #[serde(rename = "dateAnswered")]
    pub date_answered: chrono::DateTime<chrono::Utc>,
}
