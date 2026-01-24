---
name: engram-performance-analysis
description: "Profile code execution, identify performance hotspots, analyze algorithmic complexity, set performance budgets and detect regressions."
---

# Performance Analysis (Engram-Integrated)

## Overview

Systematically profile code execution to identify performance bottlenecks, analyze algorithmic complexity, establish performance budgets for critical paths, and implement regression detection. Store profiling results, complexity analyses, and performance baselines in Engram to track optimization efforts and prevent performance degradation.

## When to Use

Use this skill when:
- Application performance degrades over time (slower response, higher latency)
- Planning optimization work and need to identify highest-impact targets
- Setting SLAs or performance requirements for critical operations
- Investigating user complaints about slow features
- Before and after major refactoring to measure impact
- Establishing CI/CD performance gates to catch regressions
- Reviewing code and concerned about algorithmic complexity

## The Pattern

### Step 1: Establish Performance Baseline

Before optimization, capture current performance metrics:

```bash
engram context create \
  --title "Performance Baseline: [System/Feature Name]" \
  --content "## System Under Test\n\n**Component:** [e.g., API endpoint, database query, UI render]\n**Version:** [git commit SHA or version]\n**Environment:** [production, staging, local]\n**Date:** $(date -u +%Y-%m-%dT%H:%M:%SZ)\n\n## Baseline Metrics\n\n### Throughput\n- **Requests per second:** [N] (avg), [N] (p95), [N] (p99)\n- **Concurrent users:** [N] (tested load)\n- **Test duration:** [N minutes/hours]\n\n### Latency\n- **p50 (median):** [N ms]\n- **p95:** [N ms]\n- **p99:** [N ms]\n- **p99.9:** [N ms]\n- **max:** [N ms]\n\n### Resource Utilization\n- **CPU:** [N%] avg, [N%] peak\n- **Memory:** [N MB] avg, [N MB] peak, [N MB] allocated\n- **Disk I/O:** [N MB/s] read, [N MB/s] write\n- **Network:** [N MB/s] in, [N MB/s] out\n\n### Database Metrics\n- **Query count:** [N] queries per request\n- **Query time:** [N ms] total (p50), [N ms] (p95)\n- **Connection pool:** [N] active of [N] max\n- **Slow queries:** [N] queries >[threshold]ms\n\n### Application Metrics\n- **Garbage collection:** [N ms] pause time (p95)\n- **Thread pool:** [N] active threads of [N] max\n- **Cache hit rate:** [N%]\n- **Error rate:** [N%]\n\n## Performance Requirements\n\n**SLA Targets:**\n- p95 latency: <[N ms]\n- p99 latency: <[N ms]\n- Throughput: >[N] req/s\n- Error rate: <[N%]\n\n**Current vs Target:**\n- p95: [current] vs [target] ([over/under by N%])\n- p99: [current] vs [target] ([over/under by N%])\n\n**Assessment:** [Meeting SLA / Violating SLA / Approaching limits]" \
  --source "performance-baseline" \
  --tags "performance,baseline,[component-name]"
```

### Step 2: Profile to Identify Hotspots

Use profiling tools to find performance bottlenecks:

