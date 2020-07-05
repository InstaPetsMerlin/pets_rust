-- Your SQL goes here
CREATE TABLE posts
(
    id         SERIAL PRIMARY KEY,
    user_id   SERIAL NOT NULL,
    text   VARCHAR NOT NULL,
    image VARCHAR NOT NULL,
    date date
)