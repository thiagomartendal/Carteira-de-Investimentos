create table if not exists users (
    id bigserial primary key not null,
    email varchar(255) not null unique,
    username varchar(255) not null,
    password_hash varchar(255) not null,
    user_type int not null -- 1 - Comprador - 2 - Administrador
);

-- Comprador: apenas compra ativos

-- Administrador: apenas cadastra ativos no sistema