```bash
engram reasoning create \
  --title "Performance Profile: [Component Name]" \
  --task-id [TASK_ID] \
  --content "## Profiling Method\n\n**Tool:** [e.g., perf, py-spy, pprof, Chrome DevTools, Instruments]\n**Duration:** [N seconds/minutes]\n**Workload:** [representative user scenario or benchmark]\n**Sample rate:** [N samples/second]\n\n## CPU Profiling Results\n\n### Top Functions by CPU Time\n\n**1. function_name() - [file:line]**\n- **Self time:** [N ms] ([N%] of total)\n- **Total time:** [N ms] ([N%] of total)\n- **Call count:** [N] calls\n- **Avg time per call:** [N μs]\n- **Why slow:** [explanation]\n\n**2. another_function() - [file:line]**\n- **Self time:** [N ms] ([N%] of total)\n- **Total time:** [N ms] ([N%] of total)\n- **Call count:** [N] calls\n- **Avg time per call:** [N μs]\n- **Why slow:** [explanation]\n\n**3. third_function() - [file:line]**\n- **Self time:** [N ms] ([N%] of total)\n- **Total time:** [N ms] ([N%] of total)\n- **Call count:** [N] calls\n- **Avg time per call:** [N μs]\n- **Why slow:** [explanation]\n\n### Hotspot Analysis\n\n**Critical path:** [function_a() -> function_b() -> function_c()]\n**Total critical path time:** [N ms] ([N%] of request)\n\n**Optimization potential:**\n- Top 3 functions account for [N%] of CPU time\n- Optimizing these could improve p95 by up to [N%]\n\n## Memory Profiling Results\n\n### Allocation Hotspots\n\n**1. Type: [ObjectType]**\n- **Allocated:** [N MB] ([N%] of total)\n- **Count:** [N] objects\n- **Avg size:** [N KB] per object\n- **Allocated in:** [function_name() - file:line]\n- **Lifetime:** [short-lived / long-lived / leaked]\n\n**2. Type: [AnotherType]**\n- **Allocated:** [N MB] ([N%] of total)\n- **Count:** [N] objects\n- **Avg size:** [N KB] per object\n- **Allocated in:** [function_name() - file:line]\n- **Lifetime:** [short-lived / long-lived / leaked]\n\n### Memory Issues\n\n**Potential leaks:**\n- [Object type] count growing unbounded\n- [N MB] not freed after [N] minutes\n\n**High allocation churn:**\n- [N MB/s] allocation rate\n- [N] GC pauses per second\n- [N ms] p95 GC pause time\n\n## I/O Profiling Results\n\n### Database Operations\n\n**Query 1:** [SQL query or description]\n- **Execution count:** [N] per request\n- **Execution time:** [N ms] (p50), [N ms] (p95)\n- **Rows returned:** [N] (avg)\n- **Optimization:** [Missing index / N+1 query / Full table scan]\n\n**Query 2:** [SQL query or description]\n- **Execution count:** [N] per request\n- **Execution time:** [N ms] (p50), [N ms] (p95)\n- **Rows returned:** [N] (avg)\n- **Optimization:** [Missing index / N+1 query / Full table scan]\n\n### Network Calls\n\n**API Call: [Service/Endpoint]**\n- **Call count:** [N] per request\n- **Latency:** [N ms] (p50), [N ms] (p95)\n- **Payload size:** [N KB] request, [N KB] response\n- **Optimization:** [Batch requests / Cache responses / Reduce payload]\n\n## Flamegraph Analysis\n\n**Flamegraph file:** [path/to/flamegraph.svg]\n\n**Key observations:**\n- Wide flames (high CPU time): [function names]\n- Deep stacks (excessive recursion/depth): [call chain]\n- Unexpected hotspots: [surprising findings]\n\n## Recommendations\n\n**Priority 1 (High Impact):**\n1. [Optimization suggestion] - Expected improvement: [N%]\n2. [Another optimization] - Expected improvement: [N%]\n\n**Priority 2 (Medium Impact):**\n3. [Optimization suggestion] - Expected improvement: [N%]\n\n**Priority 3 (Low Impact):**\n4. [Minor optimization] - Expected improvement: [N%]" \
  --confidence 0.85 \
  --tags "performance,profiling,hotspots,[component-name]"
```

### Step 3: Analyze Algorithmic Complexity

Examine algorithm complexity for critical operations:

