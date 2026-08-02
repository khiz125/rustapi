-- 20260725073650_add_mobile_device_auth.sql

ALTER TYPE auth_kind ADD VALUE IF NOT EXISTS 'mobile_device';

