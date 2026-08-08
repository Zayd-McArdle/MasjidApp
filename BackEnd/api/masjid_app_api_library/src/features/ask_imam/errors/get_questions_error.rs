#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GetQuestionsError {
    QuestionsNotFound,
    UnableToGetAnsweredQuestions,
}