```bash
engram reasoning create \
  --title "Complexity Analysis: [Function/Algorithm Name]" \
  --task-id [TASK_ID] \
  --content "## Function Signature\n\n**Function:** \`[function_name(args)]\`\n**Location:** [file:line]\n**Purpose:** [what it does]\n\n## Time Complexity Analysis\n\n**Current Algorithm:** [description]\n\n**Complexity:** O([complexity])\n\n**Analysis:**\n\`\`\`\nfor i in range(n):           # O(n)\n    for j in range(n):       # O(n)\n        do_work(i, j)        # O(1)\n# Total: O(n²)\n\`\`\`\n\n**Input size n:** [typical: N, worst-case: M]\n\n**Performance at scale:**\n- n=100: [N] operations (~[N ms])\n- n=1000: [N] operations (~[N ms])\n- n=10000: [N] operations (~[N ms])\n\n**Growth rate:**\n- 10x input size → [Nx] execution time\n\n## Space Complexity Analysis\n\n**Complexity:** O([complexity])\n\n**Memory usage:**\n- n=100: [N MB]\n- n=1000: [N MB]\n- n=10000: [N MB]\n\n**Auxiliary space:** [stack depth, temp arrays, etc.]\n\n## Optimization Opportunities\n\n### Current Approach: O([current_complexity])\n\n**Problems:**\n- [Issue 1: e.g., nested loops over same data]\n- [Issue 2: e.g., repeated computation]\n- [Issue 3: e.g., inefficient data structure]\n\n### Optimized Approach: O([improved_complexity])\n\n**Strategy:** [e.g., use hash map for O(1) lookup]\n\n**Pseudocode:**\n\`\`\`\n[optimized algorithm pseudocode]\n\`\`\`\n\n**Improvement:**\n- Time: O([old]) → O([new])\n- Space: O([old]) → O([new])\n\n**Trade-offs:**\n- **Pros:** [faster execution, better scaling]\n- **Cons:** [more memory, more complex code]\n\n**Expected speedup at n=10000:**\n- Current: [N ms]\n- Optimized: [N ms]\n- Improvement: [Nx faster]\n\n## Recommendation\n\n**Proceed with optimization:** [Yes/No]\n\n**Rationale:**\n[Why optimization is/isn't worth the effort given current usage patterns]\n\n**Implementation effort:** [N hours/days]\n**Expected performance gain:** [N%] for typical workloads" \
  --confidence 0.80 \
  --tags "performance,complexity-analysis,[component-name]"
```

### Step 4: Set Performance Budgets

Establish performance budgets for critical paths:

```bash
engram reasoning create \
  --title "Performance Budget: [Feature/Page/Endpoint]" \
  --task-id [TASK_ID] \
  --content "## Scope\n\n**Feature:** [e.g., Dashboard load, Checkout flow, Search results]\n**User journey:** [step-by-step user interaction]\n**Business criticality:** [High/Medium/Low]\n\n## Performance Budget\n\n### Latency Budget (Time to Interactive)\n\n**Total budget:** [N ms] (target user experience)\n\n**Breakdown:**\n- DNS lookup: [N ms]\n- TCP connection: [N ms]\n- TLS handshake: [N ms]\n- Server processing: [N ms]\n- Response transfer: [N ms]\n- Client rendering: [N ms]\n- JavaScript execution: [N ms]\n- API calls: [N ms]\n\n**Current usage:** [N ms] ([N%] of budget)\n**Remaining:** [N ms] ([N%] of budget)\n\n### Resource Budget\n\n**JavaScript bundle size:**\n- Budget: [N KB] (gzipped)\n- Current: [N KB] ([N%] of budget)\n- Critical path JS: [N KB]\n\n**CSS size:**\n- Budget: [N KB] (gzipped)\n- Current: [N KB] ([N%] of budget)\n\n**Image assets:**\n- Budget: [N KB] per page\n- Current: [N KB] ([N%] of budget)\n\n**Total page weight:**\n- Budget: [N MB]\n- Current: [N MB] ([N%] of budget)\n\n**API requests:**\n- Budget: [N] requests per page load\n- Current: [N] requests ([N%] of budget)\n\n### Rendering Budget\n\n**First Contentful Paint (FCP):**\n- Target: <[N ms]\n- Current: [N ms]\n\n**Largest Contentful Paint (LCP):**\n- Target: <[N ms]\n- Current: [N ms]\n\n**Time to Interactive (TTI):**\n- Target: <[N ms]\n- Current: [N ms]\n\n**Cumulative Layout Shift (CLS):**\n- Target: <[N]\n- Current: [N]\n\n**First Input Delay (FID):**\n- Target: <[N ms]\n- Current: [N ms]\n\n## Budget Violations\n\n**Current violations:**\n1. [Metric] over budget by [N%]: [current] vs [target]\n2. [Another metric] over budget by [N%]: [current] vs [target]\n\n**Action items:**\n1. [Specific action to reduce overbudget metric]\n2. [Another action]\n\n## Budget Enforcement\n\n**CI/CD gates:**\n- Fail build if bundle size exceeds budget by >10%\n- Fail build if Lighthouse score drops below [N]\n- Warn if any Core Web Vital regresses\n\n**Monitoring:**\n- Track budget compliance in production (Real User Monitoring)\n- Alert on budget violations for p75 users\n- Weekly performance budget report\n\n**Review cadence:** Every [N weeks/months]" \
  --confidence 0.80 \
  --tags "performance,budget,[feature-name]"
```

