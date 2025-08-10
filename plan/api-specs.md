# API Specifications

## Overview

Comprehensive REST API specifications for the Leptos fullstack Rust Cloudflare worker application with Google OAuth integration.

## Base Configuration

```yaml
base_url: https://your-worker.workers.dev/api
content_type: application/json
authentication: Cookie-based sessions
versioning: Path-based (/api/v1/) for future versions
cors: Enabled with credentials support
```

## Standard Response Formats

### Success Response
```typescript
interface ApiResponse<T> {
  success: true;
  data: T | null;
  message?: string;
  timestamp: string; // ISO 8601 format
}
```

### Error Response
```typescript
interface ApiError {
  success: false;
  error: {
    code: string;           // ERROR_CODE_FORMAT
    message: string;        // Human-readable message
    details?: string;       // Technical details
  };
  timestamp: string;        // ISO 8601 format
}
```

### Example Responses
```json
// Success with data
{
  "success": true,
  "data": {
    "id": "user-123",
    "email": "user@example.com"
  },
  "timestamp": "2024-01-15T10:30:00Z"
}

// Success without data
{
  "success": true,
  "data": null,
  "message": "Operation completed successfully",
  "timestamp": "2024-01-15T10:30:00Z"
}

// Error response
{
  "success": false,
  "error": {
    "code": "AUTH_INVALID_TOKEN",
    "message": "Authentication token is invalid or expired",
    "details": "Token expired at 2024-01-15T09:30:00Z"
  },
  "timestamp": "2024-01-15T10:30:00Z"
}
```

## Authentication Endpoints

### OAuth Login Initiation
```http
GET /api/auth/oauth/login
```

**Query Parameters:**
- `redirect_after` (optional): URL to redirect after successful login

**Response:**
- **302 Found**: Redirect to Google OAuth authorization URL
- **400 Bad Request**: Invalid redirect URL

**Example:**
```bash
curl -X GET "https://your-worker.workers.dev/api/auth/oauth/login?redirect_after=/dashboard"
```

### OAuth Callback
```http
GET /api/auth/oauth/callback
```

**Query Parameters:**
- `code` (required): Authorization code from Google
- `state` (required): CSRF protection state parameter
- `error` (optional): Error code if authorization failed
- `error_description` (optional): Human-readable error description

**Response:**
- **302 Found**: Redirect to dashboard or specified redirect URL
- **400 Bad Request**: Invalid authorization code or state
- **401 Unauthorized**: OAuth flow failed

**Sets Cookie:**
- `session_id`: Secure HTTP-only session cookie

### Get Current User
```http
GET /api/auth/user
```

**Authentication:** Required (session cookie)

**Response:**
```typescript
interface UserResponse {
  success: true;
  data: {
    id: string;
    email: string;
    name?: string;
    picture?: string;
    verified_email: boolean;
    created_at: string;
    updated_at: string;
  };
  timestamp: string;
}
```

**Status Codes:**
- **200 OK**: User information returned
- **401 Unauthorized**: Invalid or missing session

**Example:**
```bash
curl -X GET "https://your-worker.workers.dev/api/auth/user" \
  -H "Cookie: session_id=your-session-id"
```

### Refresh Token
```http
POST /api/auth/refresh
```

**Authentication:** Required (session cookie)

**Response:**
```typescript
interface TokenRefreshResponse {
  success: true;
  data: {
    expires_in: number;     // Token lifetime in seconds
    token_type: string;     // "Bearer"
  };
  timestamp: string;
}
```

**Status Codes:**
- **200 OK**: Token refreshed successfully
- **401 Unauthorized**: Invalid session or refresh token
- **403 Forbidden**: Refresh token expired

### Logout
```http
POST /api/auth/logout
```

**Authentication:** Required (session cookie)

**Response:**
```typescript
interface LogoutResponse {
  success: true;
  data: null;
  message: "Logged out successfully";
  timestamp: string;
}
```

**Status Codes:**
- **200 OK**: Logout successful
- **401 Unauthorized**: No active session

**Clears Cookies:**
- `session_id`: Session cookie removed

### Get User Sessions
```http
GET /api/auth/sessions
```

**Authentication:** Required (session cookie)

**Response:**
```typescript
interface SessionsResponse {
  success: true;
  data: Array<{
    session_id: string;
    created_at: string;
    last_activity: string;
    user_agent?: string;
    ip_address?: string;
    is_current: boolean;
  }>;
  timestamp: string;
}
```

**Status Codes:**
- **200 OK**: Sessions retrieved
- **401 Unauthorized**: Invalid session

### End Specific Session
```http
DELETE /api/auth/sessions/{session_id}
```

**Authentication:** Required (session cookie)

**Path Parameters:**
- `session_id`: ID of session to terminate

**Response:**
```typescript
interface EndSessionResponse {
  success: true;
  data: null;
  message: "Session ended successfully";
  timestamp: string;
}
```

**Status Codes:**
- **200 OK**: Session ended
- **401 Unauthorized**: Invalid session
- **404 Not Found**: Session not found
- **403 Forbidden**: Cannot end session belonging to another user

## Health Check Endpoints

### Basic Health Check
```http
GET /api/health
```

**Authentication:** Not required

**Response:**
```typescript
interface HealthResponse {
  success: true;
  data: {
    status: "healthy" | "degraded" | "unhealthy";
    timestamp: string;
  };
  timestamp: string;
}
```

**Status Codes:**
- **200 OK**: Service is healthy
- **503 Service Unavailable**: Service is unhealthy

### Detailed Health Check
```http
GET /api/health/detailed
```

**Authentication:** Not required

