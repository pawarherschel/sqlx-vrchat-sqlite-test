-- Add migration script here
create table if not exists favorite_world
(
    id         INTEGER
        primary key,
    created_at TEXT,
    world_id   TEXT,
    group_name TEXT
);

