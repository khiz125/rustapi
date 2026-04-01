CREATE TYPE auth_kind AS ENUM ('password_hash','oauth');

CREATE TABLE users (
  id bigserial PRIMARY KEY,
  name text NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE user_auth (
  user_id bigint PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
  kind auth_kind NOT NULL,

  email text,
  password_hash text,
  provider text,
  provider_user_id text,

  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now(),

  CHECK (
    (kind = 'password_hash' AND email IS NOT NULL AND password_hash IS NOT NULL AND provider IS NULL AND provider_user_id IS NULL)
    OR
    (kind = 'oauth' AND provider IS NOT NULL AND provider_user_id IS NOT NULL AND email IS NULL AND password_hash IS NULL)
  )
);
