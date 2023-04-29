-- Add migration script here
create table if not exists _feed_online_offline
(
    id           INTEGER
        primary key,
    created_at   TEXT,
    user_id      TEXT,
    display_name TEXT,
    type         TEXT,
    location     TEXT,
    world_name   TEXT,
    time         INTEGER,
    group_name   TEXT default ''
);

