-- Create users
CREATE USER IF NOT EXISTS 'authenticationuser'@'%' IDENTIFIED BY 'BL6FxKu!237GvPS9';
CREATE USER IF NOT EXISTS 'prayertimesuser'@'%' IDENTIFIED BY 'HR0o8N&Rk%wvu@Ma%IBh7yaf';

-- Adjust user permissions
REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'authenticationuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_username TO 'authenticationuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_user_credentials TO 'authenticationuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.register_user TO 'authenticationuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.reset_user_password TO 'authenticationuser'@'%';

REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'prayertimesuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_prayer_times_file TO 'prayertimesuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.update_prayer_times_file TO 'prayertimesuser'@'%';

