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
    file_data LONGBLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS announcements (
    id INT NOT NULL AUTO_INCREMENT,
    title VARCHAR(50) NOT NULL,
    description VARCHAR(50) NULL,
    image LONGBLOB NULL,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    user_id INT NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT uc_announcement UNIQUE (title, description, user_id),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES user_details(id) ON DELETE CASCADE
);