### Step 5: Implement Regression Detection

Set up automated performance testing:

```bash
engram reasoning create \
  --title "Performance Regression Detection: [Component]" \
  --task-id [TASK_ID] \
  --content "## Regression Testing Strategy\n\n### Benchmark Suite\n\n**Benchmark 1: [Operation name]**\n\`\`\`bash\n# Command to run benchmark\n[benchmark command]\n\`\`\`\n\n**Baseline:** [N ms] (±[N%] variance)\n**Threshold:** Fail if >110% of baseline (>[N ms])\n**Frequency:** On every PR\n\n**Benchmark 2: [Another operation]**\n\`\`\`bash\n[benchmark command]\n\`\`\`\n\n**Baseline:** [N ms] (±[N%] variance)\n**Threshold:** Fail if >110% of baseline (>[N ms])\n**Frequency:** On every PR\n\n### Load Testing\n\n**Test scenario:** [description of realistic load]\n**Tool:** [e.g., k6, Apache Bench, wrk, Gatling]\n\n**Test script:**\n\`\`\`javascript\n[load test script]\n\`\`\`\n\n**Pass criteria:**\n- p95 latency <[N ms]\n- p99 latency <[N ms]\n- Error rate <[N%]\n- Throughput >[N] req/s\n\n**Regression threshold:**\n- Fail if p95 increases by >5%\n- Fail if p99 increases by >10%\n- Fail if throughput decreases by >5%\n\n### CI/CD Integration\n\n**Performance test stage:**\n\`\`\`yaml\nperformance-test:\n  script:\n    - run_benchmarks.sh\n    - run_load_tests.sh\n    - compare_with_baseline.sh\n  allow_failure: false\n  only:\n    - merge_requests\n\`\`\`\n\n**Baseline management:**\n- Store baseline metrics in Git (perf_baselines.json)\n- Update baseline on main branch merge\n- Track baseline history over time\n\n**Reporting:**\n- Comment on PR with performance comparison\n- Show flamegraph diff for regressions\n- Link to historical trends\n\n### Continuous Monitoring\n\n**Production metrics:**\n- Real User Monitoring (RUM) for client-side performance\n- Application Performance Monitoring (APM) for server-side\n- Synthetic monitoring for critical user journeys\n\n**Alerting:**\n- Alert if p95 latency increases >20% week-over-week\n- Alert if error rate increases >2x baseline\n- Alert if Core Web Vitals degrade\n\n**Performance dashboard:**\n- URL: [dashboard link]\n- Tracked metrics: [list of key metrics]\n- Review frequency: Daily (automated), Weekly (team review)\n\n## Regression Response Protocol\n\n**When regression detected:**\n\n1. **Identify commit:** Use git bisect to find introducing commit\n2. **Profile:** Run profiler on regression case\n3. **Analyze:** Compare profiles before/after\n4. **Decide:**\n   - Revert if >20% regression with no business justification\n   - Fix forward if <20% or acceptable trade-off\n   - Escalate if unclear\n5. **Document:** Create reasoning entity with analysis\n\n**Escalation criteria:**\n- p95 latency >50% worse than baseline\n- Production user impact (support tickets)\n- SLA violation\n\n## Historical Baseline Tracking\n\n**Baseline updates:**\n- Date: [YYYY-MM-DD]\n- Commit: [SHA]\n- Metric: [N ms]\n- Change: [+/-N%] from previous\n- Reason: [optimization / regression / expected change]" \
  --confidence 0.85 \
  --tags "performance,regression-detection,[component-name]"
```

