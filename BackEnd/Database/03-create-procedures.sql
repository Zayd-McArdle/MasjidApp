DELIMITER //
-- user_details stored procedures

CREATE PROCEDURE IF NOT EXISTS get_username(IN p_username VARCHAR(200))
BEGIN
    SELECT COUNT(*) FROM user_details WHERE username = p_username;
END //

CREATE PROCEDURE IF NOT EXISTS get_user_credentials(IN p_username VARCHAR(200))
BEGIN
    SELECT username, password, role FROM user_details WHERE username = p_username;
END //

CREATE PROCEDURE IF NOT EXISTS register_user(IN p_full_name VARCHAR(100), 
                                                IN p_role VARCHAR(50), 
                                                IN p_email VARCHAR(50), 
                                                IN p_username VARCHAR(200), 
                                                IN p_password VARCHAR(200))
BEGIN
    INSERT INTO user_details (full_name, role, email, username, password) 
    VALUES (p_full_name, p_role, p_email, p_username, p_password);
END //

CREATE PROCEDURE IF NOT EXISTS reset_user_password(IN p_username VARCHAR(200), 
                                                    IN p_password VARCHAR(200))
BEGIN
    UPDATE user_details 
    SET password = p_password 
    WHERE username = p_username;
END //

-- prayer_times stored procedures

CREATE PROCEDURE IF NOT EXISTS get_prayer_times()
BEGIN
    SELECT data, hash from prayer_times;
END //

CREATE PROCEDURE IF NOT EXISTS get_updated_prayer_times(IN p_hash CHAR(64))
BEGIN
    DECLARE v_count INT;

    SELECT COUNT(*) INTO v_count 
    FROM prayer_times 
    WHERE hash = p_hash;

    IF v_count = 0 THEN
        CALL get_prayer_times();
    ELSE 
        SELECT hash 
        FROM prayer_times 
        WHERE hash = p_hash;
    END IF;
END //

CREATE PROCEDURE IF NOT EXISTS upsert_prayer_times(IN p_data LONGBLOB, 
                                                    IN p_hash CHAR(64))
BEGIN
    UPDATE prayer_times SET data = p_data, hash = p_hash;
    IF ROW_COUNT() = 0 THEN 
        INSERT INTO prayer_times (data, hash) VALUES (p_data, p_hash);
    END IF;
END //

-- events stored procedures

CREATE PROCEDURE IF NOT EXISTS get_events()
BEGIN
    SELECT id, 
    title, 
    description, 
    date, 
    type, 
    recurrence, 
    status, 
    minimum_age,
    maximum_age,
    image_url, 
    full_name, 
    phone_number, 
    email
    FROM events;
END //

CREATE PROCEDURE IF NOT EXISTS upsert_event(IN p_id INT,
                                            IN p_title VARCHAR(50), 
                                            IN p_description VARCHAR(250), 
                                            IN p_date TIMESTAMP, 
                                            IN p_type VARCHAR(10), 
                                            IN p_recurrence VARCHAR(15),
                                            IN p_status VARCHAR(15),
                                            IN p_minimum_age TINYINT UNSIGNED,
                                            IN p_maximum_age TINYINT UNSIGNED,
                                            IN p_image_url VARCHAR(2083),
                                            IN p_full_name VARCHAR (100),
                                            IN p_phone_number VARCHAR(15),
                                            IN p_email VARCHAR(50))
BEGIN
    IF p_id = 0 THEN
        INSERT INTO events (title, 
            description, 
            date, 
            type, 
            recurrence,
            status, 
            minimum_age, 
            maximum_age, 
            image_url, 
            full_name, 
            phone_number, 
            email)
        VALUES (p_title,
            p_description,
            p_date,
            p_type,
            p_recurrence,
            p_status,
            p_minimum_age,
            p_maximum_age,
            p_image_url,
            p_full_name,
            p_phone_number,
            p_email);
    ELSE 
        UPDATE events SET title = p_title, 
            description = p_description, 
            date = p_date,
            type = p_type,
            recurrence = p_recurrence,
            status = p_status,
            minimum_age = p_minimum_age,
            maximum_age = p_maximum_age,
            image_url = p_image_url,
            full_name = p_full_name,
            phone_number = p_phone_number,
            email = p_email
        WHERE id = p_id;
    END IF;
END //

CREATE PROCEDURE IF NOT EXISTS retrieve_image_url_by_event_id(IN p_id INT)
BEGIN
    SELECT image_url FROM events WHERE id = p_id;
END //

CREATE PROCEDURE IF NOT EXISTS delete_event_by_id(IN p_id INT)
BEGIN
    DELETE FROM events WHERE id = p_id;
END //

-- imam_question stored procedures

CREATE PROCEDURE IF NOT EXISTS get_all_imam_questions()
BEGIN
    SELECT * FROM imam_question;
END //

CREATE PROCEDURE IF NOT EXISTS get_unanswered_imam_questions()
BEGIN
    SELECT * FROM imam_question WHERE answer IS NULL;
END //

CREATE PROCEDURE IF NOT EXISTS get_unanswered_imam_questions_by_topic(IN p_topic VARCHAR(20))
BEGIN
    SELECT * FROM imam_question WHERE answer IS NULL AND topic = p_topic;
END //

CREATE PROCEDURE IF NOT EXISTS get_unanswered_imam_questions_by_school_of_thought(IN p_school_of_thought VARCHAR(7))
BEGIN
    SELECT * FROM imam_question WHERE answer IS NULL AND (school_of_thought = p_school_of_thought OR school_of_thought IS NULL);
END //

