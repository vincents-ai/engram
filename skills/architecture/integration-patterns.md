---
name: engram-integration-patterns
description: "Design service integrations using sync/async patterns, events, queues, API Gateway, BFF, CQRS, and Event Sourcing."
---

# Integration Patterns (Engram-Integrated)

## Overview

Design integration strategies for connecting services, systems, and external APIs. Choose appropriate patterns based on latency requirements, consistency needs, scalability constraints, and failure modes. Store integration decisions and trade-offs in Engram for architectural reference.

## When to Use

Use this skill when:
- Connecting microservices or distributed systems
- Integrating with external APIs or third-party services
- Designing data synchronization between systems
- Choosing between sync and async communication
- Implementing event-driven architectures
- Designing API gateways or backend-for-frontend patterns
- Evaluating CQRS or Event Sourcing architectures
- Planning for system decoupling and scalability

## The Pattern

### Step 1: Analyze Integration Requirements

Define what needs to be integrated:

```bash
engram context create \
  --title "Integration Requirements: [System A] ↔ [System B]" \
  --content "## Systems to Integrate\n\n**Source System:** [System A]\n- Purpose: [What it does]\n- Technology: [Language, framework]\n- Location: [Same datacenter, different region, external]\n- Ownership: [Your team, different team, third party]\n\n**Target System:** [System B]\n- Purpose: [What it does]\n- Technology: [Language, framework]\n- Location: [Location]\n- Ownership: [Owner]\n\n## Integration Needs\n\n**Data Flow:**\n- Direction: [A → B, B → A, bidirectional]\n- Data Type: [User data, events, commands, queries]\n- Volume: [Requests per second]\n- Size: [Bytes per message]\n\n**Consistency Requirements:**\n- Strong consistency needed? [Yes/No - why?]\n- Eventual consistency acceptable? [Yes/No - max staleness?]\n- Order preservation needed? [Yes/No]\n\n**Latency Requirements:**\n- Synchronous response needed? [Yes/No]\n- Max acceptable latency: [e.g., 100ms, 1s, 1 minute, eventual]\n- User-facing? [Yes/No - affects latency tolerance]\n\n**Reliability Requirements:**\n- Can tolerate data loss? [Yes/No]\n- Retries needed? [Yes/No - how many?]\n- Idempotency required? [Yes/No]\n- Dead letter queue? [Yes/No]\n\n**Scalability:**\n- Expected growth: [e.g., 10x in 1 year]\n- Peak traffic: [e.g., 10K requests/sec]\n- Backpressure handling: [How to handle overload]\n\n**Failure Modes:**\n- What if System B is down? [Block, queue, drop]\n- What if network partition? [Retry, fail, queue]\n- What if System A is down? [System B impact]\n\n**Security:**\n- Authentication: [API key, OAuth, mTLS]\n- Authorization: [Role-based, scope-based]\n- Data sensitivity: [Public, internal, confidential]" \
  --source "integration-patterns" \
  --tags "integration,requirements,[system-a],[system-b]"
```

### Step 2: Choose Integration Style

Evaluate sync vs async:

```bash
engram reasoning create \
  --title "Integration Style: [System A] ↔ [System B]" \
  --task-id [TASK_ID] \
  --content "## Integration Styles Evaluated\n\n### Synchronous (Request-Response)\n\n**Pattern:** Client waits for response\n**Protocol:** HTTP/REST, gRPC, GraphQL\n**Coupling:** Tight - both systems must be available\n**Latency:** Low (milliseconds to seconds)\n**Complexity:** Low - simple to implement and debug\n\n**Use When:**\n- User needs immediate response\n- Operation is fast (< 100ms)\n- Strong consistency required\n- Simple request-response flow\n- Failure can be handled by retry immediately\n\n**Avoid When:**\n- Long-running operations (> 1s)\n- Target system unreliable\n- Need to decouple sender and receiver\n- High volume could overwhelm receiver\n\n**Example:** Get user profile, Validate credit card, Check inventory\n\n### Asynchronous (Message-Based)\n\n**Pattern:** Fire-and-forget or eventual response\n**Protocol:** Message queue (RabbitMQ, SQS), Event stream (Kafka, Kinesis)\n**Coupling:** Loose - systems can be independently available\n**Latency:** Higher (seconds to minutes)\n**Complexity:** Medium - need to handle eventual consistency\n\n**Use When:**\n- Long-running operations (> 1s)\n- User doesn't need immediate response\n- High volume (need backpressure)\n- Target system may be temporarily unavailable\n- Multiple consumers need same message\n- Order preservation needed\n\n**Avoid When:**\n- User needs immediate response\n- Debugging complexity is concern\n- Team unfamiliar with async patterns\n\n**Example:** Send email, Process payment (async confirmation), Generate report\n\n### Event-Driven\n\n**Pattern:** Publish events, multiple subscribers react\n**Protocol:** Event bus (Kafka, EventBridge, Pub/Sub)\n**Coupling:** Very loose - publishers don't know subscribers\n**Latency:** Higher (milliseconds to seconds)\n**Complexity:** High - need event schema, versioning, replay\n\n**Use When:**\n- Multiple downstream systems interested in same event\n- Need audit trail of all changes\n- Building event sourcing system\n- Decoupling producers from consumers\n- Enabling future unknown use cases\n\n**Avoid When:**\n- Simple point-to-point integration sufficient\n- Event schema evolution is concern\n- Team unfamiliar with event-driven architecture\n\n**Example:** User registered (→ send welcome email, create account, log analytics)\n\n## Recommendation for [System A] ↔ [System B]\n\n**Selected Style:** [Synchronous/Asynchronous/Event-Driven]\n\n**Rationale:**\n[Why this choice based on requirements above]\n\n**Trade-offs:**\n- Pros: [Benefits of this choice]\n- Cons: [Drawbacks accepted]\n\n**Implementation:**\n- Protocol: [HTTP, gRPC, SQS, Kafka]\n- Format: [JSON, Protobuf, Avro]\n- Authentication: [Method]\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "integration,style,[system-a],[system-b]"
```

### Step 3: Select Integration Pattern

Choose architectural pattern:

```bash
engram reasoning create \
  --title "Integration Pattern: [Use Case]" \
  --task-id [TASK_ID] \
  --content "## Integration Patterns Evaluated\n\n### API Gateway\n\n**Purpose:** Single entry point for all client requests, routing to backend services\n\n**Components:**\n```\nClient → API Gateway → Service A\n                    → Service B\n                    → Service C\n```\n\n**Responsibilities:**\n- Request routing\n- Authentication/authorization\n- Rate limiting\n- Request/response transformation\n- Protocol translation (HTTP → gRPC)\n- Caching\n- Logging/monitoring\n\n**Use When:**\n- Multiple backend services\n- Need centralized auth, rate limiting\n- Clients are external (web, mobile)\n- Need to aggregate responses from multiple services\n- Want to decouple client from backend changes\n\n**Technology:** Kong, AWS API Gateway, Nginx, Envoy\n\n**Trade-offs:**\n- Pros: Simplified client, centralized control, security boundary\n- Cons: Single point of failure, additional latency, can become bottleneck\n\n### Backend for Frontend (BFF)\n\n**Purpose:** Dedicated backend for each client type (web, mobile, IoT)\n\n**Components:**\n```\nWeb Client → Web BFF → Services\nMobile App → Mobile BFF → Services\nIoT Device → IoT BFF → Services\n```\n\n**Responsibilities:**\n- Client-specific logic\n- Aggregate multiple service calls\n- Transform data for client needs\n- Handle client-specific auth\n\n**Use When:**\n- Different client types with different needs\n- Web needs different data than mobile\n- Want to optimize payload size per client\n- Client teams want autonomy\n\n**Trade-offs:**\n- Pros: Optimized per client, team ownership, independent evolution\n- Cons: Code duplication, more services to maintain\n\n### CQRS (Command Query Responsibility Segregation)\n\n**Purpose:** Separate read and write models for different optimization\n\n**Components:**\n```\nClient\n  ├── Commands → Command Handler → Write Model (SQL)\n  └── Queries → Query Handler → Read Model (NoSQL, cache)\n                                     ↑\n                         Events from Write Model\n```\n\n**Pattern:**\n- Commands: Change state (create, update, delete)\n- Queries: Read state (get, list, search)\n- Write model: Optimized for consistency, transactions\n- Read model: Optimized for query performance, denormalized\n- Sync: Events propagate from write to read model\n\n**Use When:**\n- Read and write patterns are very different\n- High read volume, low write volume (or vice versa)\n- Complex queries that don't map to write model\n- Need to scale reads and writes independently\n- Building event sourcing system\n\n**Avoid When:**\n- Simple CRUD application\n- Read and write models are similar\n- Eventual consistency not acceptable\n- Team unfamiliar with pattern\n\n**Trade-offs:**\n- Pros: Independently scalable, optimized models, better performance\n- Cons: Eventual consistency, complexity, data duplication\n\n### Event Sourcing\n\n**Purpose:** Store all changes as sequence of events instead of current state\n\n**Components:**\n```\nCommand → Aggregate → Event → Event Store\n                               ↓\n                          Event Handler → Projections (Read Models)\n```\n\n**Pattern:**\n- All state changes stored as events\n- Current state derived by replaying events\n- Events are immutable, append-only\n- Projections built from event stream\n\n**Example:**\n```\nEvents: [UserRegistered, EmailUpdated, ProfileCompleted]\nCurrent State: Replay all events to rebuild user state\n```\n\n**Use When:**\n- Need full audit trail\n- Complex business logic with many state transitions\n- Time-travel queries (state at any point in past)\n- Multiple read models from same events\n- Building distributed systems with eventual consistency\n\n**Avoid When:**\n- Simple CRUD sufficient\n- No need for audit trail\n- Query current state only\n- Team unfamiliar with event sourcing\n\n**Technology:** EventStore, Kafka, Custom\n\n**Trade-offs:**\n- Pros: Complete history, audit trail, time-travel, flexible read models\n- Cons: Complexity, eventual consistency, event schema evolution\n\n### Message Queue\n\n**Purpose:** Decouple sender and receiver with async message delivery\n\n**Components:**\n```\nProducer → Queue → Consumer\n              ↓\n          Dead Letter Queue (failures)\n```\n\n**Guarantees:**\n- At-most-once: May lose messages (fast, unreliable)\n- At-least-once: May duplicate messages (reliable, need idempotency)\n- Exactly-once: No loss or duplication (slowest, complex)\n\n**Use When:**\n- Decouple services\n- Handle traffic spikes (backpressure)\n- Retry failed operations\n- Guarantee delivery\n\n**Technology:** RabbitMQ, AWS SQS, Redis Queue\n\n### Event Streaming\n\n**Purpose:** Append-only log of events with multiple consumers\n\n**Components:**\n```\nProducer → Topic (partitioned) → Consumer Group A\n                              → Consumer Group B\n                              → Consumer Group C\n```\n\n**Characteristics:**\n- Events retained for time period (replay possible)\n- Ordered within partition\n- Multiple consumers read same events\n- Scales by partitioning\n\n**Use When:**\n- Multiple consumers need same events\n- Need to replay events\n- High throughput (millions of events/sec)\n- Building event-driven architecture\n\n**Technology:** Kafka, AWS Kinesis, Pulsar\n\n### Service Mesh\n\n**Purpose:** Infrastructure layer for service-to-service communication\n\n**Components:**\n```\nService A → Sidecar Proxy → Network → Sidecar Proxy → Service B\n```\n\n**Responsibilities:**\n- Service discovery\n- Load balancing\n- Retries, timeouts, circuit breaking\n- mTLS encryption\n- Observability (metrics, traces)\n\n**Use When:**\n- Many microservices (> 10)\n- Need consistent networking policies\n- Multi-language services\n- Complex routing (canary, blue-green)\n\n**Technology:** Istio, Linkerd, Consul Connect\n\n**Trade-offs:**\n- Pros: Consistent networking, language-agnostic, powerful traffic management\n- Cons: Complexity, operational overhead, latency from proxy\n\n## Recommendation for [Use Case]\n\n**Selected Pattern:** [Pattern name]\n\n**Architecture:**\n```\n[ASCII diagram of chosen pattern]\n```\n\n**Rationale:**\n[Why this pattern fits requirements]\n\n**Implementation Steps:**\n1. [Step 1]\n2. [Step 2]\n3. [Step 3]\n\n**Migration Strategy (if applicable):**\n[How to migrate from current to new pattern]\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "integration,pattern,[use-case]"
```

