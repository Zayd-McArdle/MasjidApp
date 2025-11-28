CREATE TABLE IF NOT EXISTS user_details (
    id INT NOT NULL AUTO_INCREMENT,
    full_name VARCHAR(100),
    role VARCHAR(50),
    email VARCHAR(50),
    username VARCHAR(200),
    password VARCHAR(200),
    UNIQUE (email, username),
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS prayer_times (
    data LONGBLOB NOT NULL,
    hash CHAR(64) NOT NULL
);

CREATE TABLE IF NOT EXISTS events (
    id INT NOT NULL AUTO_INCREMENT,
    title VARCHAR(50) NOT NULL,
    description VARCHAR(250) NULL,
    date TIMESTAMP NOT NULL,
    -- Event Details
    type VARCHAR(10),
    recurrence VARCHAR(15),
    status VARCHAR(15),
    minimum_age TINYINT UNSIGNED,
    maximum_age TINYINT UNSIGNED,
    image_url VARCHAR(2083),
    -- Contact Details
    full_name VARCHAR(100),
    phone_number VARCHAR(15),
    email VARCHAR(50),
    PRIMARY KEY (id)
);