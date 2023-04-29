-- Add migration script here
create table if not exists _friend_log_current
(
    user_id      TEXT
        primary key,
    display_name TEXT,
    trust_level  TEXT
);

