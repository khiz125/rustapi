CREATE TYPE subscription_provider AS ENUM (
  'google_play', 'app_store', 'stripe', 'paypay'
);
CREATE TYPE subscription_status AS ENUM (
  'active', 'canceled', 'past_due', 'expired'
);

CREATE TABLE subscriptions (
  id bigserial PRIMARY KEY,
  user_id bigint NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  provider subscription_provider NOT NULL,
  provider_subscription_id text NOT NULL,
  status subscription_status NOT NULL DEFAULT 'active',
  plan user_plan NOT NULL DEFAULT 'premium',
  started_at timestamptz NOT NULL DEFAULT now(),
  expires_at timestamptz NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now(),

  CONSTRAINT uq_subscriptions_provider UNIQUE (provider, provider_subscription_id)
);

CREATE INDEX idx_subscriptions_user_id ON subscriptions(user_id);

