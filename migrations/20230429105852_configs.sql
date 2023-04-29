-- Add migration script here
create table if not exists configs
(
    key   TEXT
        primary key,
    value TEXT
);

