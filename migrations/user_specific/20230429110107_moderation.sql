-- Add migration script here
create table if not exists _moderation
(
    user_id      TEXT
        primary key,
    updated_at   TEXT,
    display_name TEXT,
    block        INTEGER,
    mute         INTEGER
);

