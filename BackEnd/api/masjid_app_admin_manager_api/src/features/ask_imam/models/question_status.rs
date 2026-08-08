use enum_stringify::EnumStringify;
use serde::Deserialize;

#[derive(Deserialize, EnumStringify)]
#[enum_stringify(case = "lower")]
pub enum QuestionStatus {
    Unanswered,
    Answered,
}
