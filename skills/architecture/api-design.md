---
name: engram-api-design
description: "Design REST/GraphQL/gRPC APIs with proper versioning, pagination, filtering, error handling, and rate limiting. Document contracts and ensure backward compatibility."
---

# API Design (Engram-Integrated)

## Overview

Design robust, well-documented APIs by systematically defining resources, endpoints, request/response formats, error handling, and versioning strategies. Store API specifications, design decisions, and evolution plans in Engram for long-term API governance.

## When to Use

Use this skill when:
- Designing a new API from scratch
- Adding new endpoints to an existing API
- Planning API versioning strategy
- Defining error response formats
- Designing pagination and filtering for large datasets
- Planning rate limiting and authentication
- Ensuring backward compatibility during API evolution
- Creating API documentation for internal or external consumers

## The Pattern

### Step 1: Define API Resources and Operations

Start by identifying resources and CRUD operations:

```bash
engram context create \
  --title "API Design: [API Name] - Resources" \
  --content "## API Style\n\n**Choice:** REST (Resource-Oriented)\n\n**Alternative Considered:**\n- GraphQL: More flexible queries but adds complexity\n- gRPC: Better performance but less universal (requires protobuf)\n\n**Rationale:** REST is simple, widely understood, and sufficient for use case.\n\n## Core Resources\n\n### Resource: User\n\n**Endpoints:**\n\n**Create User**\n- Method: POST\n- Path: /api/v1/users\n- Auth: Public (registration)\n- Request Body:\n  \`\`\`json\n  {\"email\": \"user@example.com\", \"password\": \"secret\", \"name\": \"Alice\"}\n  \`\`\`\n- Response: 201 Created\n  \`\`\`json\n  {\"id\": \"550e8400-...\", \"email\": \"user@example.com\", \"name\": \"Alice\", \"created_at\": \"2026-01-24T12:00:00Z\"}\n  \`\`\`\n\n**Get User**\n- Method: GET\n- Path: /api/v1/users/{id}\n- Auth: Required (JWT)\n- Response: 200 OK\n  \`\`\`json\n  {\"id\": \"550e8400-...\", \"email\": \"user@example.com\", \"name\": \"Alice\"}\n  \`\`\`\n\n**Update User**\n- Method: PATCH\n- Path: /api/v1/users/{id}\n- Auth: Required (own account or admin)\n- Request Body (partial update):\n  \`\`\`json\n  {\"name\": \"Alice Smith\"}\n  \`\`\`\n- Response: 200 OK\n  \`\`\`json\n  {\"id\": \"550e8400-...\", \"email\": \"user@example.com\", \"name\": \"Alice Smith\"}\n  \`\`\`\n\n**Delete User**\n- Method: DELETE\n- Path: /api/v1/users/{id}\n- Auth: Required (own account or admin)\n- Response: 204 No Content\n\n**List Users**\n- Method: GET\n- Path: /api/v1/users\n- Auth: Required (admin only)\n- Query Params: ?page=1&limit=20&sort=created_at:desc\n- Response: 200 OK\n  \`\`\`json\n  {\n    \"data\": [{\"id\": \"...\", \"email\": \"...\", \"name\": \"...\"}],\n    \"pagination\": {\"page\": 1, \"limit\": 20, \"total\": 150, \"total_pages\": 8}\n  }\n  \`\`\`\n\n### Resource: Document\n\n**Endpoints:**\n\n**Create Document**\n- POST /api/v1/documents\n- Auth: Required\n- Request: {\"title\": \"...\", \"content\": \"...\"}\n- Response: 201 Created\n\n**Get Document**\n- GET /api/v1/documents/{id}\n- Auth: Required (owner or shared with)\n- Response: 200 OK\n\n**Update Document**\n- PATCH /api/v1/documents/{id}\n- Auth: Required (owner or write permission)\n- Request: {\"title\": \"New Title\"}\n- Response: 200 OK\n\n**Delete Document**\n- DELETE /api/v1/documents/{id}\n- Auth: Required (owner only)\n- Response: 204 No Content\n\n**List Documents**\n- GET /api/v1/documents\n- Auth: Required\n- Filters: ?owner_id={id}&visibility=public\n- Response: 200 OK with pagination\n\n**Share Document**\n- POST /api/v1/documents/{id}/shares\n- Auth: Required (owner only)\n- Request: {\"user_id\": \"...\", \"permission\": \"read\"}\n- Response: 201 Created\n\n## Resource Hierarchy\n\n**Nested Resources:**\n\n- /api/v1/users/{user_id}/documents (documents owned by user)\n- /api/v1/documents/{doc_id}/shares (shares for document)\n- /api/v1/documents/{doc_id}/versions (version history)\n\n**Rationale:** Nested routes show resource relationships clearly.\n\n**Trade-off:** Deep nesting (>2 levels) can be unwieldy. Use query params instead:\n- Don't: /api/v1/orgs/{org_id}/users/{user_id}/documents/{doc_id}/comments\n- Do: /api/v1/comments?document_id={doc_id}\n\n## HTTP Method Semantics\n\n**GET:** Read resource (idempotent, cacheable)\n**POST:** Create resource (not idempotent)\n**PUT:** Replace entire resource (idempotent)\n**PATCH:** Update partial resource (idempotent)\n**DELETE:** Remove resource (idempotent)\n\n**Idempotency:** Calling N times has same effect as calling once.\n\n**Example:**\n- POST /users (not idempotent) → creates N users if called N times\n- PUT /users/{id} (idempotent) → same result whether called 1x or N times\n- DELETE /users/{id} (idempotent) → user deleted (subsequent calls: 404)" \
  --source "api-design" \
  --tags "api-design,resources,[api-name]"
```

