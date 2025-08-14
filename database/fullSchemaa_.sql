-- Ensure required extensions
CREATE EXTENSION IF NOT EXISTS pgcrypto; -- gen_random_uuid(), crypto helpers
CREATE EXTENSION IF NOT EXISTS citext;   -- case-insensitive text for email/username

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'role_user') THEN
        CREATE TYPE ROLE_USER AS ENUM ('Admin', 'User', 'System');
    END IF;
END $$;

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = NOW();
   RETURN NEW;
END;
$$ language 'plpgsql';

-- Core users table (represents the person)
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    primary_email CITEXT UNIQUE NOT NULL,
    username CITEXT UNIQUE,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    display_name VARCHAR(255),
    avatar_url VARCHAR(500),
    role ROLE_USER DEFAULT 'User',
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_verified BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    last_login_at TIMESTAMPTZ
);

-- Providers lookup
CREATE TABLE IF NOT EXISTS providers (
    id SMALLINT PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    is_oauth BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true
);

-- Token types
CREATE TABLE IF NOT EXISTS token_types (
    id SMALLINT PRIMARY KEY,
    name VARCHAR(20) UNIQUE NOT NULL,
    description VARCHAR(100) NOT NULL,
    expiration INTERVAL NOT NULL
);

-- Linked accounts: each external or local account a user connects
CREATE TABLE IF NOT EXISTS linked_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider_id SMALLINT NOT NULL REFERENCES providers(id),
    provider_user_id VARCHAR(255) NOT NULL, -- external unique id (or email for local)

    -- Provider profile data (can differ from user's main profile)
    provider_email VARCHAR(255),
    provider_username VARCHAR(100),
    provider_display_name VARCHAR(255),
    provider_avatar_url VARCHAR(500),
    provider_profile_data JSONB,

    -- For local/internal accounts (provider_id = 1)
    password_hash VARCHAR(255),

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_primary BOOLEAN NOT NULL DEFAULT false,

    -- Timestamps
    connected_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    last_login_at TIMESTAMPTZ,

    -- Global uniqueness: same provider_user_id cannot belong to multiple users
    UNIQUE (provider_id, provider_user_id),
    -- Local provider must have a password hash
    CONSTRAINT chk_local_requires_password CHECK ((provider_id <> 1) OR (password_hash IS NOT NULL))
);

-- Tokens can be user-scoped (persist across all linked accounts) OR account-scoped (OAuth/API token)
CREATE TABLE IF NOT EXISTS tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    linked_account_id UUID REFERENCES linked_accounts(id) ON DELETE CASCADE,
    type_id SMALLINT NOT NULL REFERENCES token_types(id),
    value TEXT NOT NULL, -- legacy plaintext column; prefer encrypted + hash below
    value_encrypted BYTEA, -- optional encrypted token
    value_hash BYTEA,      -- deterministic hash (e.g., SHA-256) for equality lookups
    expires_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(value),
    -- Exactly one of user_id or linked_account_id must be set (XOR)
    CHECK ( (user_id IS NOT NULL) <> (linked_account_id IS NOT NULL) )
);

-- Pivot table: many scopes per token
CREATE TABLE IF NOT EXISTS token_scopes (
    token_id UUID NOT NULL REFERENCES tokens(id) ON DELETE CASCADE,
    scope VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (token_id, scope)
);

