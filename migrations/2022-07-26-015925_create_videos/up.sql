-- Your SQL goes here
CREATE TABLE videos (
    id Integer PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    cid VARCHAR NOT NULL,
    size VARCHAR,
    img VARCHAR,
    created_at TIMESTAMP default (datetime('now', 'localtime')) NOT NULL
);
