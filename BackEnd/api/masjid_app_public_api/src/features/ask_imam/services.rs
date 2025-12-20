use crate::features::ask_imam::errors::InsertImamQuestionError;
use crate::features::ask_imam::repositories::ImamQuestionsPublicRepository;
use async_trait::async_trait;
use masjid_app_api_library::features::ask_imam::errors::GetQuestionsError;
use masjid_app_api_library::features::ask_imam::models::{
    ImamQuestion, ImamQuestionDTO, SchoolOfThought,
};
use masjid_app_api_library::features::ask_imam::services::AskImamServiceImpl;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait AskImamPublicService: Send + Sync {
    async fn get_answered_questions(
        &self,
        topic: Option<String>,
        school_of_thought: Option<SchoolOfThought>,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
    async fn ask_question(&self, question: ImamQuestion) -> Result<(), InsertImamQuestionError>;
}

pub fn new_ask_imam_public_service(
    repository: Arc<dyn ImamQuestionsPublicRepository>,
    in_memory_repository: Arc<dyn ImamQuestionsPublicRepository>,
) -> Arc<dyn AskImamPublicService> {
    Arc::new(AskImamServiceImpl {
        repository,
        in_memory_repository,
    })
}
#[async_trait]
impl AskImamPublicService for AskImamServiceImpl<dyn ImamQuestionsPublicRepository> {
    async fn get_answered_questions(
        &self,
        topic: Option<String>,
        school_of_thought: Option<SchoolOfThought>,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        match (topic, school_of_thought) {
            (None, None) => self
                .in_memory_repository
                .get_answered_questions()
                .await
                .or(self.repository.get_answered_questions().await),
            (Some(topic), None) => self
                .in_memory_repository
                .get_answered_questions_by_topic(&topic)
                .await
                .or(self
                    .repository
                    .get_answered_questions_by_topic(&topic)
                    .await),
            (None, Some(school_of_thought)) => self
                .in_memory_repository
                .get_answered_questions_by_school_of_thought(school_of_thought)
                .await
                .or(self
                    .repository
                    .get_answered_questions_by_school_of_thought(school_of_thought)
                    .await),
            (Some(topic), Some(school_of_thought)) => self
                .in_memory_repository
                .get_answered_questions_by_topic_and_school_of_thought(&topic, school_of_thought)
                .await
                .or(self
                    .repository
                    .get_answered_questions_by_topic_and_school_of_thought(
                        &topic,
                        school_of_thought,
                    )
                    .await),
        }
    }
    async fn ask_question(&self, question: ImamQuestion) -> Result<(), InsertImamQuestionError> {
        let insert_question_result = self
            .in_memory_repository
            .insert_question_for_imam(&question)
            .await;
        if insert_question_result.is_err() {
            tracing::warn!("insertion of question into in-memory database failed");
        }
        self.repository.insert_question_for_imam(&question).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use masjid_app_api_library::features::ask_imam::errors::GetQuestionsError;
    use masjid_app_api_library::features::ask_imam::models::{
        Answer, ImamQuestionDTO, SchoolOfThought,
    };
    use masjid_app_api_library::features::ask_imam::repositories::ImamQuestionsRepository;
    use mockall::mock;
    use std::str::FromStr;

    mock!(
        pub ImamQuestionsPublicRepository {}

        #[async_trait]
        impl ImamQuestionsRepository for ImamQuestionsPublicRepository {
            async fn get_answered_questions(&self) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
            async fn get_answered_questions_by_topic(
                &self,
                topic: &str,
            ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
            async fn get_answered_questions_by_school_of_thought(
                &self,
                school_of_thought: SchoolOfThought,
            ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
            async fn get_answered_questions_by_topic_and_school_of_thought(
                &self,
                topic: &str,
                school_of_thought: SchoolOfThought,
            ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
        }
        #[async_trait]
        impl ImamQuestionsPublicRepository for ImamQuestionsPublicRepository {
            async fn insert_question_for_imam(
                &self,
                questions: &ImamQuestion,
            ) -> Result<(), InsertImamQuestionError>;
        }
    );

    fn get_mock_answered_questions() -> Vec<ImamQuestionDTO> {
        vec![
            ImamQuestionDTO {
                id: 1,
                title: "question 1".to_string(),
                topic: "N/A".to_string(),
                school_of_thought: None,
                description: "This is a description".to_string(),
                date_of_question: Default::default(),
                answer: Some(Answer {
                    imam_name: "Zayd".to_string(),
                    text: "This is an answer".to_string(),
                    date_answered: Default::default(),
                }),
            },
            ImamQuestionDTO {
                id: 2,
                title: "question 2".to_string(),
                topic: "N/A".to_string(),
                school_of_thought: None,
                description: "This is a description".to_string(),
                date_of_question: Default::default(),
                answer: Some(Answer {
                    imam_name: "Zayd".to_string(),
                    text: "This is an answer".to_string(),
                    date_answered: Default::default(),
                }),
            },
            ImamQuestionDTO {
                id: 3,
                title: "question 3".to_string(),
                topic: "Specific topic".to_string(),
                school_of_thought: None,
                description: "This is a description".to_string(),
                date_of_question: Default::default(),
                answer: Some(Answer {
                    imam_name: "Zayd".to_string(),
                    text: "This is an answer".to_string(),
                    date_answered: Default::default(),
                }),
            },
            ImamQuestionDTO {
                id: 4,
                title: "question 4".to_string(),
                topic: "N/A".to_string(),
                school_of_thought: Some(SchoolOfThought::Hanafi),
                description: "This is a description".to_string(),
                date_of_question: Default::default(),
                answer: Some(Answer {
                    imam_name: "Zayd".to_string(),
                    text: "This is an answer".to_string(),
                    date_answered: Default::default(),
                }),
            },
            ImamQuestionDTO {
                id: 5,
                title: "question 5".to_string(),
                topic: "Specific topic".to_string(),
                school_of_thought: Some(SchoolOfThought::Hanafi),
                description: "This is a description".to_string(),
                date_of_question: Default::default(),
                answer: Some(Answer {
                    imam_name: "Zayd".to_string(),
                    text: "This is an answer".to_string(),
                    date_answered: Default::default(),
                }),
            },
        ]
    }
    #[tokio::test]
    async fn test_get_answered_question() {
        struct TestCase {
            description: &'static str,
            topic: Option<String>,
            school_of_thought: Option<String>,
            mock_in_memory_repository_result: Result<Vec<ImamQuestionDTO>, GetQuestionsError>,
            mock_repository_result: Result<Vec<ImamQuestionDTO>, GetQuestionsError>,
            expected_result: Result<Vec<ImamQuestionDTO>, GetQuestionsError>,
        }
        const TOPIC: &'static str = "Specific topic";
        let mock_all_answered_questions = get_mock_answered_questions();
        let mock_answered_questions_by_topic = mock_all_answered_questions
            .clone()
            .into_iter()
            .filter(|question| question.topic == "Specific topic")
            .collect::<Vec<ImamQuestionDTO>>();
        let mock_answered_questions_by_school_of_thought = mock_all_answered_questions
            .clone()
            .into_iter()
            .filter(|question| question.school_of_thought == Some(SchoolOfThought::Hanafi))
            .collect::<Vec<ImamQuestionDTO>>();
        let mock_answered_questions_by_topic_and_school_of_thought = mock_all_answered_questions
            .clone()
            .into_iter()
            .filter(|question| {
                question.topic == "Specific topic"
                    && question.school_of_thought == Some(SchoolOfThought::Hanafi)
            })
            .collect::<Vec<ImamQuestionDTO>>();
        let test_cases = [
            TestCase {
                description: "When no filters are applied and question retrieval fails in all repositories, I should get an error",
                topic: None,
                school_of_thought: None,
                mock_in_memory_repository_result: Err(
                    GetQuestionsError::UnableToGetAnsweredQuestions,
                ),
                mock_repository_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
            },
            TestCase {
                description: "When no filters applied and no questions were returned from all repositories, I should receive an error",
                topic: None,
                school_of_thought: None,
                mock_in_memory_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                mock_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                expected_result: Err(GetQuestionsError::QuestionsNotFound),
            },
            TestCase {
                description: "When no filters applied and questions failed to be retrieved from in-memory repository but questions not found in main repository, I should receive an error",
                topic: None,
                school_of_thought: None,
                mock_in_memory_repository_result: Err(
                    GetQuestionsError::UnableToGetAnsweredQuestions,
                ),
                mock_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                expected_result: Err(GetQuestionsError::QuestionsNotFound),
            },
            TestCase {
                description: "When no filters applied and questions not found in in-memory repository but questions failed to be retrieved from main repository, I should received an error",
                topic: None,
                school_of_thought: None,
                mock_in_memory_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                mock_repository_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
            },
            TestCase {
                description: "When no filters applied and questions returned from in-memory database, I should receive no error",
                topic: None,
                school_of_thought: None,
                mock_in_memory_repository_result: Ok(mock_all_answered_questions.clone()),
                mock_repository_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_result: Ok(mock_all_answered_questions),
            },
            TestCase {
                description: "When topic filter applied and question retrieval fails in all repositories, I should get an error",
                topic: Some(TOPIC.to_owned()),
                school_of_thought: None,
                mock_in_memory_repository_result: Err(
                    GetQuestionsError::UnableToGetAnsweredQuestions,
                ),
                mock_repository_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
            },
            TestCase {
                description: "When topic filter applied and no questions were returned from all repositories, I should receive an error",
                topic: Some(TOPIC.to_owned()),
                school_of_thought: None,
                mock_in_memory_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                mock_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                expected_result: Err(GetQuestionsError::QuestionsNotFound),
            },
            TestCase {
                description: "When topic filter applied and questions failed to be retrieved from in-memory repository but questions not found in main repository, I should receive an error",
                topic: Some(TOPIC.to_owned()),
                school_of_thought: None,
                mock_in_memory_repository_result: Err(
                    GetQuestionsError::UnableToGetAnsweredQuestions,
                ),
                mock_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                expected_result: Err(GetQuestionsError::QuestionsNotFound),
            },
            TestCase {
                description: "When topic filter applied and questions not found in in-memory repository but questions failed to be retrieved from main repository, I should received an error",
                topic: Some(TOPIC.to_owned()),
                school_of_thought: None,
                mock_in_memory_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                mock_repository_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
            },
            TestCase {
                description: "When topic filter applied and questions returned from in-memory database, I should receive no error",
                topic: Some(TOPIC.to_owned()),
                school_of_thought: None,
                mock_in_memory_repository_result: Ok(mock_answered_questions_by_topic.clone()),
                mock_repository_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_result: Ok(mock_answered_questions_by_topic),
            },
            TestCase {
                description: "When school of thought filter applied and question retrieval fails in all repositories, I should get an error",
                topic: None,
                school_of_thought: Some(SchoolOfThought::Hanafi.to_string()),
                mock_in_memory_repository_result: Err(
                    GetQuestionsError::UnableToGetAnsweredQuestions,
                ),
                mock_repository_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
            },
            TestCase {
                description: "When school of thought filter applied and no questions were returned from all repositories, I should receive an error",
                topic: None,
                school_of_thought: Some(SchoolOfThought::Hanafi.to_string()),
                mock_in_memory_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                mock_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                expected_result: Err(GetQuestionsError::QuestionsNotFound),
            },
            TestCase {
                description: "When school of thought filter applied and questions failed to be retrieved from in-memory repository but questions not found in main repository, I should receive an error",
                topic: None,
                school_of_thought: Some(SchoolOfThought::Hanafi.to_string()),
                mock_in_memory_repository_result: Err(
                    GetQuestionsError::UnableToGetAnsweredQuestions,
                ),
                mock_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                expected_result: Err(GetQuestionsError::QuestionsNotFound),
            },
            TestCase {
                description: "When school of thought filter applied and questions not found in in-memory repository but questions failed to be retrieved from main repository, I should received an error",
                topic: None,
                school_of_thought: Some(SchoolOfThought::Hanafi.to_string()),
                mock_in_memory_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                mock_repository_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
            },
            TestCase {
                description: "When school of thought filter applied and questions returned from in-memory database, I should receive no error",
                topic: None,
                school_of_thought: Some(SchoolOfThought::Hanafi.to_string()),
                mock_in_memory_repository_result: Ok(
                    mock_answered_questions_by_school_of_thought.clone()
                ),
                mock_repository_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_result: Ok(mock_answered_questions_by_school_of_thought),
            },
            TestCase {
                description: "When all filters applied and question retrieval fails in all repositories, I should get an error",
                topic: Some(TOPIC.to_owned()),
                school_of_thought: Some(SchoolOfThought::Hanafi.to_string()),
                mock_in_memory_repository_result: Err(
                    GetQuestionsError::UnableToGetAnsweredQuestions,
                ),
                mock_repository_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
            },
            TestCase {
                description: "When all filters applied and no questions were returned from all repositories, I should receive an error",
                topic: Some(TOPIC.to_owned()),
                school_of_thought: Some(SchoolOfThought::Hanafi.to_string()),
                mock_in_memory_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                mock_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                expected_result: Err(GetQuestionsError::QuestionsNotFound),
            },
            TestCase {
                description: "When all filters applied and questions failed to be retrieved from in-memory repository but questions not found in main repository, I should receive an error",
                topic: Some(TOPIC.to_owned()),
                school_of_thought: Some(SchoolOfThought::Hanafi.to_string()),
                mock_in_memory_repository_result: Err(
                    GetQuestionsError::UnableToGetAnsweredQuestions,
                ),
                mock_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                expected_result: Err(GetQuestionsError::QuestionsNotFound),
            },
            TestCase {
                description: "When all filters applied and questions not found in in-memory repository but questions failed to be retrieved from main repository, I should received an error",
                topic: Some(TOPIC.to_owned()),
                school_of_thought: Some(SchoolOfThought::Hanafi.to_string()),
                mock_in_memory_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                mock_repository_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
            },
            TestCase {
                description: "When all filters applied and questions returned from in-memory database, I should receive no error",
                topic: Some(TOPIC.to_owned()),
                school_of_thought: Some(SchoolOfThought::Hanafi.to_string()),
                mock_in_memory_repository_result: Ok(
                    mock_answered_questions_by_topic_and_school_of_thought.clone(),
                ),
                mock_repository_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_result: Ok(mock_answered_questions_by_topic_and_school_of_thought),
            },
        ];

        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_in_memory_repository = MockImamQuestionsPublicRepository::new();
            let mut mock_repository = MockImamQuestionsPublicRepository::new();

            match (test_case.topic.clone(), test_case.school_of_thought.clone()) {
                (None, None) => {
                    mock_in_memory_repository
                        .expect_get_answered_questions()
                        .return_once(|| test_case.mock_in_memory_repository_result);
                    mock_repository
                        .expect_get_answered_questions()
                        .return_once(|| test_case.mock_repository_result);
                }
                (Some(_topic), None) => {
                    mock_in_memory_repository
                        .expect_get_answered_questions_by_topic()
                        .return_once(|_| test_case.mock_in_memory_repository_result);
                    mock_repository
                        .expect_get_answered_questions_by_topic()
                        .return_once(|_| test_case.mock_repository_result);
                }
                (None, Some(_school_of_thought)) => {
                    mock_in_memory_repository
                        .expect_get_answered_questions_by_school_of_thought()
                        .return_once(|_| test_case.mock_in_memory_repository_result);
                    mock_repository
                        .expect_get_answered_questions_by_school_of_thought()
                        .return_once(|_| test_case.mock_repository_result);
                }
                (Some(_topic), Some(_school_of_thought)) => {
                    mock_in_memory_repository
                        .expect_get_answered_questions_by_topic_and_school_of_thought()
                        .return_once(|_, _| test_case.mock_in_memory_repository_result);
                    mock_repository
                        .expect_get_answered_questions_by_topic_and_school_of_thought()
                        .return_once(|_, _| test_case.mock_repository_result);
                }
            }
            let arc_repository: Arc<dyn ImamQuestionsPublicRepository> = Arc::new(mock_repository);
            let arc_in_memory_repository: Arc<dyn ImamQuestionsPublicRepository> =
                Arc::new(mock_in_memory_repository);

            let service = new_ask_imam_public_service(arc_repository, arc_in_memory_repository);
            let actual_result = service
                .get_answered_questions(
                    test_case.topic,
                    test_case.school_of_thought.and_then(|school_of_thought| {
                        SchoolOfThought::from_str(&school_of_thought).ok()
                    }),
                )
                .await;
            assert_eq!(test_case.expected_result, actual_result);
        }
    }
    #[tokio::test]
    async fn test_ask_question() {
        struct TestCase {
            description: &'static str,
            question: ImamQuestion,
            mock_in_memory_repository_result: Result<(), InsertImamQuestionError>,
            mock_repository_result: Result<(), InsertImamQuestionError>,
            expected_result: Result<(), InsertImamQuestionError>,
        }
        let question = ImamQuestion {
            id: 0,
            title: "".to_string(),
            topic: "".to_string(),
            school_of_thought: None,
            description: "".to_string(),
            date_of_question: Default::default(),
            imam_name: None,
            answer: None,
            date_answered: None,
        };
        let test_cases = [
            TestCase {
                description: "When insertion fails on both repositories, I should receive an error",
                question: question.clone(),
                mock_in_memory_repository_result: Err(
                    InsertImamQuestionError::UnableToInsertQuestion,
                ),
                mock_repository_result: Err(InsertImamQuestionError::UnableToInsertQuestion),
                expected_result: Err(InsertImamQuestionError::UnableToInsertQuestion),
            },
            TestCase {
                description: "When insertion succeeds for in-memory repository but not on main repository, I should receive an error",
                question: question.clone(),
                mock_in_memory_repository_result: Ok(()),
                mock_repository_result: Err(InsertImamQuestionError::UnableToInsertQuestion),
                expected_result: Err(InsertImamQuestionError::UnableToInsertQuestion),
            },
            TestCase {
                description: "When insertion fails for in-memory repository but succeeds on main repository, I should receive no error",
                question: question.clone(),
                mock_in_memory_repository_result: Err(
                    InsertImamQuestionError::UnableToInsertQuestion,
                ),
                mock_repository_result: Ok(()),
                expected_result: Ok(()),
            },
            TestCase {
                description: "When insertion succeeds for both repositories, I should receive no error",
                question: question,
                mock_in_memory_repository_result: Ok(()),
                mock_repository_result: Ok(()),
                expected_result: Ok(()),
            },
        ];
        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_in_memory_repository = MockImamQuestionsPublicRepository::new();
            let mut mock_repository = MockImamQuestionsPublicRepository::new();

            mock_in_memory_repository
                .expect_insert_question_for_imam()
                .return_once(move |_| test_case.mock_in_memory_repository_result);
            mock_repository
                .expect_insert_question_for_imam()
                .return_once(move |_| test_case.mock_repository_result);
            let mock_repository: Arc<dyn ImamQuestionsPublicRepository> = Arc::new(mock_repository);
            let mock_in_memory_repository: Arc<dyn ImamQuestionsPublicRepository> =
                Arc::new(mock_in_memory_repository);
            let actual_result =
                new_ask_imam_public_service(mock_repository, mock_in_memory_repository)
                    .ask_question(test_case.question)
                    .await;
            assert_eq!(test_case.expected_result, actual_result);
        }
    }
}
