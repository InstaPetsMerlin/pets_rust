-- Your SQL goes here
CREATE TABLE pets
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR NOT NULL,
    age        smallint
)