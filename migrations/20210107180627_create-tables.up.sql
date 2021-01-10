CREATE EXTENSION
IF NOT EXISTS "uuid-ossp";

create table users
(
    id uuid default uuid_generate_v4() primary key,
    username varchar not null unique,
    active boolean not null default true,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);