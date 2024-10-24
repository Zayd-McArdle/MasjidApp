DELIMITER //
-- user_details stored procedures
CREATE PROCEDURE IF NOT EXISTS get_username(IN p_username VARCHAR(200))
BEGIN
    SELECT COUNT(*) FROM user_details WHERE username = p_username;
END //

CREATE PROCEDURE IF NOT EXISTS get_user_credentials(IN p_username VARCHAR(200))
BEGIN
    SELECT username, password FROM user_details WHERE username = p_username;
END //

CREATE PROCEDURE IF NOT EXISTS register_user(IN p_first_name VARCHAR(50), IN p_last_name VARCHAR(50), IN p_role VARCHAR(50), IN p_email VARCHAR(50), IN p_username VARCHAR(200), IN p_password VARCHAR(200))
BEGIN
    INSERT INTO user_details (first_name, last_name, role, email, username, password) 
    VALUES (p_first_name, p_last_name, p_role, p_email, p_username, p_password);
END //

CREATE PROCEDURE IF NOT EXISTS reset_user_password(IN p_username VARCHAR(200), IN p_password VARCHAR(200))
BEGIN
    UPDATE user_details 
    SET password = p_password 
    WHERE username = p_username;
END //

-- prayer_times stored procedures
CREATE PROCEDURE IF NOT EXISTS get_prayer_times_file()
BEGIN
    SELECT file_data from prayer_times;
END //

CREATE PROCEDURE IF NOT EXISTS update_prayer_times_file(IN p_updated_prayer_times_file LONGBLOB)
BEGIN
    UPDATE prayer_times SET file_data = p_updated_prayer_times_file;
END //

CREATE PROCEDURE IF NOT EXISTS get_announcements()
BEGIN
    SELECT * FROM announcements;
END //

CREATE PROCEDURE IF NOT EXISTS post_announcement(IN p_title VARCHAR(50), IN p_description VARCHAR(50), IN p_image LONGBLOB)
BEGIN
    INSERT INTO announcements (title, description, image)
    VALUES (p_title, p_description, p_image);
    SELECT LAST_INSERT_ID();
END //

CREATE PROCEDURE IF NOT EXISTS edit_announcement(IN p_id INT, IN p_title VARCHAR(50), IN p_description VARCHAR(50), IN p_image LONGBLOB)
BEGIN 
    UPDATE announcements SET title = p_title, description = p_description, image = p_image, last_updated = CURDATE() where id = p_id;
END //


DELIMITER ;