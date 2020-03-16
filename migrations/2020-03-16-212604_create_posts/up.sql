CREATE TABLE posts
(
    id      BIGINT PRIMARY KEY AUTO_INCREMENT,
    user_id BIGINT NOT NULL,
    body    TEXT   NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
)