### Step 2: Design Request and Response Formats

Define consistent formats for all endpoints:

```bash
engram reasoning create \
  --title "API Design: [API Name] - Request/Response Format" \
  --task-id [TASK_ID] \
  --content "## Request Format\n\n### Content-Type\n\n**Standard:** application/json (UTF-8)\n\n**Headers:**\n\`\`\`\nContent-Type: application/json\nAuthorization: Bearer {jwt_token}\n\`\`\`\n\n### Request Body Schema\n\n**Use JSON Schema for validation:**\n\n\`\`\`json\n{\n  \"type\": \"object\",\n  \"properties\": {\n    \"title\": {\"type\": \"string\", \"minLength\": 1, \"maxLength\": 200},\n    \"content\": {\"type\": \"string\", \"maxLength\": 50000},\n    \"visibility\": {\"type\": \"string\", \"enum\": [\"private\", \"public\", \"shared\"]}\n  },\n  \"required\": [\"title\"]\n}\n\`\`\`\n\n**Validation Errors:** Return 400 Bad Request with details\n\n### Query Parameters\n\n**Pagination:**\n- page (default: 1)\n- limit (default: 20, max: 100)\n\n**Sorting:**\n- sort=field:order (e.g., sort=created_at:desc)\n- Multiple: sort=created_at:desc,title:asc\n\n**Filtering:**\n- field=value (e.g., status=active)\n- Operators: field[gt]=100 (greater than), field[like]=%keyword%\n\n**Searching:**\n- q=keyword (full-text search)\n\n**Field Selection:**\n- fields=id,title,created_at (return only specified fields)\n\n**Example:**\n\`\`\`\nGET /api/v1/documents?page=2&limit=50&sort=created_at:desc&status=published&q=tutorial\n\`\`\`\n\n## Response Format\n\n### Success Response\n\n**Single Resource:**\n\n\`\`\`json\n{\n  \"id\": \"550e8400-e29b-41d4-a716-446655440000\",\n  \"title\": \"My Document\",\n  \"content\": \"...\",\n  \"owner_id\": \"...\",\n  \"created_at\": \"2026-01-24T12:00:00Z\",\n  \"updated_at\": \"2026-01-24T13:00:00Z\"\n}\n\`\`\`\n\n**Collection (with Pagination):**\n\n\`\`\`json\n{\n  \"data\": [\n    {\"id\": \"...\", \"title\": \"...\"},\n    {\"id\": \"...\", \"title\": \"...\"}\n  ],\n  \"pagination\": {\n    \"page\": 2,\n    \"limit\": 20,\n    \"total\": 150,\n    \"total_pages\": 8\n  },\n  \"links\": {\n    \"first\": \"/api/v1/documents?page=1&limit=20\",\n    \"prev\": \"/api/v1/documents?page=1&limit=20\",\n    \"next\": \"/api/v1/documents?page=3&limit=20\",\n    \"last\": \"/api/v1/documents?page=8&limit=20\"\n  }\n}\n\`\`\`\n\n**Alternative: Cursor-Based Pagination**\n\nFor infinite scroll or real-time feeds:\n\n\`\`\`json\n{\n  \"data\": [...],\n  \"pagination\": {\n    \"next_cursor\": \"eyJpZCI6IjU1MGU4NDAwIiwiY3JlYXRlZF9hdCI6IjIwMjYtMDEtMjQifQ==\",\n    \"has_more\": true\n  }\n}\n\`\`\`\n\n**Rationale:** Cursor pagination handles concurrent updates better (no missing items when page shifts).\n\n### Error Response\n\n**Standard Format:**\n\n\`\`\`json\n{\n  \"error\": {\n    \"code\": \"VALIDATION_ERROR\",\n    \"message\": \"Request validation failed\",\n    \"details\": [\n      {\"field\": \"email\", \"message\": \"Invalid email format\"},\n      {\"field\": \"password\", \"message\": \"Password must be at least 8 characters\"}\n    ],\n    \"request_id\": \"req_abc123\"\n  }\n}\n\`\`\`\n\n**Error Codes (Application-Level):**\n- VALIDATION_ERROR: Invalid input\n- AUTHENTICATION_REQUIRED: Missing or invalid token\n- PERMISSION_DENIED: Insufficient permissions\n- RESOURCE_NOT_FOUND: Resource doesn't exist\n- RATE_LIMIT_EXCEEDED: Too many requests\n- INTERNAL_ERROR: Server error (500)\n\n### HTTP Status Codes\n\n**Success:**\n- 200 OK: Request succeeded (GET, PATCH)\n- 201 Created: Resource created (POST)\n- 204 No Content: Success with no body (DELETE)\n\n**Client Errors:**\n- 400 Bad Request: Invalid syntax or validation error\n- 401 Unauthorized: Missing or invalid authentication\n- 403 Forbidden: Authenticated but not authorized\n- 404 Not Found: Resource doesn't exist\n- 409 Conflict: State conflict (duplicate email)\n- 422 Unprocessable Entity: Semantic validation error\n- 429 Too Many Requests: Rate limit exceeded\n\n**Server Errors:**\n- 500 Internal Server Error: Unexpected server error\n- 503 Service Unavailable: Temporary outage\n\n**Rationale:** Use specific status codes for better client error handling.\n\n## Response Headers\n\n**Common Headers:**\n\n\`\`\`\nContent-Type: application/json; charset=utf-8\nX-Request-ID: req_abc123\nX-RateLimit-Limit: 1000\nX-RateLimit-Remaining: 999\nX-RateLimit-Reset: 1706097600\nCache-Control: no-cache (or: public, max-age=300)\nETag: \"33a64df551425fcc55e4d42a148795d9f25f89d4\"\n\`\`\`\n\n**Conditional Requests:**\n\n\`\`\`\nGET /api/v1/documents/123\nIf-None-Match: \"33a64df551425fcc55e4d42a148795d9f25f89d4\"\n\n→ 304 Not Modified (if ETag matches)\n→ 200 OK with body (if ETag changed)\n\`\`\`\n\n**Rationale:** Reduces bandwidth and improves performance.\n\n## API Consistency Guidelines\n\n**Field Naming:**\n- Use snake_case (created_at, user_id)\n- Be consistent across all endpoints\n\n**Date/Time Format:**\n- ISO 8601 UTC (2026-01-24T12:00:00Z)\n- Never use Unix timestamps in JSON (not human-readable)\n\n**IDs:**\n- Use UUIDs (not sequential integers)\n- Prevents enumeration attacks\n- Allows distributed ID generation\n\n**Boolean Fields:**\n- Use true/false (not 1/0 or \"true\"/\"false\" strings)\n\n**Null vs. Absent:**\n- null: Field exists but has no value\n- Absent: Field not included in response\n- Use absent for optional fields not set\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "api-design,format,[api-name]"
```

