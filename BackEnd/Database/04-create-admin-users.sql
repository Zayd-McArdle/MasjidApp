-- Create users
CREATE USER IF NOT EXISTS 'authenticationuser'@'%' IDENTIFIED BY 'BL6FxKu!237GvPS9';
CREATE USER IF NOT EXISTS 'prayertimesadmin'@'%' IDENTIFIED BY 'HR0o8NRkwvuMaIBh7yaf';
CREATE USER IF NOT EXISTS 'announcementsadmin'@'%' IDENTIFIED BY 'LzwvN6bU4y3EqmAYBMJFrn';

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

REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'announcementsadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_announcements TO 'announcementsadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.post_announcement TO 'announcementsadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.edit_announcement TO 'announcementsadmin'@'%';