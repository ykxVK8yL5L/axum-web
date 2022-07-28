-- Your SQL goes here
CREATE TABLE settings (
    id Integer PRIMARY KEY NOT NULL,
    key VARCHAR NOT NULL,
    value Text NOT NULL,
    desc VARCHAR,
    hidden BOOLEAN NOT NULL DEFAULT 0
);