### Step 6: Link Performance Entities to Task

```bash
# Link performance analysis to task
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [BASELINE_ID] --target-type context \
  --relationship-type references \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [PROFILE_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [COMPLEXITY_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [BUDGET_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [REGRESSION_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]
```

## Example

User reports that dashboard page takes 5+ seconds to load, violating SLA.

### Step 1: Establish Baseline

```bash
# Capture current performance
BASELINE=$(engram context create \
  --title "Performance Baseline: Dashboard Page Load" \
  --content "## System Under Test

**Component:** Dashboard page load
**Version:** commit abc123
**Environment:** production
**Date:** 2026-01-24T10:00:00Z

## Baseline Metrics

### Latency
- **p50 (median):** 2400 ms
- **p95:** 5200 ms (SLA violation - target: 3000 ms)
- **p99:** 7800 ms
- **max:** 12000 ms

### Resource Utilization
- **Server CPU:** 45% avg, 70% peak
- **Server Memory:** 1.2 GB avg, 1.8 GB peak
- **Client CPU:** 60% avg (desktop), 95% peak (mobile)
- **Network transferred:** 3.5 MB total (2.2 MB JS, 800 KB images, 500 KB API)

### Database Metrics
- **Query count:** 28 queries per page load
- **Query time:** 450 ms total (p50), 1200 ms (p95)
- **Slow queries:** 5 queries >100ms

### Application Metrics
- **API requests:** 12 requests per page load
- **Cache hit rate:** 60%
- **First Contentful Paint:** 1800 ms
- **Time to Interactive:** 5500 ms

## Performance Requirements

**SLA Targets:**
- p95 latency: <3000 ms (currently: 5200 ms - VIOLATION)
- p99 latency: <5000 ms (currently: 7800 ms - VIOLATION)

**Current vs Target:**
- p95: 5200 ms vs 3000 ms (73% over target)
- p99: 7800 ms vs 5000 ms (56% over target)

**Assessment:** Violating SLA - requires immediate optimization" \
  --source "performance-baseline" \
  --tags "performance,baseline,dashboard" \
  --json | jq -r '.id')

echo "Baseline captured: $BASELINE"
```

### Step 2: Profile to Find Hotspots

Agent runs profiler on dashboard load:

```bash
# Server-side profiling
perf record -g -F 99 -p $(pgrep -f 'dashboard_server') -- sleep 30
perf script > dashboard_profile.perf
FlameGraph/flamegraph.pl dashboard_profile.perf > dashboard_flame.svg

# Client-side profiling with Chrome DevTools Performance tab
# Manual profiling session captures data

PROFILE=$(engram reasoning create \
  --title "Performance Profile: Dashboard Page Load" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "## Profiling Method

**Server-side tool:** perf (Linux)
**Client-side tool:** Chrome DevTools Performance
**Duration:** 30 seconds (5 page loads)
**Workload:** Full dashboard load with 100 widgets

## CPU Profiling Results

### Server-Side Top Functions

**1. fetch_widget_data() - services/widgets.py:45**
- **Self time:** 800 ms (35% of request time)
- **Total time:** 1200 ms (52% of request time)
- **Call count:** 100 calls (one per widget)
- **Avg time per call:** 12 ms
- **Why slow:** N+1 query problem - fetching widgets individually

**2. serialize_json() - utils/json.py:78**
- **Self time:** 350 ms (15% of request time)
- **Total time:** 350 ms (15% of request time)
- **Call count:** 1 call
- **Avg time per call:** 350 ms
- **Why slow:** Large response payload (2 MB JSON)

**3. check_permissions() - auth/middleware.py:23**
- **Self time:** 180 ms (8% of request time)
- **Total time:** 180 ms (8% of request time)
- **Call count:** 100 calls
- **Avg time per call:** 1.8 ms
- **Why slow:** Database query per widget for permissions

### Client-Side Top Functions

**1. renderWidgets() - dashboard.tsx:120**
- **Self time:** 1400 ms (25% of page load)
- **Total time:** 3200 ms (58% of page load)
- **Call count:** 1
- **Why slow:** Synchronous rendering of 100 widgets blocks main thread

**2. processChartData() - chart-lib.js:456**
- **Self time:** 850 ms (15% of page load)
- **Total time:** 850 ms (15% of page load)
- **Call count:** 30 calls (30 chart widgets)
- **Avg time per call:** 28 ms
- **Why slow:** Heavy data transformation for charts

### Hotspot Analysis

**Server critical path:**
fetch_widget_data() [1200ms] → serialize_json() [350ms]
= 1550ms (68% of server time)

**Client critical path:**
renderWidgets() → processChartData()
= 3200ms (58% of client time)

**Optimization potential:**
- Server: Top 2 functions = 1550ms (68% of 2300ms server time)
- Client: Top 2 functions = 2250ms (41% of 5500ms total time)
- Combined optimization could improve p95 by 40-50%

## Database Profiling Results

### Query Analysis

**Query 1: SELECT * FROM widgets WHERE user_id = ?**
- **Execution count:** 100 (once per widget - N+1 problem)
- **Execution time:** 8 ms per query, 800 ms total
- **Rows returned:** 1 per query
- **Optimization:** Batch into single query with IN clause

**Query 2: SELECT * FROM widget_data WHERE widget_id = ?**
- **Execution count:** 100 (N+1 problem)
- **Execution time:** 4 ms per query, 400 ms total
- **Rows returned:** 1-50 per query (avg: 10)
- **Optimization:** JOIN with widgets query or use data loader pattern

**Query 3: SELECT permissions FROM acl WHERE user_id = ? AND resource_id = ?**
- **Execution count:** 100 (checked per widget)
- **Execution time:** 1.8 ms per query, 180 ms total
- **Optimization:** Load all permissions upfront, check in memory

### Database Time Breakdown
- N+1 widget queries: 800 ms (67% of DB time)
- N+1 widget_data queries: 400 ms (33% of DB time)
- Total DB time: 1200 ms (52% of server time)

## Network Analysis

**API Call Waterfall:**
- Initial HTML: 200 ms
- API /dashboard/widgets: 2300 ms (SLOW - see above)
- 10x API /widgets/:id/details: 150 ms each (parallel) = 150 ms total
- 2x API /user/preferences: 80 ms each (parallel) = 80 ms total

**Sequential dependency chain:**
HTML → /dashboard/widgets → render → /widgets/:id/details
= 200 + 2300 + 500 + 150 = 3150 ms minimum

## Recommendations

**Priority 1 (High Impact):**
1. Fix N+1 queries: batch widget fetch into 1 query - Expected: -1200ms server (-52%)
2. Lazy load widgets: render above-fold first - Expected: -2000ms perceived load (-36%)

**Priority 2 (Medium Impact):**
3. Optimize JSON serialization: stream response - Expected: -150ms server (-7%)
4. Web Worker for chart data processing - Expected: -600ms client (-11%)

**Priority 3 (Low Impact):**
5. Cache permission checks in memory - Expected: -180ms server (-8%)" \
  --confidence 0.90 \
  --tags "performance,profiling,hotspots,dashboard" \
  --json | jq -r '.id')

echo "Profile analysis complete: $PROFILE"
```

### Step 3: Analyze Algorithmic Complexity

Agent examines the N+1 query problem:

