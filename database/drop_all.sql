-- Drop all tables, views, triggers, functions, types, and extensions
-- Order matters due to dependencies

-- Drop views first (depend on tables)
DROP VIEW IF EXISTS user_accounts_view CASCADE;
DROP VIEW IF EXISTS user_tokens_view CASCADE;
DROP VIEW IF EXISTS account_tokens_view CASCADE;

-- Drop tables (CASCADE handles foreign key dependencies)
DROP TABLE IF EXISTS audit_log CASCADE;
DROP TABLE IF EXISTS sessions_table CASCADE;
DROP TABLE IF EXISTS token_scopes CASCADE;
DROP TABLE IF EXISTS tokens CASCADE;
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
