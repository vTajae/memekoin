-- Insert demo user with bcrypt hashed password for "password"
-- The hash below is for the password "password"
INSERT INTO users (id, username, email, password_hash, created_at, updated_at) 
VALUES (
    'demo-user-id-12345678-1234-1234-1234-123456789012'::uuid,
    'demo',
    'demo@example.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/RK.s5uO.G',
    NOW(),
    NOW()
) ON CONFLICT (username) DO NOTHING;

-- Verify the user was created
SELECT id, username, email, created_at FROM users WHERE username = 'demo';
