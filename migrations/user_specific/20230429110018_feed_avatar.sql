-- Add migration script here
create table if not exists _feed_avatar
(
    id                                          INTEGER
        primary key,
    created_at                                  TEXT,
    user_id                                     TEXT,
    display_name                                TEXT,
    owner_id                                    TEXT,
    avatar_name                                 TEXT,
    current_avatar_image_url                    TEXT,
    current_avatar_thumbnail_image_url          TEXT,
    previous_current_avatar_image_url           TEXT,
    previous_current_avatar_thumbnail_image_url TEXT
);

