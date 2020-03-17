CREATE TABLE posts
(
    id      BIGINT PRIMARY KEY AUTO_INCREMENT,
    user_id BIGINT NOT NULL,
    body    TEXT   NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

INSERT INTO posts(user_id, body) VALUES (1, "Ay-ooooooh");
INSERT INTO posts(user_id, body) VALUES (1, "Anyway the wind blows");
INSERT INTO posts(user_id, body) VALUES (1, "We are the champions, my friends!");
INSERT INTO posts(user_id, body) VALUES (2, "I'm rocket man!");
INSERT INTO posts(user_id, body) VALUES (2, "Candle in the dark");
INSERT INTO posts(user_id, body) VALUES (3, "Under pressure!");
INSERT INTO posts(user_id, body) VALUES (3, "There's is a starman, waiting in the sky");