```bash
COMPLEXITY=$(engram reasoning create \
  --title "Complexity Analysis: fetch_widget_data() N+1 Problem" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "## Function Signature

**Function:** \`fetch_widget_data(user_id: str) -> List[Widget]\`
**Location:** services/widgets.py:45
**Purpose:** Load all widgets and their data for dashboard display

## Time Complexity Analysis

**Current Algorithm:** Iterative per-widget fetch

**Complexity:** O(n) where n = number of widgets

**Code:**
\`\`\`python
def fetch_widget_data(user_id):
    widgets = []
    widget_ids = get_user_widget_ids(user_id)  # 1 query
    
    for widget_id in widget_ids:               # O(n)
        widget = db.query('SELECT * FROM widgets WHERE id = ?', widget_id)  # 1 query per widget
        data = db.query('SELECT * FROM widget_data WHERE widget_id = ?', widget_id)  # 1 query per widget
        widget['data'] = data
        widgets.append(widget)
    
    return widgets
# Total: 1 + 2n queries
\`\`\`

**Input size n:** typical: 100 widgets, max: 500 widgets

**Performance at scale:**
- n=10: 21 queries (~50 ms)
- n=100: 201 queries (~800 ms)
- n=500: 1001 queries (~4000 ms)

**Growth rate:**
- 5x input size → 5x execution time (linear in queries)
- Each query ~4ms overhead

## Optimization Opportunities

### Current Approach: O(n) queries

**Problems:**
- Database round-trip overhead: 4ms × 200 queries = 800ms
- Cannot use batch optimizations (query plan cache, row prefetching)
- Network latency multiplied by query count
- Connection pool exhaustion under load

### Optimized Approach: O(1) queries

**Strategy:** Batch fetch with JOIN

**Pseudocode:**
\`\`\`python
def fetch_widget_data_optimized(user_id):
    # Single query with JOIN - O(1) queries
    query = '''
        SELECT w.*, wd.*
        FROM widgets w
        JOIN widget_data wd ON wd.widget_id = w.id
        WHERE w.user_id = ?
    '''
    results = db.query(query, user_id)
    
    # Group by widget_id in memory - O(n)
    widgets = group_by_widget_id(results)
    return widgets
# Total: 1 query + O(n) in-memory grouping
\`\`\`

**Improvement:**
- Queries: O(n) → O(1)
- Time: 800ms → 50ms (16x faster)
- Space: O(n) both cases (no change)

**Trade-offs:**
- **Pros:** 
  - 16x faster for n=100
  - Scales to large widget counts
  - Reduced connection pool pressure
  - Single query plan, better caching
- **Cons:** 
  - Slightly more complex grouping logic
  - Must handle NULL values from LEFT JOIN if some widgets have no data
  - Single large result set (vs streaming individual results)

**Expected speedup at n=100:**
- Current: 800 ms (query time)
- Optimized: 50 ms (single query + grouping)
- Improvement: 16x faster

## Recommendation

**Proceed with optimization:** Yes

**Rationale:**
N+1 query is the #1 bottleneck (52% of server time). Optimization is straightforward with well-known pattern (batch + JOIN). 16x speedup would reduce p95 latency from 5200ms to ~3800ms, much closer to SLA target.

**Implementation effort:** 4 hours (refactor, test, deploy)
**Expected performance gain:** 32% reduction in p95 latency (1550ms → 200ms server time)" \
  --confidence 0.90 \
  --tags "performance,complexity-analysis,dashboard" \
  --json | jq -r '.id')

echo "Complexity analysis complete: $COMPLEXITY"
```

### Step 4: Set Performance Budget

Agent establishes budget to prevent regressions:

