-- Create database
CREATE DATABASE IF NOT EXISTS masjidappdatabase;
USE masjidappdatabase;

-- Create tables
CREATE TABLE IF NOT EXISTS user_details (
    id INT AUTO_INCREMENT PRIMARY KEY,
    first_name VARCHAR(50),
    last_name VARCHAR(50),
    role VARCHAR(50),
    email VARCHAR(50),
    username VARCHAR(MAX),
    password VARCHAR(MAX),
);

-- Create stored procedures
DELIMITER $$

CREATE PROCEDURE IF NOT EXISTS get_username(IN p_username VARCHAR(MAX))
BEGIN
    SELECT COUNT(*) FROM user_details WHERE username = p_username;
END

CREATE PROCEDURE IF NOT EXISTS get_user_credentials(IN p_username VARCHAR(MAX), IN p_password VARCHAR(MAX))
BEGIN
    SELECT COUNT(*) FROM user_details WHERE username = p_username AND password = p_password;
END

CREATE PROCEDURE IF NOT EXISTS register_user(IN p_first_name VARCHAR(50), IN p_last_name VARCHAR(50), IN role VARCHAR(50), IN p_email VARCHAR(50), IN p_username VARCHAR(MAX), IN p_password VARCHAR(MAX))
BEGIN
    INSERT INTO user_details (first_name, last_name, role, email, username, password) 
    VALUES (p_first_name, p_last_name, p_role, p_email, p_username, p_password);
END

CREATE PROCEDURE IF NOT EXISTS reset_user_password(IN p_username VARCHAR(MAX), IN p_password VARCHAR(MAX))
BEGIN
    UPDATE user_details SET password = p_password WHERE username = p_username
END

DELIMITER ;

-- Create users
CREATE USER IF NOT EXISTS 'authenticationuser'@'%' IDENTIFIED BY 'BL/6Fx$Ku!2{37GvPS9';

--Adjust user permissions
REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'authenticationuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_username TO 'authenticationuser'@'%'
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_user_credentials TO 'authenticationuser'@'%'
GRANT EXECUTE ON PROCEDURE masjidappdatabase.register_user TO 'authenticationuser'@'%'
GRANT EXECUTE ON PROCEDURE masjidappdatabase.reset_user_password TO 'authenticationuser'@'%'

