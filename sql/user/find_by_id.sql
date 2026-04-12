SELECT
  u.id,
  u.name,
  u.created_at,
  u.updated_at,
  a.kind::text as "kind!: String",
  a.email,
  a.password_hash,
  a.provider,
  a.provider_user_id,
  a.created_at as auth_created_at,
  a.updated_at as auth_updated_at
FROM users u
INNER JOIN user_auth a ON a.user_id = u.id
WHERE u.id = $1;
