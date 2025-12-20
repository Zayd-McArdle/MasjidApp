-- Create users
CREATE USER IF NOT EXISTS 'prayertimesuser'@'%' IDENTIFIED BY 'HR0o8NRkwvuMaIBh7yaf';
CREATE USER IF NOT EXISTS 'eventsuser'@'%' IDENTIFIED BY 'changeme';
CREATE USER IF NOT EXISTS 'askimamuser'@'%' IDENTIFIED BY 'changeme';

-- Adjust user permissions
REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'prayertimesuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_prayer_times TO 'prayertimesuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_updated_prayer_times TO 'prayertimesuser'@'%';

REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'eventsuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_events TO 'eventsuser'@'%';

REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'askimamuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_answered_imam_questions TO 'askimamuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_answered_imam_questions_by_topic TO 'askimamuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_answered_imam_questions_by_school_of_thought TO 'askimamuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_answered_imam_questions_by_topic_and_school_of_thought TO 'askimamuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.insert_question_for_imam TO 'askimamuser'@'%';

