-- Create users
CREATE USER IF NOT EXISTS 'prayertimesuser'@'%' IDENTIFIED BY 'HR0o8NRkwvuMaIBh7yaf';
CREATE USER IF NOT EXISTS 'announcementsuser'@'%' IDENTIFIED BY 'LzwvN6bU4y3EqmAYBMJFrn';

-- Adjust user permissions
REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'prayertimesuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_prayer_times TO 'prayertimesuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_updated_prayer_times TO 'prayertimesuser'@'%';

REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'announcementsuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_announcements TO 'announcementsuser'@'%';