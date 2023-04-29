-- Add migration script here
create table if not exists _friend_log_history
(
    id                    INTEGER
        primary key,
    created_at            TEXT,
    type                  TEXT,
    user_id               TEXT,
    display_name          TEXT,
    previous_display_name TEXT,
    trust_level           TEXT,
    previous_trust_level  TEXT
);

