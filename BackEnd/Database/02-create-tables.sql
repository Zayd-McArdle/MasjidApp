CREATE TABLE IF NOT EXISTS user_details (
    id INT NOT NULL AUTO_INCREMENT,
    first_name VARCHAR(50),
    last_name VARCHAR(50),
    role VARCHAR(50),
    email VARCHAR(50) UNIQUE,
    username VARCHAR(200) UNIQUE,
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
    user_id INT NOT NULL,
    PRIMARY KEY (id)
);