CREATE PROCEDURE IF NOT EXISTS get_unanswered_imam_questions_by_topic_and_school_of_thought(IN p_topic VARCHAR(20), 
                                                                                                IN p_school_of_thought VARCHAR(7))
BEGIN
    SELECT * FROM imam_question WHERE answer IS NULL AND topic = p_topic AND (school_of_thought = p_school_of_thought OR school_of_thought IS NULL);
END //

CREATE PROCEDURE IF NOT EXISTS get_answered_imam_questions()
BEGIN
    SELECT * FROM imam_question WHERE answer IS NOT NULL;
END //

CREATE PROCEDURE IF NOT EXISTS get_answered_imam_questions_by_topic(IN p_topic VARCHAR(20))
BEGIN
    SELECT * FROM imam_question WHERE answer IS NOT NULL AND topic = p_topic;
END //

CREATE PROCEDURE IF NOT EXISTS get_answered_imam_questions_by_school_of_thought(IN p_school_of_thought VARCHAR(7))
BEGIN
    SELECT * FROM imam_question WHERE answer IS NOT NULL AND (school_of_thought = p_school_of_thought OR school_of_thought IS NULL);
END //

CREATE PROCEDURE IF NOT EXISTS get_answered_imam_questions_by_topic_and_school_of_thought(IN p_topic VARCHAR(20), 
                                                                                            IN p_school_of_thought VARCHAR(7))
BEGIN
    SELECT * FROM imam_question WHERE answer IS NOT NULL AND topic = p_topic AND (school_of_thought = p_school_of_thought OR school_of_thought IS NULL);
END //

CREATE PROCEDURE IF NOT EXISTS insert_question_for_imam(IN p_title VARCHAR(50),
                                                        IN p_topic VARCHAR(20),
                                                        IN p_school_of_thought VARCHAR(7),
                                                        IN p_description VARCHAR(250),
                                                        IN p_date TIMESTAMP)
BEGIN
    INSERT INTO imam_question (title,
        topic,
        school_of_thought,
        description,
        date) 
    VALUES (p_title,
        p_topic,
        p_school_of_thought,
        p_description,
        p_date);
END //


CREATE PROCEDURE IF NOT EXISTS upsert_imam_answer_to_question(IN p_imam_name VARCHAR(50), 
                                                              IN p_answer VARCHAR(250),
                                                              IN p_date_answered TIMESTAMP,
                                                              IN p_id INT)
BEGIN
    UPDATE imam_question 
    SET imam_name = p_imam_name, 
        answer = p_answer, 
        date_answered = p_date_answered
    WHERE id = p_id;
END //

CREATE PROCEDURE IF NOT EXISTS delete_imam_question_by_id(IN p_id INT)
BEGIN
    DELETE FROM imam_question WHERE id = p_id;
END //

-- donation_history stored procedures

CREATE PROCEDURE IF NOT EXISTS get_donation_transactions(IN p_cause VARCHAR(50), 
                                                            IN p_donation_intention VARCHAR(7),
                                                            IN p_email VARCHAR(50),
                                                            IN p_phone_number VARCHAR(15),
                                                            IN p_amount INT UNSIGNED,
                                                            IN p_is_gift_aid BOOLEAN,
                                                            IN p_donation_frequency VARCHAR(10),
                                                            IN p_transaction_status VARCHAR(10))
BEGIN
    SELECT * FROM donation_history 
        WHERE (p_cause IS NULL OR cause = p_cause)
        AND (p_donation_intention IS NULL OR donation_intention = p_donation_intention)
        AND (p_email IS NULL OR email = p_email)
        AND (p_phone_number IS NULL OR phone_number = p_phone_number)
        AND (p_amount IS NULL OR amount = p_amount)
        AND (p_is_gift_aid IS NULL OR is_gift_aid = p_is_gift_aid)
        AND (p_donation_frequency IS NULL OR donation_frequency = p_donation_frequency)
        AND (p_transaction_status IS NULL OR transaction_status = p_transaction_status);
END //



CREATE PROCEDURE IF NOT EXISTS insert_donation_transaction(IN p_cause VARCHAR(50),
                                                            IN p_donation_intention VARCHAR(7),
                                                            IN p_donor_full_name VARCHAR(50),
                                                            IN p_donor_title VARCHAR(50),
                                                            IN p_phone_number VARCHAR(15),
                                                            IN p_email VARCHAR(50),
                                                            IN p_address_line_1 VARCHAR(40),
                                                            IN p_address_line_2 VARCHAR(40),
                                                            IN p_address_city VARCHAR(30),
                                                            IN p_address_region VARCHAR(30),
                                                            IN p_address_country VARCHAR(30),
                                                            IN p_address_postal VARCHAR(12),
                                                            IN p_amount INT UNSIGNED,
                                                            IN p_is_gift_aid BOOLEAN,
                                                            IN p_donation_frequency VARCHAR(10),
                                                            IN p_transaction_status VARCHAR(10))
BEGIN
    INSERT INTO donation_history (cause,
        donation_intention, 
        donor_full_name, 
        donor_title, 
        phone_number,
        email,
        address_line_1,
        address_line_2,
        address_city,
        address_region,
        address_country,
        address_postal,
        amount,
        is_gift_aid,
        donation_frequency,
        transaction_status)
    VALUES (p_cause, 
        p_donation_intention,
        p_donor_full_name, 
        p_donor_title, 
        p_phone_number,
        p_email,
        p_address_line_1,
        p_address_line_2,
        p_address_city,
        p_address_region,
        p_address_country,
        p_address_postal,
        p_amount,
        p_is_gift_aid,
        p_donation_frequency,
        p_transaction_status);
END //
DELIMITER ;

