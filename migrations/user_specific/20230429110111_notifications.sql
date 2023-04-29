-- Add migration script here
create table if not exists _notifications
(
    id               TEXT
        primary key,
    created_at       TEXT,
    type             TEXT,
    sender_user_id   TEXT,
    sender_username  TEXT,
    receiver_user_id TEXT,
    message          TEXT,
    world_id         TEXT,
    world_name       TEXT,
    image_url        TEXT,
    invite_message   TEXT,
    request_message  TEXT,
    response_message TEXT,
    expired          INTEGER
);

