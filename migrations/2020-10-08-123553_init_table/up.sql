-- Your SQL goes here
CREATE TABLE todo_lists
(
    id      SERIAL PRIMARY KEY,
    title   VARCHAR NOT NULL
);

CREATE TABLE todos
(
    id      SERIAL PRIMARY KEY,
    title   VARCHAR NOT NULL,
    list_id INTEGER NOT NULL,
    checked BOOLEAN NOT NULL DEFAULT 'f'
);