### Step 3: Design API Versioning Strategy

Plan how API evolves over time:

```bash
engram reasoning create \
  --title "API Design: [API Name] - Versioning Strategy" \
  --task-id [TASK_ID] \
  --content "## Versioning Approach\n\n**Strategy: URL Path Versioning**\n\n**Format:** /api/v{major}/resource\n\n**Example:**\n- /api/v1/users\n- /api/v2/users\n\n**Alternatives Considered:**\n\n**Header Versioning:**\n\`\`\`\nAccept: application/vnd.example.v1+json\n\`\`\`\n\n**Pros:** Clean URLs\n**Cons:** Less visible, harder to test (can't just paste URL in browser)\n\n**Query Parameter:**\n\`\`\`\n/api/users?version=1\n\`\`\`\n\n**Pros:** Flexible\n**Cons:** Easy to forget, conflicts with other query params\n\n**Decision: URL Path Versioning**\n\n**Rationale:**\n- Most visible and explicit\n- Easy to test (curl, browser)\n- Clear separation of versions (can deploy different code paths)\n\n## When to Increment Version\n\n**Major Version (v1 → v2):**\n\nBreaking changes:\n- Remove endpoint\n- Remove field from response\n- Change field type (string → integer)\n- Change endpoint behavior significantly\n- Rename field\n\n**Minor Version (v1.1 → v1.2):**\n\nBackward-compatible changes (don't increment major version):\n- Add new endpoint\n- Add new optional field to request\n- Add new field to response (clients ignore unknown fields)\n- Add new query parameter\n- Deprecate field (but still return it)\n\n**Patch Version (v1.1.0 → v1.1.1):**\n\nBug fixes, no API changes.\n\n## Backward Compatibility Rules\n\n**Safe Changes (No Version Bump):**\n\n1. Add new endpoint: POST /api/v1/analytics\n2. Add optional request field: {\"title\": \"...\", \"tags\": []} (tags is optional)\n3. Add response field: {\"id\": \"...\", \"created_at\": \"...\", \"updated_at\": \"...\"} (added updated_at)\n4. Make required field optional: password becomes optional (for OAuth users)\n\n**Breaking Changes (Require Major Version):**\n\n1. Remove endpoint: DELETE /api/v1/legacy is removed\n2. Remove response field: No longer return email in user object\n3. Change field type: user_id changes from string to integer\n4. Rename field: name becomes full_name\n5. Change semantics: PATCH now replaces instead of merging\n\n## Deprecation Process\n\n**Step 1: Announce Deprecation**\n\n- Add header to responses:\n  \`\`\`\n  Deprecation: true\n  Sunset: Sat, 31 Dec 2026 23:59:59 GMT\n  Link: <https://api.example.com/docs/v2>; rel=\"successor-version\"\n  \`\`\`\n\n- Add to API documentation\n- Email all API consumers\n\n**Step 2: Grace Period**\n\nMaintain v1 for at least 6-12 months after v2 release.\n\n**Step 3: Sunset**\n\n- Return 410 Gone for deprecated endpoints\n- Or redirect to v2 if possible (301 Moved Permanently)\n\n**Example:**\n\n\`\`\`json\n{\n  \"error\": {\n    \"code\": \"ENDPOINT_SUNSET\",\n    \"message\": \"This endpoint was deprecated and is no longer available. Please use v2.\",\n    \"sunset_date\": \"2026-12-31\",\n    \"successor\": \"https://api.example.com/v2/users\"\n  }\n}\n\`\`\`\n\n## Supporting Multiple Versions\n\n**Architecture:**\n\n**Option 1: Separate Code Paths**\n\n\`\`\`\napp/\n  controllers/\n    v1/\n      users_controller.rb\n    v2/\n      users_controller.rb\n\`\`\`\n\n**Pros:** Complete isolation, easy to deprecate v1\n**Cons:** Code duplication\n\n**Option 2: Shared Logic with Versioned Serializers**\n\n\`\`\`\napp/\n  controllers/\n    users_controller.rb  # Shared logic\n  serializers/\n    v1/\n      user_serializer.rb  # Returns {id, name, email}\n    v2/\n      user_serializer.rb  # Returns {id, full_name, email}\n\`\`\`\n\n**Pros:** Less duplication, shared business logic\n**Cons:** Must be careful not to break v1 when changing shared code\n\n**Decision: Shared Logic with Versioned Serializers**\n\n**Rationale:** Business logic rarely changes between versions, mostly response format changes.\n\n## Testing Multi-Version API\n\n**Integration Tests:**\n\n\`\`\`ruby\ndescribe \"GET /api/v1/users/:id\" do\n  it \"returns user with name field\" do\n    response = get(\"/api/v1/users/#{user.id}\")\n    expect(response).to include(\"name\")\n  end\nend\n\ndescribe \"GET /api/v2/users/:id\" do\n  it \"returns user with full_name field\" do\n    response = get(\"/api/v2/users/#{user.id}\")\n    expect(response).to include(\"full_name\")\n    expect(response).not_to include(\"name\")\n  end\nend\n\`\`\`\n\n**Contract Tests:**\n\nUse tools like Pact or Spring Cloud Contract to ensure API contract stability.\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "api-design,versioning,[api-name]"
```

