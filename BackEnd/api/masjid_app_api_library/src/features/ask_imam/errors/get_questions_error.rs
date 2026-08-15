#[derive(Debug)]
pub enum GetQuestionsError {
    QuestionsNotFound,
    UnableToGetAnsweredQuestions,
}
