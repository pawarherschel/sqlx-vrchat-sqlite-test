-- Add migration script here
create table if not exists cache_world
(
    id                  TEXT PRIMARY KEY,
    added_at            TEXT,
    author_id           TEXT,
    author_name         TEXT,
    created_at          TEXT,
    description         TEXT,
    image_url           TEXT,
    name                TEXT,
    release_status      TEXT,
    thumbnail_image_url TEXT,
    updated_at          TEXT,
    version             INTEGER
)

