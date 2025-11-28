-- Create users
CREATE USER IF NOT EXISTS 'prayertimesuser'@'%' IDENTIFIED BY 'HR0o8NRkwvuMaIBh7yaf';
CREATE USER IF NOT EXISTS 'eventsuser'@'%' IDENTIFIED BY 'changeme';

-- Adjust user permissions
REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'prayertimesuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_prayer_times TO 'prayertimesuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_updated_prayer_times TO 'prayertimesuser'@'%';

REVOKE ALL PRIVILEGES, GRANT OPTION FROM 'eventsuser'@'%';
GRANT EXECUTE ON PROCEDURE masjidappdatabase.get_events TO 'eventsuser'@'%';

