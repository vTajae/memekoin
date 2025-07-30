# Authentication System Implementation Guide

## Overview

This Leptos fullstack application includes a complete authentication system with the following features:

### Frontend (leptos-wasm)
- Login and Registration forms with Tailwind CSS styling
- Password visibility toggle
- Remember me checkbox
- Form validation and error handling
- Loading states with spinners
- Protected routes
- Session persistence using localStorage
- Automatic navigation after login/register

### Backend (axum-worker)
- User registration and login endpoints
- Password hashing (simplified SHA256 for Cloudflare Workers compatibility)
- JWT token generation and validation
- In-memory user storage with demo user
- Protected endpoints with Bearer token authentication

## Usage

### Starting the Development Server

```bash
powershell -ExecutionPolicy Bypass -File dev.ps1
```

The dev server includes auto-reload functionality for both frontend and backend changes.

### Demo Credentials

- Username: `demo`
- Password: `password`

### API Endpoints

- `POST /api/auth/register` - Register a new user
- `POST /api/auth/login` - Login with credentials
- `GET /api/auth/me` - Get current user info (requires auth token)

## Known Issues and Solutions

### Build Issues

1. **UUID Compilation Error**: Fixed by adding the "js" feature to uuid dependency
2. **Ring/Argon2 WASM Compatibility**: Replaced with SHA256 for simplicity
3. **Leptos 0.8 API Changes**: Updated to use `attr:class` instead of `class` for `<A>` components

### Frontend Structure

- `/src/auth.rs` - Authentication context and API functions
- `/src/components/login.rs` - Login and Register form components
- `/src/components/navbar.rs` - Navigation bar with auth state display and ProtectedRoute
- `/src/pages/home.rs` - Home page with auth-aware content

### Backend Structure

- `/src/auth.rs` - Authentication logic, user management, JWT handling
- `/src/lib.rs` - API routes and request handlers

## Security Notes

⚠️ **Important**: This implementation uses simplified password hashing (SHA256) for Cloudflare Workers compatibility. In production:
- Use a proper password hashing algorithm (Argon2, bcrypt, or scrypt)
- Implement proper salt generation
- Use secure JWT secrets from environment variables
- Add rate limiting and CSRF protection
- Implement proper session management

## Deployment

The app is designed to work with Cloudflare Workers:

1. Build the frontend: `cd leptos-wasm && trunk build --release`
2. Deploy to Cloudflare: `cd axum-worker && npx wrangler deploy`

## Future Enhancements

- Add password reset functionality
- Implement OAuth providers
- Add user profile management
- Implement refresh tokens
- Add email verification
- Improve error handling and user feedback