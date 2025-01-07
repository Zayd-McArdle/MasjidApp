CREATE TABLE IF NOT EXISTS user_details (
    id INT AUTO_INCREMENT PRIMARY KEY,
    first_name VARCHAR(50),
    last_name VARCHAR(50),
    role VARCHAR(50),
    email VARCHAR(50),
    username VARCHAR(MAX),
    password VARCHAR(MAX),
);