-- Sessions reference the user
CREATE TABLE IF NOT EXISTS sessions_table (
    session_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_id UUID REFERENCES tokens(id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    user_agent TEXT,
    ip_address INET
);

CREATE TABLE IF NOT EXISTS audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    linked_account_id UUID REFERENCES linked_accounts(id) ON DELETE SET NULL,
    event VARCHAR(50) NOT NULL,
    ip_address INET,
    success BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Seed data
INSERT INTO providers (id, name, display_name, is_oauth, is_active) VALUES
(1, 'internal', 'Local Account', false, true),
(2, 'google', 'Google OAuth', true, true),
(3, 'github', 'GitHub OAuth', true, true),
(4, 'discord', 'Discord OAuth', true, true),
(5, 'binance', 'Binance API', false, true),
(6, 'coinbase', 'Coinbase API', false, true),
(7, 'kraken', 'Kraken API', false, true),
(8, 'axiom', 'Axiom Trade API', false, true)
ON CONFLICT (id) DO NOTHING;

INSERT INTO token_types (id, name, description, expiration) VALUES
(1, 'session', 'User session token', INTERVAL '24 hours'),
(2, 'oauth_access', 'OAuth access token', INTERVAL '1 hour'),
(3, 'oauth_refresh', 'OAuth refresh token', INTERVAL '30 days'),
(4, 'api_key_permanent', 'Permanent API key', INTERVAL '5 years'),
(5, 'api_key_monthly', 'Monthly API key', INTERVAL '30 days'),
(6, 'api_key_yearly', 'Yearly API key', INTERVAL '365 days'),
(7, 'api_key_temporary', 'Temporary API key', INTERVAL '7 days')
ON CONFLICT (id) DO NOTHING;

-- Triggers for updated_at
CREATE TRIGGER trigger_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trigger_linked_accounts_updated_at
    BEFORE UPDATE ON linked_accounts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trigger_tokens_updated_at
    BEFORE UPDATE ON tokens
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Default token expiry from token_types when not supplied
CREATE OR REPLACE FUNCTION tokens_set_default_expiry()
RETURNS trigger AS $$
BEGIN
  IF NEW.expires_at IS NULL THEN
     SELECT NOW() + tt.expiration INTO NEW.expires_at
     FROM token_types tt WHERE tt.id = NEW.type_id;
  END IF;
  RETURN NEW;
END; $$ LANGUAGE plpgsql;

CREATE TRIGGER trg_tokens_default_expiry
  BEFORE INSERT ON tokens
  FOR EACH ROW EXECUTE FUNCTION tokens_set_default_expiry();-- Useful view
CREATE OR REPLACE VIEW user_accounts_view AS
SELECT 
    u.id as user_id,
    u.primary_email,
    u.username as user_username,
    u.first_name,
    u.last_name,
    u.display_name as user_display_name,
    u.avatar_url as user_avatar_url,
    u.role,
    u.is_active as user_active,
    u.is_verified,
    u.created_at as user_created_at,
    u.last_login_at as user_last_login_at,

    la.id as linked_account_id,
    la.provider_id,
    la.provider_user_id,
    la.provider_email,
    la.provider_username,
    la.provider_display_name,
    la.provider_avatar_url,
    la.is_active as account_active,
    la.is_primary,
    la.connected_at,
    la.last_login_at as account_last_login_at,

    p.name as provider_name,
    p.display_name as provider_display_name_ref,
    p.is_oauth,

    t.id as token_id,
    t.user_id as token_user_id,
    t.linked_account_id as token_linked_account_id,
    t.value as token_value,
    t.expires_at as token_expires_at,
    t.last_used_at as token_last_used_at,

    tt.name as token_type,
    tt.description as token_type_description,

    array_agg(ts.scope) FILTER (WHERE ts.scope IS NOT NULL) as token_scopes
FROM users u
LEFT JOIN linked_accounts la ON u.id = la.user_id
LEFT JOIN providers p ON la.provider_id = p.id
LEFT JOIN tokens t ON (t.user_id = u.id OR t.linked_account_id = la.id)
LEFT JOIN token_types tt ON t.type_id = tt.id
LEFT JOIN token_scopes ts ON t.id = ts.token_id
GROUP BY 
    u.id, u.primary_email, u.username, u.first_name, u.last_name, u.display_name, 
    u.avatar_url, u.role, u.is_active, u.is_verified, u.created_at, u.last_login_at,
    la.id, la.provider_id, la.provider_user_id, la.provider_email, la.provider_username, 
    la.provider_display_name, la.provider_avatar_url, la.is_active, la.is_primary, 
    la.connected_at, la.last_login_at,
    p.name, p.display_name, p.is_oauth,
    t.id, t.user_id, t.linked_account_id, t.value, t.expires_at, t.last_used_at,
    tt.name, tt.description;

-- Safer, planner-friendly views (kept additive for compatibility)
CREATE OR REPLACE VIEW user_tokens_view AS
SELECT 
    u.id AS user_id,
    u.primary_email,
    t.id AS token_id,
    t.value,
    t.value_hash,
    t.expires_at,
    t.last_used_at,
    tt.name AS token_type,
    array_agg(ts.scope) FILTER (WHERE ts.scope IS NOT NULL) AS token_scopes
FROM users u
JOIN tokens t ON t.user_id = u.id
JOIN token_types tt ON t.type_id = tt.id
LEFT JOIN token_scopes ts ON ts.token_id = t.id
GROUP BY u.id, u.primary_email, t.id, t.value, t.value_hash, t.expires_at, t.last_used_at, tt.name;

CREATE OR REPLACE VIEW account_tokens_view AS
SELECT 
    u.id AS user_id,
    u.primary_email,
    la.id AS linked_account_id,
    la.provider_id,
    p.name AS provider_name,
    t.id AS token_id,
    t.value,
    t.value_hash,
    t.expires_at,
    t.last_used_at,
    tt.name AS token_type,
    array_agg(ts.scope) FILTER (WHERE ts.scope IS NOT NULL) AS token_scopes
FROM users u
JOIN linked_accounts la ON la.user_id = u.id
JOIN providers p ON p.id = la.provider_id
JOIN tokens t ON t.linked_account_id = la.id
JOIN token_types tt ON t.type_id = tt.id
LEFT JOIN token_scopes ts ON ts.token_id = t.id
GROUP BY u.id, u.primary_email, la.id, la.provider_id, p.name, t.id, t.value, t.value_hash, t.expires_at, t.last_used_at, tt.name;

-- Indexes
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_users_primary_email ON users(primary_email);
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_linked_accounts_user_id ON linked_accounts(user_id);
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_linked_accounts_provider_user ON linked_accounts(provider_id, provider_user_id);
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_linked_accounts_provider_email ON linked_accounts(provider_email);
CREATE UNIQUE INDEX CONCURRENTLY IF NOT EXISTS unique_primary_account_per_user ON linked_accounts(user_id) WHERE (is_primary);
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_tokens_user_id ON tokens(user_id);
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_tokens_linked_account_id ON tokens(linked_account_id);
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_tokens_expires_at ON tokens(expires_at);
-- Prefer deterministic hash column and unique index for lookups
CREATE UNIQUE INDEX CONCURRENTLY IF NOT EXISTS ux_tokens_value_hash ON tokens(value_hash);
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_audit_log_user_event ON audit_log(user_id, event);
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_audit_log_created_at ON audit_log(created_at);
-- Ensure only one active OAuth access (type_id=2) and one refresh (type_id=3) token per linked account.
-- Implemented as a PARTIAL UNIQUE INDEX (cannot be a named constraint in Postgres, so
-- application code must use: ON CONFLICT (linked_account_id, type_id) ... (NOT ON CONSTRAINT ...)
-- This works because inserts for type_id IN (2,3) satisfy the predicate and will see conflicts.
-- Session tokens (type_id=1, user scoped) and any future token types are unaffected.
-- IMPORTANT CHANGE (2025-08-13): Replaced prior PARTIAL UNIQUE INDEX (predicate type_id IN (2,3))
-- with a FULL UNIQUE INDEX on (linked_account_id, type_id). Reason: Postgres ON CONFLICT index
-- inference for "ON CONFLICT (linked_account_id, type_id)" requires a matching unique index
-- (partial indexes can be ignored if predicate inference fails in some environments/tools), which
-- led to runtime error: "there is no unique or exclusion constraint matching the ON CONFLICT specification".
-- A full unique index keeps desired semantics for account-scoped tokens: one row per token type per
-- linked account (NULL linked_account_id values for user-scoped tokens do NOT conflict because NULLs
-- are treated as distinct). This still permits multiple user-scoped session tokens (type_id=1) since
-- those rows have linked_account_id NULL.
-- If historical versions per type are later needed, drop this unique index and implement a soft
-- versioning strategy (e.g., add created_at to uniqueness or archive old rows before inserting new).
CREATE UNIQUE INDEX ux_tokens_linked_account_type ON tokens(linked_account_id, type_id);

-- NOTE: If future requirements necessitate multiple tokens of the same type per linked account
-- (e.g., rotating access tokens), replace this with a trigger that enforces max-active = 1 while
-- allowing archival, or include a status column and partial uniqueness on (linked_account_id, type_id)
-- WHERE status = 'active'.

-- Additional helpful indexes
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_linked_accounts_active ON linked_accounts(user_id) WHERE is_active;
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_tokens_not_expired ON tokens(expires_at) WHERE expires_at IS NOT NULL;
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_linked_accounts_profile_gin ON linked_accounts USING gin (provider_profile_data);

-- Comments
COMMENT ON TABLE users IS 'Core users table - represents actual people in your system';
COMMENT ON TABLE linked_accounts IS 'External/local accounts linked to users (multiple per provider allowed)';
COMMENT ON TABLE providers IS 'OAuth and API provider lookup table';
COMMENT ON TABLE token_types IS 'Token type definitions and metadata';
COMMENT ON TABLE tokens IS 'User-scoped (global) and account-scoped tokens (OAuth/API keys)';
COMMENT ON TABLE token_scopes IS 'Pivot table for token permissions/scopes';
COMMENT ON TABLE sessions_table IS 'Session storage for axum_session middleware';
COMMENT ON TABLE audit_log IS 'Security and compliance audit trail';

COMMENT ON VIEW user_accounts_view IS 'Users with their linked accounts and tokens (user- and account-scoped)';

COMMENT ON COLUMN linked_accounts.provider_user_id IS 'External provider unique ID (email for local, user_id for OAuth)';
COMMENT ON COLUMN tokens.value IS 'Encrypted token value - handles OAuth tokens, API keys, sessions';
COMMENT ON COLUMN token_types.expiration IS 'Default expiration interval for this token type';

-- Session storage table expected by application (simple key/value with expiry)
CREATE TABLE IF NOT EXISTS account_sessions (
    id TEXT PRIMARY KEY,
    data TEXT NOT NULL,
    expires BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_account_sessions_expires ON account_sessions(expires);