```bash
BUDGET=$(engram reasoning create \
  --title "Performance Budget: Dashboard Page Load" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "## Scope

**Feature:** Dashboard page load
**User journey:** User logs in → clicks Dashboard → sees 100 widgets with data
**Business criticality:** High (primary application interface)

## Performance Budget

### Latency Budget (Time to Interactive)

**Total budget:** 3000 ms (SLA target)

**Breakdown:**
- Server processing: 500 ms (17%)
- Response transfer: 200 ms (7%)
- Client parsing: 300 ms (10%)
- JavaScript execution: 800 ms (27%)
- Rendering: 1200 ms (40%)

**Current usage:** 5500 ms (183% of budget - OVER)
**After optimization:** ~2800 ms (93% of budget - UNDER)

### Resource Budget

**JavaScript bundle size:**
- Budget: 300 KB (gzipped)
- Current: 450 KB (150% of budget - OVER)
- Action: Code split dashboard charts into separate bundle

**API payload size:**
- Budget: 500 KB (gzipped response)
- Current: 800 KB (160% of budget - OVER)
- Action: Remove redundant fields, compress more aggressively

**Database queries:**
- Budget: 5 queries per page load
- Current: 201 queries (4020% of budget - SEVERE VIOLATION)
- After optimization: 3 queries (60% of budget - UNDER)

**API requests:**
- Budget: 3 requests per page load
- Current: 12 requests (400% of budget - OVER)
- After optimization: 4 requests (133% - still over, next iteration)

### Rendering Budget

**First Contentful Paint (FCP):**
- Target: <1200 ms
- Current: 1800 ms (OVER)
- After optimization: ~1000 ms (UNDER)

**Largest Contentful Paint (LCP):**
- Target: <2500 ms
- Current: 4200 ms (OVER)
- After optimization: ~2200 ms (UNDER)

**Time to Interactive (TTI):**
- Target: <3000 ms
- Current: 5500 ms (OVER)
- After optimization: ~2800 ms (UNDER)

**Cumulative Layout Shift (CLS):**
- Target: <0.1
- Current: 0.25 (OVER - widgets shift during load)
- Action: Reserve space for widgets, load skeleton first

## Budget Violations

**Current violations:**
1. TTI over budget by 83%: 5500ms vs 3000ms target
2. Database queries over budget by 4020%: 201 vs 5 target
3. Bundle size over budget by 50%: 450KB vs 300KB target
4. CLS over budget by 150%: 0.25 vs 0.1 target

**Action items:**
1. Fix N+1 queries (HIGH): Reduce 201 → 3 queries
2. Lazy load widgets (HIGH): Render above-fold first
3. Code split charts (MEDIUM): Move chart lib to separate bundle
4. Reserve widget space (MEDIUM): Prevent layout shift

## Budget Enforcement

**CI/CD gates:**
- Fail build if bundle size exceeds 350 KB (16% grace)
- Fail build if Lighthouse Performance score drops below 75
- Warn if database query count exceeds 10 per page

**Monitoring:**
- Track TTI via Real User Monitoring (weekly review)
- Alert if p95 TTI exceeds 3500ms (SLA + 500ms grace)
- Dashboard: https://grafana.example.com/dashboard-perf

**Review cadence:** Monthly performance budget review" \
  --confidence 0.85 \
  --tags "performance,budget,dashboard" \
  --json | jq -r '.id')

echo "Performance budget established: $BUDGET"
```

### Step 5: Link to Task

```bash
engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $BASELINE --target-type context \
  --relationship-type references --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $PROFILE --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $COMPLEXITY --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $BUDGET --target-type reasoning \
  --relationship-type documents --agent default
```

### Step 6: Communicate Findings

Agent reports to user:

"Performance analysis complete. Dashboard violates SLA (5200ms p95 vs 3000ms target). Root cause: N+1 query problem (201 queries per load). Profiling shows 52% of time in database calls. Optimization: batch queries into single JOIN. Expected improvement: 16x database speedup, reducing p95 from 5200ms → 2800ms (7% under SLA). Implementation: 4 hours. Recommend proceeding immediately. All analysis stored in Engram."

## Querying Performance Analysis

```bash
# Get performance baselines
engram context list | grep "Performance Baseline:"

# Get profiling results
engram reasoning list | grep "Performance Profile:"

# Get complexity analyses
engram reasoning list | grep "Complexity Analysis:"

# Get performance budgets
engram reasoning list | grep "Performance Budget:"

# Get regression detection configs
engram reasoning list | grep "Performance Regression Detection:"

# Get all performance work for a component
engram relationship connected --entity-id [TASK_ID] | grep -i "performance"
```

## Related Skills

This skill integrates with:
- `engram-system-design` - Design systems with performance requirements from the start
- `engram-refactoring-strategy` - Plan performance-focused refactoring work
- `engram-assumption-validation` - Test performance assumptions before implementation
- `engram-scalability-analysis` - Analyze performance at scale
- `engram-code-quality` - Balance performance optimization with code maintainability
