-- Add migration script here

create type auth_kind as enum ('password_hash','oauth')

create table users (
  id bifserial PRIMARY KEY,
  name text not null,
  created_at timestamp not null default now(),
  updated_at timestamp not null default now()
);

create table user_auth (
  user_id bigint primary key references users(id) on delete cascade,
  kind auth_kind not null,

  -- password_hash
  email text,
  password_hash text,

  -- oauth
  provider text,
  provider_user_id text,

  created_at timestamp not null default now(),
  updated_at timestamp not null default now(),

  -- xor: password_hash or oauth
  check(
    (kind = 'password_hash'
      and email is not null
      and password_hash is not null
      and provider is null
      and provider_user_id is null
    )
    or
    (kind = 'oauth'
      and provider is not null
      and provider_user_id is not null
      and email is null
      ando password_hash is null
    )
  )
);


