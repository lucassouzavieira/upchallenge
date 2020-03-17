CREATE TABLE users
(
    id    BIGINT PRIMARY KEY AUTO_INCREMENT,
    name  TEXT NOT NULL,
    email TEXT NOT NULL
);

INSERT INTO users(name, email) VALUES ("Freddie Mercury", "freddie@queen.com");
INSERT INTO users(name, email) VALUES ("Elton John", "elton@rocket.com");
INSERT INTO users(name, email) VALUES ("David Bowie", "bowie@startman.com");