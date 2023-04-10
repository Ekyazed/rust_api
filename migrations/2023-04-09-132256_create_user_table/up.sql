-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username varchar(255) not null,
    email varchar(255) not null,
    password varchar(255) not null
)