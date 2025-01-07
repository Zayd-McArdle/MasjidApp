DELIMITER //
-- user_details stored procedures
CREATE PROCEDURE IF NOT EXISTS get_username(IN p_username VARCHAR(200))
BEGIN
    SELECT COUNT(*) FROM user_details WHERE username = p_username;
END //

CREATE PROCEDURE IF NOT EXISTS get_user_credentials(IN p_username VARCHAR(200), IN p_password VARCHAR(200))
BEGIN
    SELECT COUNT(*) FROM user_details WHERE username = p_username AND password = p_password;
END //

CREATE PROCEDURE IF NOT EXISTS register_user(IN p_first_name VARCHAR(50), IN p_last_name VARCHAR(50), IN role VARCHAR(50), IN p_email VARCHAR(50), IN p_username VARCHAR(200), IN p_password VARCHAR(200))
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

CREATE PROCEDURE IF NOT EXISTS get_prayer_times_file()
BEGIN
    SELECT file_data from prayer_times;
END //

CREATE PROCEDURE IF NOT EXISTS update_prayer_times_file(IN updated_prayer_times_file LONGBLOB)
BEGIN
    UPDATE prayer_times SET file_data = updated_prayer_times_file;
END //

DELIMITER ;