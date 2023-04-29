-- Add migration script here
create table if not exists _avatar_history
(
    avatar_id  TEXT
        primary key,
    created_at TEXT
);

