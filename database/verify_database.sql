-- Database verification and data inspection script
-- Shows table row counts, sample data, and basic statistics

\echo '=== DATABASE VERIFICATION SCRIPT ==='
\echo 'Checking table structure and data presence...'
\echo ''

-- Show all tables
\echo '=== TABLES IN DATABASE ==='
\dt
\echo ''

-- Show all views
\echo '=== VIEWS IN DATABASE ==='
\dv
\echo ''

-- Show extensions
\echo '=== INSTALLED EXTENSIONS ==='
\dx
\echo ''

-- Check row counts for all tables
\echo '=== TABLE ROW COUNTS ==='
SELECT 
    schemaname,
    tablename,
    n_tup_ins AS "Total Inserts",
    n_tup_upd AS "Total Updates", 
    n_tup_del AS "Total Deletes",
    n_live_tup AS "Live Rows",
    n_dead_tup AS "Dead Rows"
FROM pg_stat_user_tables 
ORDER BY tablename;
\echo ''

-- Simple row count query
\echo '=== SIMPLE ROW COUNTS ==='
SELECT 'users' AS table_name, COUNT(*) AS row_count FROM users
UNION ALL
SELECT 'providers', COUNT(*) FROM providers
UNION ALL
SELECT 'token_types', COUNT(*) FROM token_types
UNION ALL
SELECT 'linked_accounts', COUNT(*) FROM linked_accounts
UNION ALL
SELECT 'tokens', COUNT(*) FROM tokens
UNION ALL
SELECT 'token_scopes', COUNT(*) FROM token_scopes
UNION ALL
SELECT 'sessions_table', COUNT(*) FROM sessions_table
UNION ALL
SELECT 'audit_log', COUNT(*) FROM audit_log
ORDER BY table_name;
\echo ''

-- Check seed data
\echo '=== PROVIDERS (SEED DATA) ==='
SELECT id, name, display_name, is_oauth, is_active FROM providers ORDER BY id;
\echo ''

\echo '=== TOKEN TYPES (SEED DATA) ==='
SELECT id, name, description, expiration FROM token_types ORDER BY id;
\echo ''

-- Check for any user data
\echo '=== USERS DATA ==='
SELECT 
    id,
    primary_email,
    username,
    first_name,
    last_name,
    role,
    is_active,
    is_verified,
    created_at
FROM users 
ORDER BY created_at DESC
LIMIT 10;
\echo ''

-- Check for linked accounts
\echo '=== LINKED ACCOUNTS DATA ==='
SELECT 
    la.id,
    la.user_id,
    p.name as provider_name,
    la.provider_user_id,
    la.provider_email,
    la.is_active,
    la.is_primary,
    la.connected_at
FROM linked_accounts la
JOIN providers p ON la.provider_id = p.id
ORDER BY la.connected_at DESC
LIMIT 10;
\echo ''

-- Check for tokens
\echo '=== TOKENS DATA ==='
SELECT 
    t.id,
    t.user_id,
    t.linked_account_id,
    tt.name as token_type,
    LEFT(t.value, 20) || '...' as token_preview,
    t.value_hash IS NOT NULL as has_hash,
    t.value_encrypted IS NOT NULL as has_encrypted,
    t.expires_at,
    t.created_at
FROM tokens t
JOIN token_types tt ON t.type_id = tt.id
ORDER BY t.created_at DESC
LIMIT 10;
\echo ''

-- Check for sessions
\echo '=== SESSIONS DATA ==='
SELECT 
    session_id,
    user_id,
    token_id,
    expires_at,
    created_at,
    user_agent IS NOT NULL as has_user_agent,
    ip_address
FROM sessions_table
ORDER BY created_at DESC
LIMIT 10;
\echo ''

-- Check for audit log entries
\echo '=== AUDIT LOG DATA ==='
SELECT 
    id,
    user_id,
    linked_account_id,
    event,
    success,
    ip_address,
    created_at
FROM audit_log
ORDER BY created_at DESC
LIMIT 10;
\echo ''

-- Check indexes
\echo '=== TABLE INDEXES ==='
SELECT 
    schemaname,
    tablename,
    indexname,
    indexdef
FROM pg_indexes 
WHERE schemaname = 'public'
ORDER BY tablename, indexname;
\echo ''

-- Check triggers
\echo '=== TABLE TRIGGERS ==='
SELECT 
    event_object_table AS table_name,
    trigger_name,
    event_manipulation AS trigger_event,
    action_timing
FROM information_schema.triggers 
WHERE trigger_schema = 'public'
ORDER BY event_object_table, trigger_name;
\echo ''

\echo '=== VERIFICATION COMPLETE ==='
