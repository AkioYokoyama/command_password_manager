CREATE TABLE IF NOT EXISTS passwords
(
    id          INTEGER PRIMARY KEY NOT NULL,
    key         TEXT    UNIQUE      NOT NULL,
    password    TEXT                NOT NULL
);
