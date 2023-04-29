-- Add migration script here
create table if not exists gamelog_video_play
(
    id           INTEGER
        primary key,
    created_at   TEXT,
    video_url    TEXT,
    video_name   TEXT,
    video_id     TEXT,
    location     TEXT,
    display_name TEXT,
    user_id      TEXT,
    unique (created_at, video_url)
);

