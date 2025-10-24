-- Create users
CREATE USER IF NOT EXISTS 'authenticationuser'@'%' IDENTIFIED BY 'BL6FxKu!237GvPS9';
CREATE USER IF NOT EXISTS 'prayertimesadmin'@'%' IDENTIFIED BY 'HR0o8NRkwvuMaIBh7yaf';
CREATE USER IF NOT EXISTS 'announcementsadmin'@'%' IDENTIFIED BY 'LzwvN6bU4y3EqmAYBMJFrn';
CREATE USER IF NOT EXISTS 'eventsadmin'@'%' IDENTIFIED BY 'changeme';

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

REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'eventsadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_events TO 'eventsadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.upsert_event TO 'eventsadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.retrieve_image_url_by_event_id TO 'eventsadmin'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.delete_event_by_id TO 'eventsadmin'@'%';