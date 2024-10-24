CREATE TABLE IF NOT EXISTS user_details (
    id INT NOT NULL AUTO_INCREMENT,
    first_name VARCHAR(50),
    last_name VARCHAR(50),
    role VARCHAR(50),
    email VARCHAR(50),
    username VARCHAR(200),
    password VARCHAR(200),
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS prayer_times (
    file_data LONGBLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS announcements (
    id INT NOT NULL AUTO_INCREMENT,
    title VARCHAR(50) NOT NULL,
    description VARCHAR(50) NOT NULL,
    image LONGBLOB NULL,
    last_updated DATE DEFAULT (CURDATE()),
    PRIMARY KEY (id)
);