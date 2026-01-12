use crate::common::data_access_layer;
use crate::common::data_access_layer::DatabaseCredentials;
use crate::common::logging::setup_logging;
use masjid_app_admin_manager_api::features::ask_imam::errors::DeleteQuestionError;
use masjid_app_admin_manager_api::features::ask_imam::repositories::new_imam_questions_admin_repository;
use masjid_app_api_library::features::ask_imam::errors::GetQuestionsError;
use masjid_app_api_library::features::ask_imam::models::{
    Answer, ImamQuestion, ImamQuestionDTO, SchoolOfThought,
};
use masjid_app_api_library::shared::data_access::repository_manager::RepositoryMode;
use masjid_app_public_api::features::ask_imam::repositories::new_imam_questions_public_repository;
use sqlx::types::chrono;

#[tokio::test]
async fn test_ask_imam() {
    setup_logging();
    let container = data_access_layer::mysql::setup_container(DatabaseCredentials {
        username: "askimamadmin".to_string(),
        password: "changeme".to_string(),
        environment_variable: "ASK_IMAM_CONNECTION".to_string(),
    })
    .await;

    let public_repository = new_imam_questions_public_repository(RepositoryMode::Normal).await;
    let admin_repository = new_imam_questions_admin_repository(RepositoryMode::Normal).await;

    eprintln!(
        "Given no questions were asked for the imam, I should receive an error when deleting a question"
    );
    let delete_question_result = admin_repository
        .delete_imam_question_by_id(&1)
        .await
        .unwrap_err();
    assert_eq!(
        delete_question_result,
        DeleteQuestionError::QuestionNotFound
    );

    eprintln!("When retrieving questions from an empty database, I should receive an error");
    let get_all_questions_result = admin_repository.get_all_imam_questions().await.unwrap_err();
    assert_eq!(
        get_all_questions_result,
        GetQuestionsError::QuestionsNotFound
    );

    eprintln!("When inserting valid questions, I should receive no error");
    let valid_questions = get_valid_questions();
    for question in valid_questions.iter() {
        let insert_question_result = public_repository.insert_question_for_imam(&question).await;
        assert!(insert_question_result.is_ok());
    }

    eprintln!("When answering a question, I should receive no error");
    let date_question_answered = chrono::NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(2025, 12, 8).unwrap(),
        chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
    )
    .and_utc();
    let answer = Answer {
        imam_name: "Zayd".to_owned(),
        text: "This is an answer".to_owned(),
        date_answered: date_question_answered,
    };
    let mut upsert_answer_to_question_result = admin_repository
        .upsert_imam_answer_to_question(&1, &answer)
        .await;
    assert!(upsert_answer_to_question_result.is_ok());

    upsert_answer_to_question_result = admin_repository
        .upsert_imam_answer_to_question(&7, &answer)
        .await;
    assert!(upsert_answer_to_question_result.is_ok());

    eprintln!(
        "When attempting to retrieve all questions for imam from database, I should get all questions without error"
    );
    let get_all_questions_result = admin_repository.get_all_imam_questions().await.unwrap();
    assert_eq!(valid_questions.len(), get_all_questions_result.len());
    let mut valid_questions_dto = Vec::new();
    for valid_question in valid_questions {
        valid_questions_dto.push(ImamQuestionDTO::from(valid_question));
    }
    assert_dto_fields(&valid_questions_dto, &get_all_questions_result);

    eprintln!(
        "When attempting to retrieve unanswered questions for imam from database, I should only get unanswered questions without error"
    );
    let valid_unanswered_questions_dto: Vec<ImamQuestionDTO> = valid_questions_dto
        .clone()
        .into_iter()
        .filter(|question| question.answer.is_none())
        .collect();
    let get_unanswered_questions_result = admin_repository
        .get_unanswered_imam_questions()
        .await
        .unwrap();
    assert_eq!(
        valid_unanswered_questions_dto.len(),
        get_unanswered_questions_result.len()
    );
    assert_dto_fields(
        &valid_unanswered_questions_dto,
        &get_unanswered_questions_result,
    );
    eprintln!(
        "When attempting to retrieve unanswered questions by topic, I should only get questions for that relevant topic"
    );

    let valid_questions_by_fiqh: Vec<ImamQuestionDTO> = valid_unanswered_questions_dto
        .clone()
        .into_iter()
        .filter(|q| q.topic == "Fiqh")
        .collect();
    let get_questions_by_fiqh_result = admin_repository
        .get_unanswered_imam_questions_by_topic("Fiqh")
        .await
        .unwrap();
    assert_dto_fields(&valid_questions_by_fiqh, &get_questions_by_fiqh_result);

    eprintln!(
        "When attempting to retrieve unanswered questions by school of thought, I should only get questions for that relevant school or non-school specific"
    );

    let valid_questions_by_school_of_thought: Vec<ImamQuestionDTO> = valid_unanswered_questions_dto
        .into_iter()
        .filter(|q| {
            q.school_of_thought == Some(SchoolOfThought::Hanafi) || q.school_of_thought == None
        })
        .collect();
    let get_questions_by_school_of_thought_result = admin_repository
        .get_unanswered_imam_questions_by_school_of_thought(SchoolOfThought::Hanafi)
        .await
        .unwrap();
    assert_dto_fields(
        &valid_questions_by_school_of_thought,
        &get_questions_by_school_of_thought_result,
    );

    //
    eprintln!(
        "When attempting to retrieve answered questions for imam from database, I should only get answered questions without error"
    );
    let valid_answered_questions_dto: Vec<ImamQuestionDTO> = valid_questions_dto
        .clone()
        .into_iter()
        .filter(|question| question.answer.is_some())
        .collect();
    let get_unanswered_questions_result = public_repository.get_answered_questions().await.unwrap();
    assert_eq!(
        valid_answered_questions_dto.len(),
        get_unanswered_questions_result.len()
    );
    assert_dto_fields(
        &valid_answered_questions_dto,
        &get_unanswered_questions_result,
    );
    eprintln!(
        "When attempting to retrieve answered questions by topic, I should only get questions for that relevant topic"
    );

    let valid_questions_by_fiqh: Vec<ImamQuestionDTO> = valid_answered_questions_dto
        .clone()
        .into_iter()
        .filter(|q| q.topic == "Tafseer")
        .collect();
    let get_questions_by_fiqh_result = public_repository
        .get_answered_questions_by_topic("Tafseer")
        .await
        .unwrap();
    assert_dto_fields(&valid_questions_by_fiqh, &get_questions_by_fiqh_result);

    eprintln!(
        "When attempting to retrieve answered questions by school of thought, I should only get questions for that relevant school or non-school specific"
    );

    let valid_questions_by_school_of_thought: Vec<ImamQuestionDTO> = valid_answered_questions_dto
        .into_iter()
        .filter(|q| {
            q.school_of_thought == Some(SchoolOfThought::Maliki) || q.school_of_thought == None
        })
        .collect();
    let get_questions_by_school_of_thought_result = public_repository
        .get_answered_questions_by_school_of_thought(SchoolOfThought::Maliki)
        .await
        .unwrap();
    assert_dto_fields(
        &valid_questions_by_school_of_thought,
        &get_questions_by_school_of_thought_result,
    );

    eprintln!("When deleting an existing question, I should receive no error");
    let delete_questions_result = admin_repository.delete_imam_question_by_id(&7).await;
    assert!(delete_questions_result.is_ok());

    container.stop().await.unwrap();
}
fn assert_dto_fields(expected_dto: &Vec<ImamQuestionDTO>, actual_dto: &Vec<ImamQuestionDTO>) {
    for i in 0..actual_dto.len() {
        assert_eq!(expected_dto[i].title, actual_dto[i].title);
        assert_eq!(expected_dto[i].description, actual_dto[i].description);
        assert_eq!(expected_dto[i].topic, actual_dto[i].topic);
        assert_eq!(
            expected_dto[i].school_of_thought,
            actual_dto[i].school_of_thought
        );
        assert_eq!(expected_dto[i].answer, actual_dto[i].answer);
    }
}
fn get_valid_questions() -> Vec<ImamQuestion> {
    let date_question_answered = chrono::NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(2025, 12, 8).unwrap(),
        chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
    )
    .and_utc();
    vec![
        ImamQuestion {
            id: 1,
            title: "Question 1".to_string(),
            topic: "Generic".to_string(),
            school_of_thought: None,
            description: "This is my description".to_string(),
            date_of_question: chrono::Utc::now(),
            imam_name: Some("Zayd".to_owned()),
            answer: Some("This is an answer".to_owned()),
            date_answered: Some(date_question_answered.clone()),
        },
        ImamQuestion {
            id: 2,
            title: "Question 2".to_string(),
            topic: "Generic".to_string(),
            school_of_thought: Some("Hanafi".to_owned()),
            description: "This is my description".to_string(),
            date_of_question: chrono::Utc::now(),
            imam_name: None,
            answer: None,
            date_answered: None,
        },
        ImamQuestion {
            id: 3,
            title: "Question 3".to_string(),
            topic: "Fiqh".to_string(),
            school_of_thought: Some("Hanafi".to_owned()),
            description: "This is my description".to_string(),
            date_of_question: chrono::Utc::now(),
            imam_name: None,
            answer: None,
            date_answered: None,
        },
        ImamQuestion {
            id: 4,
            title: "Question 4".to_string(),
            topic: "Fiqh".to_string(),
            school_of_thought: Some("Hanbali".to_owned()),
            description: "This is my description".to_string(),
            date_of_question: chrono::Utc::now(),
            imam_name: None,
            answer: None,
            date_answered: None,
        },
        ImamQuestion {
            id: 5,
            title: "Question 5".to_string(),
            topic: "Aqeedah".to_string(),
            school_of_thought: Some("Hanbali".to_owned()),
            description: "This is my description".to_string(),
            date_of_question: chrono::Utc::now(),
            imam_name: None,
            answer: None,
            date_answered: None,
        },
        ImamQuestion {
            id: 6,
            title: "Question 6".to_string(),
            topic: "Aqeedah".to_string(),
            school_of_thought: Some("Maliki".to_owned()),
            description: "This is my description".to_string(),
            date_of_question: chrono::Utc::now(),
            imam_name: None,
            answer: None,
            date_answered: None,
        },
        ImamQuestion {
            id: 7,
            title: "Question 7".to_string(),
            topic: "Tafseer".to_string(),
            school_of_thought: Some("Maliki".to_owned()),
            description: "This is my description".to_string(),
            date_of_question: chrono::Utc::now(),
            imam_name: Some("Zayd".to_owned()),
            answer: Some("This is an answer".to_owned()),
            date_answered: Some(date_question_answered),
        },
    ]
}
