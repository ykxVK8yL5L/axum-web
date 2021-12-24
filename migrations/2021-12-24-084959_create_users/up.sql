-- Your SQL goes here
CREATE TABLE users (
    id Integer PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    login_session VARCHAR 
);