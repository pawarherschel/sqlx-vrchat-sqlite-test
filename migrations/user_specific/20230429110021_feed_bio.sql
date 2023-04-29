-- Add migration script here
create table if not exists _feed_bio
(
    id           INTEGER
        primary key,
    created_at   TEXT,
    user_id      TEXT,
    display_name TEXT,
    bio          TEXT,
    previous_bio TEXT
);

