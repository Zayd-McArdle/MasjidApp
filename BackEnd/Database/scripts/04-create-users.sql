-- Create users
CREATE USER IF NOT EXISTS 'authenticationuser'@'%' IDENTIFIED BY 'BL/6Fx$Ku!2{37GvPS9';

--Adjust user permissions
REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'authenticationuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_username TO 'authenticationuser'@'%'
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_user_credentials TO 'authenticationuser'@'%'
GRANT EXECUTE ON PROCEDURE masjidappdatabase.register_user TO 'authenticationuser'@'%'
GRANT EXECUTE ON PROCEDURE masjidappdatabase.reset_user_password TO 'authenticationuser'@'%'
