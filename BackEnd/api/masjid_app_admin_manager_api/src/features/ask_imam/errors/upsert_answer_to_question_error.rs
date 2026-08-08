#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UpsertAnswerToQuestionError {
    QuestionNotFound,
    UnableToUpsertAnswerToQuestion,
}
