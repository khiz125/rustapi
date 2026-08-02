-- Add migration script here
ALTER TABLE user_auth DROP CONSTRAINT user_auth_check;

ALTER TABLE user_auth ADD CONSTRAINT user_auth_check CHECK (
  (kind = 'password_hash' AND email IS NOT NULL AND password_hash IS NOT NULL AND provider IS NULL AND provider_user_id IS NULL)
  OR
  (kind = 'oauth' AND provider IS NOT NULL and provider_user_id IS NOT NULL AND password_hash IS NULL)
  OR
  (kind = 'mobile_device' AND email IS NULL AND password_hash IS NULL AND provider IS NULL AND provider_user_id IS NOT NULL)
);
