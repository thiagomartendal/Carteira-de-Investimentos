create table if not exists owned_assets (
  id bigserial primary key not null,
  user_id bigserial not null references users(id),
  asset_id bigserial not null references assets(id),
  bought_for double precision not null,
  quantity_owned double precision not null,
  timestamp timestamp with time zone not null default now()
);
