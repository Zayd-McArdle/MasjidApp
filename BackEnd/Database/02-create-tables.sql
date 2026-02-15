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

CREATE TABLE IF NOT EXISTS imam_question (
    id INT NOT NULL AUTO_INCREMENT,
    title VARCHAR(50) NOT NULL,
    topic VARCHAR(20) NOT NULL,
    school_of_thought VARCHAR(7) NULL CHECK (school_of_thought IN ('Hanafi', 'Shaafi', 'Maliki', 'Hanbali')),
    description VARCHAR(250),
    date TIMESTAMP NOT NULL,
    imam_name VARCHAR(50) NULL,
    answer VARCHAR(250) NULL,
    date_answered TIMESTAMP NULL,
    PRIMARY KEY (id),
    CONSTRAINT chk_question_is_answered_or_not CHECK (
        (imam_name IS NULL AND answer IS NULL AND date_answered IS NULL) OR
        (imam_name IS NOT NULL AND answer IS NOT NULL AND date_answered IS NOT NULL)
    )
);

CREATE TABLE IF NOT EXISTS donation_history (
    id INT NOT NULL AUTO_INCREMENT,
    cause VARCHAR(50) NOT NULL,
    donation_intention VARCHAR(7) NOT NULL CHECK (donation_intention IN ('Lillah', 'Sadaqah', 'Zakat')),
    donor_full_name VARCHAR(50) NOT NULL,
    donor_title VARCHAR(50) NOT NULL,
    phone_number VARCHAR(15) NOT NULL,
    email VARCHAR(50) NULL,
    address_line_1 VARCHAR(40) NOT NULL,
    address_line_2 VARCHAR(40) NULL,
    address_city VARCHAR(30) NOT NULL,
    address_region VARCHAR(30) NOT NULL,
    address_country VARCHAR(30) NULL,
    address_postal VARCHAR(12) NOT NULL,
    amount INT UNSIGNED NOT NULL,
    is_gift_aid BOOLEAN NOT NULL,
    donation_frequency VARCHAR(10) NOT NULL,
    transaction_status VARCHAR(40) NOT NULL,
    PRIMARY KEY (id)
);