-- Create users
CREATE USER IF NOT EXISTS 'authenticationuser'@'%' IDENTIFIED BY 'BL6FxKu!237GvPS9';
CREATE USER IF NOT EXISTS 'prayertimesadmin'@'%' IDENTIFIED BY 'HR0o8NRkwvuMaIBh7yaf';
CREATE USER IF NOT EXISTS 'eventsadmin'@'%' IDENTIFIED BY 'changeme';
CREATE USER IF NOT EXISTS 'askimamadmin'@'%' IDENTIFIED BY 'changeme';

-- Adjust user permissions
REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'authenticationuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_username TO 'authenticationuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_user_credentials TO 'authenticationuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.register_user TO 'authenticationuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.reset_user_password TO 'authenticationuser'@'%';

REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'prayertimesadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_prayer_times TO 'prayertimesadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_updated_prayer_times TO 'prayertimesadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.upsert_prayer_times TO 'prayertimesadmin'@'%';

REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'eventsadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_events TO 'eventsadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.upsert_event TO 'eventsadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.retrieve_image_url_by_event_id TO 'eventsadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.delete_event_by_id TO 'eventsadmin'@'%';

REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'askimamadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_all_imam_questions TO 'askimamadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_unanswered_imam_questions TO 'askimamadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_unanswered_imam_questions_by_topic TO 'askimamadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_unanswered_imam_questions_by_school_of_thought TO 'askimamadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_unanswered_imam_questions_by_topic_and_school_of_thought TO 'askimamadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_answered_imam_questions TO 'askimamadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_answered_imam_questions_by_topic TO 'askimamadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_answered_imam_questions_by_school_of_thought TO 'askimamadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_answered_imam_questions_by_topic_and_school_of_thought TO 'askimamadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.insert_question_for_imam TO 'askimamadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.upsert_imam_answer_to_question TO 'askimamadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.delete_imam_question_by_id TO 'askimamadmin'@'%';