### Step 4: Design Data Flow

Map data flow through integration:

```bash
engram context create \
  --title "Integration Data Flow: [Use Case]" \
  --content "## Request Flow\n\n**Synchronous Example:**\n```\n1. Client sends GET /users/123\n2. API Gateway authenticates request (50ms)\n3. Gateway routes to User Service (10ms)\n4. User Service queries database (30ms)\n5. User Service returns response (10ms)\n6. Gateway returns to client\nTotal: 100ms\n```\n\n**Asynchronous Example:**\n```\n1. Client sends POST /orders\n2. API receives request, validates (20ms)\n3. API publishes OrderCreated event to queue\n4. API returns 202 Accepted to client (with order_id)\nTotal: 20ms (client response)\n\nBackground:\n5. Order Service consumes OrderCreated event\n6. Order Service calls Payment Service (500ms)\n7. Payment Service calls Stripe API (800ms)\n8. Payment Service publishes PaymentCompleted event\n9. Order Service updates order status\n10. Notification Service sends email\nTotal: 2-3 seconds (background processing)\n```\n\n**Event-Driven Example:**\n```\n1. User Service publishes UserRegistered event\n2. Event stored in Kafka topic\n3. Multiple consumers process event:\n   - Email Service → Send welcome email (2s)\n   - Analytics Service → Track signup (100ms)\n   - CRM Service → Create contact (500ms)\n   - Notification Service → Send push notification (300ms)\n4. Each consumer processes independently\n```\n\n## Data Format\n\n**Request Format:**\n```json\n{\n  \"user_id\": \"user_123\",\n  \"action\": \"purchase\",\n  \"amount\": 99.99,\n  \"currency\": \"USD\",\n  \"timestamp\": \"2026-01-24T10:30:00Z\"\n}\n```\n\n**Response Format:**\n```json\n{\n  \"status\": \"success\",\n  \"transaction_id\": \"txn_456\",\n  \"message\": \"Payment processed\"\n}\n```\n\n**Event Schema:**\n```json\n{\n  \"event_id\": \"evt_789\",\n  \"event_type\": \"OrderCreated\",\n  \"event_version\": \"1.0\",\n  \"timestamp\": \"2026-01-24T10:30:00Z\",\n  \"source\": \"order-service\",\n  \"payload\": {\n    \"order_id\": \"order_123\",\n    \"user_id\": \"user_456\",\n    \"items\": [...],\n    \"total\": 99.99\n  }\n}\n```\n\n## Error Handling\n\n**Retry Strategy:**\n- Max retries: 3\n- Backoff: Exponential (1s, 2s, 4s)\n- Timeout: 5s per attempt\n- After retries exhausted: Send to dead letter queue\n\n**Circuit Breaker:**\n- Failure threshold: 50% errors in 10s window\n- Open circuit: Return cached response or error\n- Half-open: Try 1 request after 30s\n- Close circuit: If 5 consecutive successes\n\n**Fallback:**\n- Primary: Call Service B\n- Fallback 1: Return cached data\n- Fallback 2: Return degraded response\n- Fallback 3: Return error with retry-after header\n\n## Performance\n\n**Latency Budget:**\n- Client to Gateway: 10ms\n- Gateway to Service: 20ms\n- Service processing: 50ms\n- Database query: 20ms\n- Total: 100ms (p50 target)\n\n**Throughput:**\n- Current: 1K requests/sec\n- Peak: 5K requests/sec\n- Scale target: 10K requests/sec\n\n**Bottlenecks:**\n- Gateway: Scales to 10K req/sec per instance\n- Service: Scales to 5K req/sec per instance\n- Database: Limited to 10K queries/sec (need read replicas)\n\n## Monitoring\n\n**Metrics:**\n- integration_requests_total {source, target, status}\n- integration_duration_seconds {source, target}\n- integration_errors_total {source, target, error_type}\n- queue_depth {queue_name}\n- event_lag_seconds {consumer_group}\n\n**Traces:**\n- Trace ID propagated across all service calls\n- Each service adds span with timing\n- Failed requests include error context\n\n**Logs:**\n- Log all integration requests/responses\n- Include trace_id for correlation\n- Sanitize sensitive data" \
  --source "integration-patterns" \
  --tags "integration,data-flow,[use-case]"
```

### Step 5: Link Integration Design

```bash
# Link all integration docs to task
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [REQUIREMENTS_ID] --target-type context \
  --relationship-type references --agent default

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [STYLE_ID] --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [PATTERN_ID] --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [DATAFLOW_ID] --target-type context \
  --relationship-type references --agent default
```

## Example

User needs to integrate e-commerce checkout with payment processing.

### Step 1: Requirements

```bash
REQ=$(engram context create \
  --title "Integration Requirements: Checkout ↔ Payment" \
  --content "## Systems\n\nSource: Checkout Service (Node.js, internal)\nTarget: Payment Service (Rust, internal) → Stripe API (external)\n\n## Integration Needs\n\nData Flow: Checkout → Payment (one-way)\nVolume: 100 payments/minute peak\nLatency: User waits for confirmation (< 3s acceptable)\nConsistency: Strong (money involved)\nReliability: No data loss, retries needed, idempotent\n\n## Failure Modes\n\nPayment Service down → Queue payment for later, show user pending status\nStripe API timeout → Retry with backoff, max 3 attempts\nCheckout Service down → User retries, payment must be idempotent" \
  --source "integration-patterns" \
  --tags "integration,requirements,checkout,payment" \
  --json | jq -r '.id')
```

### Step 2: Integration Style

```bash
STYLE=$(engram reasoning create \
  --title "Integration Style: Checkout ↔ Payment" \
  --task-id checkout-integration-123 \
  --content "## Selected: Asynchronous (Message Queue)\n\nRationale:\n- User can wait 2-3 seconds for confirmation (not instant)\n- Payment Service may be temporarily overloaded\n- Stripe API is external (unreliable network)\n- Need guaranteed delivery (money involved)\n- Want to decouple services\n\nImplementation:\n- Checkout publishes PaymentRequested message to SQS\n- Payment Service consumes from queue\n- Payment Service calls Stripe API\n- Payment Service publishes PaymentCompleted event\n- Checkout Service listens for PaymentCompleted\n- Checkout updates order status and notifies user\n\nUX:\n- User clicks Pay\n- Checkout shows Spinner: Processing payment...\n- After 2s: Show success or pending status\n- If pending: Email sent when payment completes\n\nProtocol: AWS SQS (at-least-once delivery)\nFormat: JSON\nAuth: IAM roles (both services in same AWS account)" \
  --confidence 0.85 \
  --tags "integration,style,checkout,payment" \
  --json | jq -r '.id')
```

### Step 3: Integration Pattern

```bash
PATTERN=$(engram reasoning create \
  --title "Integration Pattern: Checkout Payment Flow" \
  --task-id checkout-integration-123 \
  --content "## Selected: Message Queue + Event-Driven\n\nArchitecture:\n```\nCheckout Service\n    ↓ (publish)\nSQS Queue: payment-requests\n    ↓ (consume)\nPayment Service\n    ↓ (call)\nStripe API\n    ↓ (publish)\nSNS Topic: payment-events\n    ↓ (subscribe)\nCheckout Service (update order)\nEmail Service (notify user)\nAnalytics Service (track revenue)\n```\n\nMessage Flow:\n1. Checkout publishes PaymentRequested {order_id, amount, payment_method}\n2. Payment Service consumes message\n3. Payment Service calls Stripe API (with idempotency key = order_id)\n4. If success: Publish PaymentCompleted event\n5. If failure after retries: Publish PaymentFailed event, send to DLQ\n6. Checkout listens for events, updates order\n\nIdempotency:\n- Use order_id as idempotency key\n- Payment Service checks if payment already processed\n- If duplicate: Return existing result\n\nError Handling:\n- Stripe timeout: Retry 3 times with exponential backoff\n- After 3 retries: Send to DLQ, alert on-call\n- DLQ handler: Manual review and retry\n\nConfidence: 0.90" \
  --confidence 0.90 \
  --tags "integration,pattern,checkout,payment" \
  --json | jq -r '.id')
```

### Step 4: Data Flow

```bash
FLOW=$(engram context create \
  --title "Integration Data Flow: Checkout Payment" \
  --content "## Happy Path\n\n1. User clicks Pay (t=0ms)\n2. Checkout validates cart (t=20ms)\n3. Checkout publishes to SQS (t=30ms)\n4. Checkout returns 202 Accepted {order_id, status: pending} to user (t=40ms)\n5. Payment Service consumes message (t=100ms)\n6. Payment calls Stripe API (t=900ms)\n7. Stripe returns success (t=1000ms)\n8. Payment publishes PaymentCompleted event (t=1020ms)\n9. Checkout receives event, updates order to confirmed (t=1100ms)\n10. Email Service sends confirmation (t=3000ms)\n\nUser sees: Spinner for 40ms, then Success (payment processing)\n\n## Error Path: Stripe Timeout\n\n5. Payment calls Stripe API (timeout after 5s)\n6. Payment retries (wait 2s, call again)\n7. Payment retries (wait 4s, call again)\n8. Payment retries (wait 8s, call again)\n9. All retries failed → Send to DLQ\n10. Payment publishes PaymentPending event\n11. Checkout updates order to pending_payment\n12. Email Service sends Payment Processing email\n13. DLQ handler manually reviews and retries\n\nUser sees: Payment is processing, well send confirmation\n\n## Message Format\n\nPaymentRequested:\n{order_id, user_id, amount, currency, payment_method_id, idempotency_key}\n\nPaymentCompleted:\n{order_id, payment_id, stripe_charge_id, amount, timestamp}\n\nPaymentFailed:\n{order_id, error_type, error_message, retry_count}" \
  --source "integration-patterns" \
  --tags "integration,data-flow,checkout,payment" \
  --json | jq -r '.id')
```

### Step 5: Link Everything

```bash
for ID in $REQ $STYLE $PATTERN $FLOW; do
  engram relationship create \
    --source-id checkout-integration-123 --source-type task \
    --target-id $ID \
    --relationship-type documents --agent default
done
```

## Querying Integration Patterns

```bash
# Get all integration designs
engram relationship connected --entity-id [TASK_ID] | grep -E "Integration"

# Find all async integrations
engram reasoning list | grep "Asynchronous"

# Find all CQRS implementations
engram reasoning list | grep "CQRS"

# Search for specific pattern
engram reasoning list | grep -i "event sourcing"
```

## Related Skills

This skill integrates with:
- `engram-system-design` - Design systems with integration points
- `engram-observability-design` - Monitor integration health with metrics and traces
- `engram-api-design` - Design REST/GraphQL APIs for sync integrations
- `engram-scalability-analysis` - Plan for integration scaling
- `engram-risk-assessment` - Assess integration failure risks
- `engram-dependency-mapping` - Map service dependencies and integration flows