### Step 4: Design Rate Limiting

Define request limits and throttling:

```bash
engram reasoning create \
  --title "API Design: [API Name] - Rate Limiting" \
  --task-id [TASK_ID] \
  --content "## Rate Limiting Strategy\n\n**Why Rate Limit:**\n- Prevent abuse (DDoS, scraping)\n- Ensure fair resource distribution\n- Protect backend from overload\n- Monetization (free vs paid tiers)\n\n## Rate Limit Tiers\n\n**Anonymous (No Auth):**\n- 10 requests per minute per IP\n- Applies to: Registration, login\n\n**Authenticated (Free Tier):**\n- 1,000 requests per hour per user\n- Burst: 50 requests per minute\n\n**Authenticated (Paid Tier):**\n- 10,000 requests per hour per user\n- Burst: 200 requests per minute\n\n**Admin:**\n- No limit (trusted internal use)\n\n## Algorithm: Token Bucket\n\n**How It Works:**\n\n1. Each user has a bucket with N tokens\n2. Tokens refill at rate R per second\n3. Each request consumes 1 token\n4. If bucket empty, request rejected (429)\n\n**Configuration:**\n\n\`\`\`yaml\nfree_tier:\n  capacity: 1000  # Max tokens\n  refill_rate: 0.277  # tokens per second (1000/hour)\n  burst_capacity: 50  # Max burst\n\`\`\`\n\n**Example:**\n\n- User starts with 1000 tokens\n- Makes 50 requests in 1 minute (burst) → 950 tokens left\n- Waits 3 minutes → 950 + (0.277 * 180) = 1000 tokens (capped)\n\n**Implementation (Redis):**\n\n\`\`\`python\nimport redis\nimport time\n\ndef check_rate_limit(user_id, capacity=1000, refill_rate=0.277):\n    r = redis.Redis()\n    key = f\"rate_limit:{user_id}\"\n    \n    current = r.get(key)\n    if current is None:\n        # First request, initialize bucket\n        r.setex(key, 3600, capacity - 1)\n        return True\n    \n    tokens = float(current)\n    if tokens >= 1:\n        # Has tokens, consume one\n        r.decr(key)\n        return True\n    else:\n        # Out of tokens\n        return False\n\n# Token refill in background job (every second)\ndef refill_tokens():\n    for user_id in get_active_users():\n        key = f\"rate_limit:{user_id}\"\n        r.incrbyfloat(key, refill_rate)\n        # Cap at capacity\n        if float(r.get(key)) > capacity:\n            r.set(key, capacity)\n\`\`\`\n\n## Rate Limit Headers\n\n**Response Headers:**\n\n\`\`\`\nX-RateLimit-Limit: 1000\nX-RateLimit-Remaining: 950\nX-RateLimit-Reset: 1706097600\n\`\`\`\n\n**When Rate Limited:**\n\n\`\`\`\nHTTP/1.1 429 Too Many Requests\nRetry-After: 60\nX-RateLimit-Limit: 1000\nX-RateLimit-Remaining: 0\nX-RateLimit-Reset: 1706097600\n\n{\n  \"error\": {\n    \"code\": \"RATE_LIMIT_EXCEEDED\",\n    \"message\": \"Rate limit exceeded. Try again in 60 seconds.\",\n    \"retry_after\": 60\n  }\n}\n\`\`\`\n\n## Per-Endpoint Rate Limits\n\n**Different Limits for Different Endpoints:**\n\n**Expensive Operations:**\n- POST /api/v1/exports → 5 per hour\n- POST /api/v1/ai/generate → 20 per hour\n\n**Cheap Operations:**\n- GET /api/v1/users/{id} → 1000 per hour\n\n**Implementation:**\n\nSeparate buckets per endpoint:\n\n\`\`\`\nrate_limit:user123:global → 1000/hour\nrate_limit:user123:exports → 5/hour\nrate_limit:user123:ai_generate → 20/hour\n\`\`\`\n\n## Rate Limit by Cost\n\n**Alternative: Assign Cost to Each Endpoint**\n\n- GET /users/{id} → 1 credit\n- POST /documents → 5 credits\n- POST /exports → 100 credits\n\n**User has credit budget:** 1000 credits per hour\n\n**Benefit:** More flexible, users can make trade-offs\n\n## Bypass Rate Limiting\n\n**Scenarios:**\n\n1. Internal services (service-to-service calls)\n2. Background jobs\n3. Load testing\n\n**Implementation:**\n\nSpecial header or token:\n\n\`\`\`\nX-Internal-Service: backend-worker\nAuthorization: Bearer internal-service-token\n\`\`\`\n\n**Security:** Internal tokens must not be exposed to clients.\n\n## Monitoring\n\n**Metrics to Track:**\n- Rate limit hits per user (identify abusers)\n- 429 response rate (if high, limits too strict)\n- Top users by request count\n\n**Alerts:**\n- Single user exceeds 90% of limit frequently → potential abuse\n- Global 429 rate > 5% → limits too strict, consider increasing\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "api-design,rate-limiting,[api-name]"
```