**Response:**
```typescript
interface DetailedHealthResponse {
  success: true;
  data: {
    status: "healthy" | "degraded" | "unhealthy";
    version: string;
    environment: string;
    services: {
      database: "healthy" | "degraded" | "unhealthy";
      oauth: "healthy" | "unhealthy";
      session_store: "healthy" | "degraded" | "unhealthy";
    };
    metrics: {
      uptime: number;           // Uptime in seconds
      requests_total: number;   // Total requests processed
      active_sessions: number;  // Current active sessions
    };
  };
  timestamp: string;
}
```

## Error Codes Reference

### Authentication Errors
```yaml
AUTH_INVALID_CREDENTIALS:
  status: 401
  message: "Invalid credentials provided"

AUTH_MISSING_TOKEN:
  status: 401
  message: "Authentication token is missing"

AUTH_INVALID_TOKEN:
  status: 401
  message: "Authentication token is invalid or expired"

AUTH_USER_NOT_FOUND:
  status: 404
  message: "User not found"

AUTH_FORBIDDEN:
  status: 403
  message: "Insufficient permissions"

AUTH_OAUTH_ERROR:
  status: 400
  message: "OAuth authentication failed"

AUTH_SESSION_ERROR:
  status: 401
  message: "Session error occurred"
```

### General Errors
```yaml
VALIDATION_ERROR:
  status: 400
  message: "Validation failed"

DATABASE_ERROR:
  status: 500
  message: "Database operation failed"

EXTERNAL_SERVICE_ERROR:
  status: 502
  message: "External service unavailable"

CONFIG_ERROR:
  status: 500
  message: "Configuration error"

INTERNAL_SERVER_ERROR:
  status: 500
  message: "Internal server error"
```

## Request/Response Headers

### Standard Request Headers
```yaml
Content-Type: application/json
Accept: application/json
Cookie: session_id=your-session-id  # For authenticated requests
User-Agent: YourApp/1.0.0
```

### Standard Response Headers
```yaml
Content-Type: application/json
Cache-Control: no-store              # Prevent caching of sensitive data
X-Content-Type-Options: nosniff      # Security header
X-Frame-Options: DENY                # Prevent clickjacking
X-XSS-Protection: 1; mode=block      # XSS protection
```

### CORS Headers
```yaml
Access-Control-Allow-Origin: https://your-domain.com
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS
Access-Control-Allow-Headers: Content-Type, Authorization, Cookie
Access-Control-Allow-Credentials: true
Access-Control-Max-Age: 3600
```

## Rate Limiting (Future Implementation)

### Rate Limit Headers
```yaml
X-RateLimit-Limit: 100              # Requests per window
X-RateLimit-Remaining: 95           # Remaining requests
X-RateLimit-Reset: 1642248000       # Unix timestamp when window resets
X-RateLimit-Window: 3600            # Window size in seconds
```

### Rate Limit Response
```json
{
  "success": false,
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Too many requests. Please try again later.",
    "details": "Rate limit: 100 requests per hour"
  },
  "timestamp": "2024-01-15T10:30:00Z"
}
```

## Future Gmail API Endpoints

### List Messages
```http
GET /api/gmail/messages
```

**Query Parameters:**
- `q` (optional): Gmail search query
- `maxResults` (optional): Maximum number of results (default: 10)
- `pageToken` (optional): Page token for pagination

### Get Message
```http
GET /api/gmail/messages/{messageId}
```

**Path Parameters:**
- `messageId`: Gmail message ID

### Send Message
```http
POST /api/gmail/messages/send
```

**Request Body:**
```typescript
interface SendMessageRequest {
  to: string[];
  cc?: string[];
  bcc?: string[];
  subject: string;
  body: string;
  isHtml?: boolean;
}
```

### List Labels
```http
GET /api/gmail/labels
```

**Response:**
```typescript
interface LabelsResponse {
  success: true;
  data: Array<{
    id: string;
    name: string;
    type: "system" | "user";
    messageListVisibility: string;
    labelListVisibility: string;
  }>;
  timestamp: string;
}
```

## Testing Examples

### Authentication Flow Test
```bash
# 1. Initiate OAuth login
curl -X GET "https://your-worker.workers.dev/api/auth/oauth/login" \
  -i

# 2. Complete OAuth flow (manual step in browser)

# 3. Check authenticated user
curl -X GET "https://your-worker.workers.dev/api/auth/user" \
  -H "Cookie: session_id=your-session-id" \
  -i

# 4. Logout
curl -X POST "https://your-worker.workers.dev/api/auth/logout" \
  -H "Cookie: session_id=your-session-id" \
  -i
```

### Health Check Test
```bash
# Basic health check
curl -X GET "https://your-worker.workers.dev/api/health" \
  -H "Accept: application/json"

# Detailed health check
curl -X GET "https://your-worker.workers.dev/api/health/detailed" \
  -H "Accept: application/json"
```

### Error Handling Test
```bash
# Test invalid endpoint
curl -X GET "https://your-worker.workers.dev/api/nonexistent" \
  -H "Accept: application/json"

# Test unauthorized access
curl -X GET "https://your-worker.workers.dev/api/auth/user" \
  -H "Accept: application/json"
```

## OpenAPI Specification

```yaml
openapi: 3.0.0
info:
  title: Leptos Cloudflare Worker API
  version: 1.0.0
  description: REST API for Leptos fullstack application with Google OAuth
servers:
  - url: https://your-worker.workers.dev/api
    description: Production server
paths:
  /auth/oauth/login:
    get:
      summary: Initiate OAuth login
      parameters:
        - name: redirect_after
          in: query
          schema:
            type: string
      responses:
        '302':
          description: Redirect to Google OAuth
        '400':
          description: Bad request
  # Additional endpoint definitions...
```

This API specification provides a comprehensive reference for all endpoints, request/response formats, error handling, and integration patterns for the Leptos fullstack application.