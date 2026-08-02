create table if not exists assets (
    id bigserial primary key not null,
    name text not null unique,
    unit_value double precision not null,
    registrant_id bigserial not null references users(id),
    registered_at timestamp with time zone not null default now()
);
