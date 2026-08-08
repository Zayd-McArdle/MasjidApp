#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeleteQuestionError {
    QuestionNotFound,
    UnableToDeleteQuestion,
}