### Step 5: Design Error Handling

Define comprehensive error responses:

```bash
engram reasoning create \
  --title "API Design: [API Name] - Error Handling" \
  --task-id [TASK_ID] \
  --content "## Error Response Philosophy\n\n**Principles:**\n1. Consistent format across all endpoints\n2. Machine-readable codes (for client logic)\n3. Human-readable messages (for debugging)\n4. Detailed field-level errors (for validation)\n5. Request ID for tracing (for support)\n\n## Standard Error Format\n\n\`\`\`json\n{\n  \"error\": {\n    \"code\": \"VALIDATION_ERROR\",\n    \"message\": \"Request validation failed\",\n    \"details\": [\n      {\"field\": \"email\", \"code\": \"INVALID_FORMAT\", \"message\": \"Must be valid email\"},\n      {\"field\": \"password\", \"code\": \"TOO_SHORT\", \"message\": \"Must be at least 8 characters\"}\n    ],\n    \"request_id\": \"req_abc123\",\n    \"timestamp\": \"2026-01-24T12:00:00Z\"\n  }\n}\n\`\`\`\n\n## Error Codes\n\n### 400 Bad Request\n\n**VALIDATION_ERROR**\n\n\`\`\`json\n{\n  \"error\": {\n    \"code\": \"VALIDATION_ERROR\",\n    \"message\": \"Request validation failed\",\n    \"details\": [{\"field\": \"email\", \"code\": \"INVALID_FORMAT\"}]\n  }\n}\n\`\`\`\n\n**INVALID_JSON**\n\n\`\`\`json\n{\n  \"error\": {\n    \"code\": \"INVALID_JSON\",\n    \"message\": \"Request body is not valid JSON\",\n    \"details\": [{\"line\": 3, \"column\": 15, \"message\": \"Unexpected token\"}]\n  }\n}\n\`\`\`\n\n### 401 Unauthorized\n\n**AUTHENTICATION_REQUIRED**\n\n\`\`\`json\n{\n  \"error\": {\n    \"code\": \"AUTHENTICATION_REQUIRED\",\n    \"message\": \"Authentication required. Please provide a valid access token.\"\n  }\n}\n\`\`\`\n\n**INVALID_TOKEN**\n\n\`\`\`json\n{\n  \"error\": {\n    \"code\": \"INVALID_TOKEN\",\n    \"message\": \"Access token is invalid or expired\",\n    \"details\": [{\"reason\": \"TOKEN_EXPIRED\", \"expired_at\": \"2026-01-24T11:00:00Z\"}]\n  }\n}\n\`\`\`\n\n### 403 Forbidden\n\n**PERMISSION_DENIED**\n\n\`\`\`json\n{\n  \"error\": {\n    \"code\": \"PERMISSION_DENIED\",\n    \"message\": \"You do not have permission to perform this action\",\n    \"details\": [\n      {\"required_permission\": \"documents:write\", \"user_permissions\": [\"documents:read\"]}\n    ]\n  }\n}\n\`\`\`\n\n### 404 Not Found\n\n**RESOURCE_NOT_FOUND**\n\n\`\`\`json\n{\n  \"error\": {\n    \"code\": \"RESOURCE_NOT_FOUND\",\n    \"message\": \"The requested resource was not found\",\n    \"details\": [{\"resource_type\": \"document\", \"resource_id\": \"550e8400-...\"}]\n  }\n}\n\`\`\`\n\n### 409 Conflict\n\n**DUPLICATE_RESOURCE**\n\n\`\`\`json\n{\n  \"error\": {\n    \"code\": \"DUPLICATE_RESOURCE\",\n    \"message\": \"A resource with this identifier already exists\",\n    \"details\": [{\"field\": \"email\", \"value\": \"user@example.com\", \"existing_id\": \"...\"}]\n  }\n}\n\`\`\`\n\n**OPTIMISTIC_LOCK_ERROR**\n\n\`\`\`json\n{\n  \"error\": {\n    \"code\": \"OPTIMISTIC_LOCK_ERROR\",\n    \"message\": \"Resource was modified by another request. Please retry.\",\n    \"details\": [{\"expected_version\": 5, \"actual_version\": 6}]\n  }\n}\n\`\`\`\n\n### 429 Too Many Requests\n\n**RATE_LIMIT_EXCEEDED**\n\n(See Rate Limiting section)\n\n### 500 Internal Server Error\n\n**INTERNAL_ERROR**\n\n\`\`\`json\n{\n  \"error\": {\n    \"code\": \"INTERNAL_ERROR\",\n    \"message\": \"An unexpected error occurred. Please try again later.\",\n    \"request_id\": \"req_abc123\"\n  }\n}\n\`\`\`\n\n**Note:** Never expose stack traces or internal details to clients (security risk).\n\n### 503 Service Unavailable\n\n**SERVICE_UNAVAILABLE**\n\n\`\`\`json\n{\n  \"error\": {\n    \"code\": \"SERVICE_UNAVAILABLE\",\n    \"message\": \"Service is temporarily unavailable. Please try again later.\",\n    \"retry_after\": 60\n  }\n}\n\`\`\`\n\n## Field-Level Validation Errors\n\n**Validation Codes:**\n\n- REQUIRED: Field is required but missing\n- INVALID_FORMAT: Wrong format (email, URL, date)\n- TOO_SHORT: String/array below min length\n- TOO_LONG: String/array above max length\n- OUT_OF_RANGE: Number outside allowed range\n- INVALID_ENUM: Value not in allowed list\n- INVALID_TYPE: Wrong type (expected string, got number)\n\n**Example:**\n\n\`\`\`json\n{\n  \"error\": {\n    \"code\": \"VALIDATION_ERROR\",\n    \"message\": \"Request validation failed\",\n    \"details\": [\n      {\n        \"field\": \"email\",\n        \"code\": \"REQUIRED\",\n        \"message\": \"Email is required\"\n      },\n      {\n        \"field\": \"age\",\n        \"code\": \"OUT_OF_RANGE\",\n        \"message\": \"Age must be between 0 and 120\",\n        \"min\": 0,\n        \"max\": 120,\n        \"actual\": 150\n      },\n      {\n        \"field\": \"tags\",\n        \"code\": \"TOO_LONG\",\n        \"message\": \"Maximum 10 tags allowed\",\n        \"max\": 10,\n        \"actual\": 15\n      }\n    ]\n  }\n}\n\`\`\`\n\n## Client Error Handling\n\n**Retry Logic:**\n\n**Retryable Errors (Transient):**\n- 429 Too Many Requests (wait retry_after seconds)\n- 500 Internal Server Error (exponential backoff)\n- 503 Service Unavailable (wait retry_after seconds)\n- Network errors (timeout, connection refused)\n\n**Non-Retryable Errors (Permanent):**\n- 400 Bad Request (fix request before retrying)\n- 401 Unauthorized (refresh token or re-login)\n- 403 Forbidden (user lacks permission, don't retry)\n- 404 Not Found (resource doesn't exist)\n\n**Example Retry Logic:**\n\n\`\`\`python\ndef call_api(url, max_retries=3):\n    for attempt in range(max_retries):\n        try:\n            response = requests.get(url)\n            \n            if response.status_code == 200:\n                return response.json()\n            elif response.status_code == 429:\n                retry_after = int(response.headers.get('Retry-After', 60))\n                time.sleep(retry_after)\n            elif response.status_code >= 500:\n                # Exponential backoff: 1s, 2s, 4s\n                time.sleep(2 ** attempt)\n            else:\n                # Don't retry 4xx errors\n                raise Exception(f\"API error: {response.status_code}\")\n        except requests.exceptions.RequestException:\n            # Network error, retry with backoff\n            time.sleep(2 ** attempt)\n    \n    raise Exception(\"Max retries exceeded\")\n\`\`\`\n\n## Logging and Monitoring\n\n**What to Log:**\n\n**All Errors:**\n- request_id (for tracing)\n- User ID (if authenticated)\n- Endpoint and method\n- Status code\n- Error code\n- Stack trace (server-side only, never expose to client)\n\n**Example Log:**\n\n\`\`\`json\n{\n  \"level\": \"error\",\n  \"timestamp\": \"2026-01-24T12:00:00Z\",\n  \"request_id\": \"req_abc123\",\n  \"user_id\": \"550e8400-...\",\n  \"method\": \"POST\",\n  \"path\": \"/api/v1/documents\",\n  \"status_code\": 500,\n  \"error_code\": \"INTERNAL_ERROR\",\n  \"error_message\": \"Database connection failed\",\n  \"stack_trace\": \"...\"\n}\n\`\`\`\n\n**Metrics:**\n- Error rate per endpoint (5xx / total requests)\n- Error rate per user (identify problematic clients)\n- Top error codes (prioritize fixes)\n\n**Alerts:**\n- 5xx error rate > 1% (trigger incident)\n- Single endpoint error rate > 5% (specific issue)\n- Error rate spike (sudden increase)\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "api-design,error-handling,[api-name]"
```

