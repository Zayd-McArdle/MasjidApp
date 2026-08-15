#[derive(Debug)]
pub enum DeleteQuestionError {
    QuestionNotFound,
    UnableToDeleteQuestion,
}
