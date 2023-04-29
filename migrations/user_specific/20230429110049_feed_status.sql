-- Add migration script here
create table if not exists _feed_status
(
    id                          INTEGER
        primary key,
    created_at                  TEXT,
    user_id                     TEXT,
    display_name                TEXT,
    status                      TEXT,
    status_description          TEXT,
    previous_status             TEXT,
    previous_status_description TEXT
);

