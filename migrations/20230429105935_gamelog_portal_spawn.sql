-- Add migration script here
create table if not exists gamelog_portal_spawn
(
    id           INTEGER
        primary key,
    created_at   TEXT,
    display_name TEXT,
    location     TEXT,
    user_id      TEXT,
    instance_id  TEXT,
    world_name   TEXT,
    unique (created_at, display_name)
);

