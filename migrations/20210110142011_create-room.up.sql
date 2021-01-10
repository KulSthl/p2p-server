CREATE EXTENSION
IF NOT EXISTS "uuid-ossp";
create table room
(
    id uuid default uuid_generate_v4() primary key,
    name varchar not null unique,
    active boolean not null default true,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);
create table user_room
(
    room_id uuid
	  REFERENCES room(id),
    users_id uuid
	  REFERENCES users(id),
    unique(room_id,users_id)
);