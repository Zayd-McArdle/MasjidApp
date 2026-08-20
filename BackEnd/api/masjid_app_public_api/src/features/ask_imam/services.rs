use crate::features::ask_imam::errors::insert_imam_question_error::InsertImamQuestionError;
use crate::features::ask_imam::repositories::ImamQuestionsPublicRepository;
use async_trait::async_trait;
use masjid_app_api_library::features::ask_imam::errors::get_questions_error::GetQuestionsError;
use masjid_app_api_library::features::ask_imam::models::get_imam_questions_filter::GetImamQuestionsFilter;
use masjid_app_api_library::features::ask_imam::models::imam_question::ImamQuestion;
use masjid_app_api_library::features::ask_imam::models::imam_question_dto::ImamQuestionDTO;
use masjid_app_api_library::features::ask_imam::services::AskImamServiceImpl;
use mockall::automock;
use std::sync::Arc;

#[automock]
#[async_trait]
pub trait AskImamPublicService: Send + Sync {
    async fn get_answered_questions(
        &self,
        filter: GetImamQuestionsFilter,
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
        filter: GetImamQuestionsFilter,
    ) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError> {
        match self.in_memory_repository.get_questions(&filter).await {
            Ok(answers) => Ok(answers),
            Err(_) => self.repository.get_questions(&filter).await,
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
    use masjid_app_api_library::features::ask_imam::models::answer::Answer;
    use masjid_app_api_library::features::ask_imam::models::imam_question::ImamQuestion;
    use masjid_app_api_library::features::ask_imam::models::imam_question_dto::ImamQuestionDTO;
    use masjid_app_api_library::features::ask_imam::models::school_of_thought::SchoolOfThought;
    use masjid_app_api_library::features::ask_imam::repositories::ImamQuestionsRepository;
    use mockall::mock;

    mock!(
        pub ImamQuestionsPublicRepository {}

        #[async_trait]
        impl ImamQuestionsRepository for ImamQuestionsPublicRepository {
            async fn get_questions(&self, filter: &GetImamQuestionsFilter) -> Result<Vec<ImamQuestionDTO>, GetQuestionsError>;
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
            mock_in_memory_repository_result: Result<Vec<ImamQuestionDTO>, GetQuestionsError>,
            mock_repository_result: Result<Vec<ImamQuestionDTO>, GetQuestionsError>,
            expected_result: Result<Vec<ImamQuestionDTO>, GetQuestionsError>,
        }
        const TOPIC: &'static str = "Specific topic";
        let mock_all_answered_questions = get_mock_answered_questions();

        let test_cases = [
            TestCase {
                description: "When question retrieval fails in all repositories, I should get an error",
                mock_in_memory_repository_result: Err(
                    GetQuestionsError::UnableToGetAnsweredQuestions,
                ),
                mock_repository_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
            },
            TestCase {
                description: "When no questions were returned from all repositories, I should receive an error",
                mock_in_memory_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                mock_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                expected_result: Err(GetQuestionsError::QuestionsNotFound),
            },
            TestCase {
                description: "When questions failed to be retrieved from in-memory repository but questions not found in main repository, I should receive an error",
                mock_in_memory_repository_result: Err(
                    GetQuestionsError::UnableToGetAnsweredQuestions,
                ),
                mock_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                expected_result: Err(GetQuestionsError::QuestionsNotFound),
            },
            TestCase {
                description: "When questions not found in in-memory repository but questions failed to be retrieved from main repository, I should receive an error",
                mock_in_memory_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                mock_repository_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
            },
            TestCase {
                description: "When questions returned from in-memory database, I should receive no error",
                mock_in_memory_repository_result: Ok(mock_all_answered_questions.clone()),
                mock_repository_result: Err(GetQuestionsError::UnableToGetAnsweredQuestions),
                expected_result: Ok(mock_all_answered_questions.clone()),
            },
            TestCase {
                description: "When questions not found in in-memory repository but found in main repository, I should receive no error",
                mock_in_memory_repository_result: Err(GetQuestionsError::QuestionsNotFound),
                mock_repository_result: Ok(mock_all_answered_questions.clone()),
                expected_result: Ok(mock_all_answered_questions),
            },
        ];

        for test_case in test_cases {
            eprintln!("{}", test_case.description);
            let mut mock_in_memory_repository = MockImamQuestionsPublicRepository::new();
            let mut mock_repository = MockImamQuestionsPublicRepository::new();

            mock_in_memory_repository
                .expect_get_questions()
                .return_once(move |_| test_case.mock_in_memory_repository_result);
            mock_repository
                .expect_get_questions()
                .return_once(move |_| test_case.mock_repository_result);

            let arc_repository: Arc<dyn ImamQuestionsPublicRepository> = Arc::new(mock_repository);
            let arc_in_memory_repository: Arc<dyn ImamQuestionsPublicRepository> =
                Arc::new(mock_in_memory_repository);

            let service = new_ask_imam_public_service(arc_repository, arc_in_memory_repository);
            let _actual_result = service
                .get_answered_questions(GetImamQuestionsFilter::default())
                .await;
            assert!(matches!(test_case.expected_result, _actual_result));
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
                question,
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
            assert!(matches!(test_case.expected_result, actual_result));
        }
    }
}
