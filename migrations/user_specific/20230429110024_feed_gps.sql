-- Add migration script here
create table if not exists _feed_gps
(
    id                INTEGER
        primary key,
    created_at        TEXT,
    user_id           TEXT,
    display_name      TEXT,
    location          TEXT,
    world_name        TEXT,
    previous_location TEXT,
    time              INTEGER,
    group_name        TEXT default ''
);

