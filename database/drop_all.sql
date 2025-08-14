-- Drop all tables, views, triggers, functions, types, and extensions
-- Order matters due to dependencies

-- 1) Drop views first (they depend on base tables)
DROP VIEW IF EXISTS user_accounts_view CASCADE;
DROP VIEW IF EXISTS user_tokens_view CASCADE;
DROP VIEW IF EXISTS account_tokens_view CASCADE;

-- 2) Drop standalone / supporting indexes explicitly (not strictly required because
--    DROP TABLE CASCADE removes them, but explicit drops ensure a clean slate if
--    a table recreation is skipped or partial cleanup is desired).
-- NOTE: Use IF EXISTS to avoid errors if some indexes were never created.
DROP INDEX IF EXISTS ux_tokens_linked_account_type;          -- unique (linked_account_id, type_id)
DROP INDEX IF EXISTS ux_tokens_value_hash;                   -- unique token hash
DROP INDEX IF EXISTS unique_primary_account_per_user;        -- partial unique index
DROP INDEX IF EXISTS idx_users_primary_email;
DROP INDEX IF EXISTS idx_users_username;
DROP INDEX IF EXISTS idx_linked_accounts_user_id;
DROP INDEX IF EXISTS idx_linked_accounts_provider_user;
DROP INDEX IF EXISTS idx_linked_accounts_provider_email;
DROP INDEX IF EXISTS idx_tokens_user_id;
DROP INDEX IF EXISTS idx_tokens_linked_account_id;
DROP INDEX IF EXISTS idx_tokens_expires_at;
DROP INDEX IF EXISTS idx_audit_log_user_event;
DROP INDEX IF EXISTS idx_audit_log_created_at;
DROP INDEX IF EXISTS idx_linked_accounts_active;
DROP INDEX IF EXISTS idx_tokens_not_expired;
DROP INDEX IF EXISTS idx_linked_accounts_profile_gin;
DROP INDEX IF EXISTS idx_account_sessions_expires;

-- 3) Drop tables (CASCADE handles foreign key dependencies, triggers, and remaining indexes)
-- Core and auxiliary tables (CASCADE ensures dependent objects like indexes & FKs are removed)
DROP TABLE IF EXISTS audit_log CASCADE;
DROP TABLE IF EXISTS sessions_table CASCADE;
DROP TABLE IF EXISTS account_sessions CASCADE; -- simple session KV store
DROP TABLE IF EXISTS token_scopes CASCADE;
DROP TABLE IF EXISTS tokens CASCADE;           -- value/value_hash & account/user scoped tokens
DROP TABLE IF EXISTS linked_accounts CASCADE;
DROP TABLE IF EXISTS token_types CASCADE;
DROP TABLE IF EXISTS providers CASCADE;
DROP TABLE IF EXISTS users CASCADE;

-- Drop functions
DROP FUNCTION IF EXISTS update_updated_at_column() CASCADE;
DROP FUNCTION IF EXISTS tokens_set_default_expiry() CASCADE;

-- Drop custom types
DROP TYPE IF EXISTS ROLE_USER CASCADE;

-- Drop extensions
DROP EXTENSION IF EXISTS citext CASCADE;
DROP EXTENSION IF EXISTS pgcrypto CASCADE;
