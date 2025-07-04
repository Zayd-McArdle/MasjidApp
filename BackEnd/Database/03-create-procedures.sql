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

CREATE PROCEDURE IF NOT EXISTS register_user(IN p_full_name VARCHAR(100), IN p_role VARCHAR(50), IN p_email VARCHAR(50), IN p_username VARCHAR(200), IN p_password VARCHAR(200))
BEGIN
    INSERT INTO user_details (full_name, role, email, username, password) 
    VALUES (p_full_name, p_role, p_email, p_username, p_password);
END //

CREATE PROCEDURE IF NOT EXISTS reset_user_password(IN p_username VARCHAR(200), IN p_password VARCHAR(200))
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

CREATE PROCEDURE IF NOT EXISTS upsert_prayer_times(IN p_data LONGBLOB, IN p_hash CHAR(64))
BEGIN
    UPDATE prayer_times SET data = p_data, hash = p_hash;
    IF ROW_COUNT() = 0 THEN 
        INSERT INTO prayer_times (data, hash) VALUES (p_data, p_hash);
    END IF;
END //


-- announcements stored procedures
CREATE PROCEDURE IF NOT EXISTS get_announcements()
BEGIN
    SELECT a.id, a.title, a.description, a.last_updated, a.image, u.full_name
    FROM announcements a
    JOIN user_details u ON a.user_id = u.id;
END //

CREATE PROCEDURE IF NOT EXISTS post_announcement(IN p_title VARCHAR(50), IN p_description VARCHAR(50), IN p_image LONGBLOB, IN p_username VARCHAR(200))
BEGIN
    DECLARE v_user_id INT;
    -- Retrieve user ID based on username
    SELECT id INTO v_user_id 
    FROM user_details 
    WHERE username = p_username
    LIMIT 1;

    IF v_user_id IS NOT NULL THEN 
        INSERT INTO announcements (title, description, image, user_id)
        VALUES (p_title, p_description, p_image, v_user_id);
        SELECT LAST_INSERT_ID();
    ELSE
        SIGNAL SQLSTATE '45000'
        SET MESSAGE_TEXT = 'User not found';
    END IF;
END //

CREATE PROCEDURE IF NOT EXISTS edit_announcement(IN p_id INT, IN p_username VARCHAR(200), IN p_title VARCHAR(50), IN p_description VARCHAR(50), IN p_image LONGBLOB)
BEGIN 
    DECLARE v_user_id INT;
    -- Retrieve user ID based on username
    SELECT id INTO v_user_id
    FROM user_details
    WHERE username = p_username
    LIMIT 1;

    IF v_user_id IS NOT NULL THEN
        UPDATE announcements 
        SET title = p_title, description = p_description, image = p_image 
        WHERE id = p_id;
    ELSE
        SIGNAL SQLSTATE '45000'
        SET MESSAGE_TEXT = 'User not found';
    END IF;
END //


DELIMITER ;