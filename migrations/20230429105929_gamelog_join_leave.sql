-- Add migration script here
create table if not exists gamelog_join_leave
(
    id           INTEGER
        primary key,
    created_at   TEXT,
    type         TEXT,
    display_name TEXT,
    location     TEXT,
    user_id      TEXT,
    time         INTEGER,
    unique (created_at, type, display_name)
);

