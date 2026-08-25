use enum_stringify::EnumStringify;
use serde::Deserialize;

#[derive(Deserialize, EnumStringify)]
#[enum_stringify(case = "lower")]
pub enum QuestionStatus {
    Unanswered,
    Answered,
    All,
}

impl Default for QuestionStatus {
    fn default() -> Self {
        Self::All
    }
}
