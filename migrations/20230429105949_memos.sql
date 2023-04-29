-- Add migration script here
create table if not exists memos
(
    user_id   TEXT
        primary key,
    edited_at TEXT,
    memo      TEXT
);

