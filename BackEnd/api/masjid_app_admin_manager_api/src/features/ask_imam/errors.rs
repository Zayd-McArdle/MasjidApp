#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UpsertAnswerToQuestionError {
    QuestionNotFound,
    UnableToUpsertAnswerToQuestion,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeleteQuestionError {
    QuestionNotFound,
    UnableToDeleteQuestion,
}
