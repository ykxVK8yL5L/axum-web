-- Your SQL goes here
CREATE TABLE login_history
(
    id Integer PRIMARY KEY NOT NULL,
    user_id Integer NOT NULL REFERENCES users(id),
    login_timestamp TIMESTAMP default (datetime('now', 'localtime')) NOT NULL
);
