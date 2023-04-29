-- Add migration script here
create table if not exists gamelog_event
(
    id         INTEGER
        primary key,
    created_at TEXT,
    data       TEXT,
    unique (created_at, data)
);