### Step 6: Link All API Design Entities

```bash
# Link all API design documents to task
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [RESOURCES_ID] --target-type context \
  --relationship-type references --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [FORMAT_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [VERSIONING_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [RATE_LIMITING_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [ERROR_HANDLING_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]
```

## Example

User wants to design a REST API for a task management system.

### Step 1: Define Resources

```bash
RESOURCES=$(engram context create \
  --title "API Design: Task Management API - Resources" \
  --content "## Core Resources\n\n### Resource: Task\n\n**Endpoints:**\n\n**Create Task:**\n- POST /api/v1/tasks\n- Auth: Required\n- Request: {\"title\": \"Fix bug\", \"description\": \"...\", \"due_date\": \"2026-01-30\", \"priority\": \"high\"}\n- Response: 201 Created\n\n**Get Task:**\n- GET /api/v1/tasks/{id}\n- Auth: Required (owner or collaborator)\n- Response: 200 OK\n\n**Update Task:**\n- PATCH /api/v1/tasks/{id}\n- Auth: Required (owner or collaborator with write permission)\n- Request: {\"status\": \"completed\"}\n- Response: 200 OK\n\n**Delete Task:**\n- DELETE /api/v1/tasks/{id}\n- Auth: Required (owner only)\n- Response: 204 No Content\n\n**List Tasks:**\n- GET /api/v1/tasks\n- Auth: Required\n- Filters: ?status=active&priority=high&due_before=2026-01-30\n- Response: 200 OK with pagination\n\n### Resource: Comment\n\n**Create Comment:**\n- POST /api/v1/tasks/{task_id}/comments\n- Auth: Required\n- Request: {\"content\": \"Great work!\"}\n- Response: 201 Created\n\n**List Comments:**\n- GET /api/v1/tasks/{task_id}/comments\n- Auth: Required\n- Response: 200 OK with pagination\n\n### Resource: Attachment\n\n**Upload Attachment:**\n- POST /api/v1/tasks/{task_id}/attachments\n- Auth: Required\n- Content-Type: multipart/form-data\n- Response: 201 Created" \
  --source "api-design" \
  --tags "api-design,resources,task-management" \
  --json | jq -r '.id')
```

## Querying API Designs

After creating API designs, agents can retrieve:

```bash
# Get all API design documents
engram context list | grep "API Design"

# Get versioning strategies
engram reasoning list | grep "Versioning"

# Get all API design reasoning for a task
engram relationship connected --entity-id [TASK_ID] --relationship-type documents | grep -i "api"
```

## Related Skills

This skill integrates with:
- `engram-system-design` - API is part of system architecture
- `engram-data-modeling` - API endpoints map to data entities
- `engram-security-architecture` - API authentication, authorization, rate limiting
- `engram-api-docs` - Generate comprehensive API documentation from design
- `engram-test-driven-development` - Test API endpoints and contracts
