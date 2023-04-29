-- Add migration script here
create table if not exists gamelog_location
(
    id         INTEGER PRIMARY KEY,
    created_at TEXT NOT NULL,
    location   TEXT NOT NULL,
    world_id   TEXT NOT NULL,
    world_name TEXT NOT NULL,
    time       INTEGER,
    group_name TEXT DEFAULT '',
    unique (created_at, location)
)
