CREATE TYPE user_plan AS ENUM ('free', 'premium');

ALTER TABLE users ADD COLUMN plan user_plan NOT NULL DEFAULT 'free';
ALTER TABLE users ADD COLUMN plan_expires_at timestamptz;
