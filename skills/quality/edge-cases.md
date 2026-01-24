---
name: engram-edge-cases
description: "Systematically identify edge cases, error conditions, boundary values, use property-based testing and fuzzing techniques."
---

# Edge Case Identification (Engram-Integrated)

## Overview

Systematically identify edge cases, error conditions, and boundary values that can cause bugs. Use techniques like boundary value analysis, equivalence partitioning, property-based testing, and fuzzing to find cases that traditional testing misses. Store edge case inventories and test strategies in Engram for comprehensive coverage.

## When to Use

Use this skill when:
- Designing test cases for new features
- Investigating production bugs (often edge cases)
- Reviewing code for potential failure modes
- Implementing input validation
- Designing APIs that handle diverse inputs
- Planning security testing (edge cases often exploitable)
- Improving test coverage beyond happy path
- Preventing regressions from known edge cases

## The Pattern

### Step 1: Inventory Inputs and Conditions

Identify all inputs and conditions:

```bash
engram context create \
  --title "Edge Case Inventory: [Feature/Function]" \
  --content "## Function/Feature Overview\n\n**Name:** [Function or feature name]\n**Purpose:** [What it does]\n**Location:** [File path and line number]\n\n## Inputs\n\n### Input 1: [Parameter name]\n\n**Type:** [string, number, array, object, etc.]\n**Valid Range:** [e.g., 0-100, non-empty string, etc.]\n**Required:** [Yes/No]\n**Default:** [Default value if any]\n\n**Equivalence Classes:**\n- Valid: [e.g., 1-99]\n- Boundary: [e.g., 0, 100]\n- Invalid: [e.g., -1, 101, null, undefined, non-numeric]\n\n**Edge Cases:**\n- Empty: [] or \"\" or null or undefined\n- Minimum: [Smallest valid value]\n- Maximum: [Largest valid value]\n- Just below min: [Invalid boundary]\n- Just above max: [Invalid boundary]\n- Wrong type: [e.g., string when number expected]\n- Special values: [e.g., NaN, Infinity, 0, -0]\n\n### Input 2: [Another parameter]\n\n(Same structure as Input 1)\n\n## Conditions\n\n### Condition 1: [State or environment condition]\n\n**What:** [e.g., Database connection state]\n**Values:** [e.g., Connected, Disconnected, Slow]\n**Edge Cases:**\n- Connection lost mid-operation\n- Timeout during query\n- Connection pool exhausted\n\n### Condition 2: [Another condition]\n\n(Same structure)\n\n## Interactions\n\n**Multiple inputs combined:**\n- Input A at boundary + Input B at boundary\n- Input A valid + Input B invalid\n- All inputs at maximum simultaneously\n- Contradictory inputs (e.g., start_date > end_date)\n\n## Output\n\n**Expected output type:** [Type]\n**Edge cases in output:**\n- Empty result\n- Single item vs multiple items\n- Maximum size result\n- Error conditions" \
  --source "edge-cases" \
  --tags "testing,edge-cases,[feature]"
```

### Step 2: Apply Boundary Value Analysis

Identify boundary values:

```bash
engram reasoning create \
  --title "Boundary Value Analysis: [Feature]" \
  --task-id [TASK_ID] \
  --content "## Boundary Value Analysis\n\n**Technique:** Test values at boundaries where behavior changes\n\n### Numeric Boundaries\n\n**Parameter: [name] (valid range: 0-100)**\n\n**Test Values:**\n- Below minimum: -1 (should fail)\n- At minimum: 0 (should pass)\n- Just above minimum: 1 (should pass)\n- Normal value: 50 (should pass)\n- Just below maximum: 99 (should pass)\n- At maximum: 100 (should pass)\n- Above maximum: 101 (should fail)\n\n**Special numeric values:**\n- Zero: 0\n- Negative zero: -0\n- Very large: Number.MAX_SAFE_INTEGER\n- Very small: Number.MIN_SAFE_INTEGER\n- Infinity: Infinity, -Infinity\n- Not a number: NaN\n- Float precision: 0.1 + 0.2 (equals 0.30000000000000004)\n\n### String Boundaries\n\n**Parameter: [name] (valid: 1-255 characters)**\n\n**Test Values:**\n- Empty: \"\" (should fail)\n- Single character: \"a\" (should pass)\n- Exactly at limit: [255 chars] (should pass)\n- Over limit: [256 chars] (should fail)\n\n**Special string cases:**\n- Whitespace only: \"   \"\n- Unicode: \"🔥\" (multi-byte character)\n- SQL injection: \"'; DROP TABLE users; --\"\n- XSS: \"<script>alert('xss')</script>\"\n- Null byte: \"test\\0data\"\n- Control characters: \"\\n\", \"\\t\", \"\\r\"\n- Very long: 10MB string\n\n### Array Boundaries\n\n**Parameter: [name] (array of items)**\n\n**Test Values:**\n- Null: null (should fail or handle gracefully)\n- Undefined: undefined (should fail)\n- Empty array: [] (should pass or fail depending on requirements)\n- Single item: [item] (should pass)\n- Many items: [10000 items] (should pass or paginate)\n- Max size: [At memory limit] (should fail gracefully)\n\n**Special array cases:**\n- Array with null items: [1, null, 3]\n- Array with mixed types: [1, \"two\", {three: 3}]\n- Sparse array: [1, , , 4] (holes in array)\n- Nested arrays: [[1, 2], [3, 4]]\n\n### Object Boundaries\n\n**Parameter: [name] (object with fields)**\n\n**Test Values:**\n- Null: null\n- Empty object: {}\n- Missing required fields: {field1: \"value\"} (missing field2)\n- Extra fields: {field1: \"value\", field2: \"value\", unexpected: \"value\"}\n- Wrong field types: {field1: 123} (when string expected)\n\n**Special object cases:**\n- Circular reference: {a: obj, b: obj} where obj.a = obj\n- Prototype pollution: {\"__proto__\": {admin: true}}\n- Very deep nesting: {a: {b: {c: {...}}}} 100 levels deep\n\n### Date/Time Boundaries\n\n**Parameter: [name] (date)**\n\n**Test Values:**\n- Epoch: 1970-01-01T00:00:00Z\n- Before epoch: 1969-12-31\n- Far future: 2100-01-01\n- Leap day: 2024-02-29\n- Not leap day: 2023-02-29 (invalid)\n- Daylight saving transition: 2026-03-08T02:30:00 (may not exist)\n- Timezone boundaries: 23:59:59 → 00:00:00\n\n### Boolean Boundaries\n\n**Parameter: [name] (boolean)**\n\n**Test Values:**\n- True: true\n- False: false\n- Truthy: 1, \"yes\", [] (may be coerced to true)\n- Falsy: 0, \"\", null, undefined (may be coerced to false)\n- String \"true\": \"true\" (not boolean)\n- String \"false\": \"false\" (not boolean)\n\n## Test Cases Generated\n\n**From boundary analysis:**\n1. Test with value = -1 (expect error)\n2. Test with value = 0 (expect success)\n3. Test with value = 100 (expect success)\n4. Test with value = 101 (expect error)\n5. Test with string = \"\" (expect error)\n6. Test with string = [255 chars] (expect success)\n7. Test with array = [] (expect success or error, define expected)\n8. Test with array = null (expect error)\n9. Test with date = \"2023-02-29\" (expect error, invalid date)\n10. Test with number = NaN (expect error)\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "testing,boundary-analysis,[feature]"
```

### Step 3: Identify Error Conditions

List all error scenarios:

```bash
engram reasoning create \
  --title "Error Condition Analysis: [Feature]" \
  --task-id [TASK_ID] \
  --content "## Error Condition Analysis\n\n**Technique:** Identify all ways the function/feature can fail\n\n### Input Validation Errors\n\n**Error 1: Invalid Input Type**\n- **Scenario:** User passes string when number expected\n- **Example:** `calculateTotal(\"abc\")` when expects `calculateTotal(123)`\n- **Expected Behavior:** Throw TypeError or return validation error\n- **Test:** Pass wrong types for all parameters\n\n**Error 2: Out of Range**\n- **Scenario:** Value outside valid range\n- **Example:** `setAge(-5)` or `setAge(200)`\n- **Expected Behavior:** Throw RangeError or return validation error\n- **Test:** Pass boundary values ± 1\n\n**Error 3: Missing Required Field**\n- **Scenario:** Required parameter not provided\n- **Example:** `createUser({name: \"John\"})` when email also required\n- **Expected Behavior:** Throw Error or return 400 Bad Request\n- **Test:** Omit each required field individually\n\n### Dependency Errors\n\n**Error 4: Database Connection Failed**\n- **Scenario:** Cannot connect to database\n- **Example:** Database is down or network issue\n- **Expected Behavior:** Throw ConnectionError, retry 3 times, then fail\n- **Test:** Mock database to reject connections\n\n**Error 5: External API Timeout**\n- **Scenario:** External API doesn't respond within timeout\n- **Example:** Payment provider takes > 30s to respond\n- **Expected Behavior:** Cancel request, return timeout error\n- **Test:** Mock API with delayed response\n\n**Error 6: External API Returns Error**\n- **Scenario:** External API returns 4xx or 5xx error\n- **Example:** Payment declined (402), rate limited (429)\n- **Expected Behavior:** Handle gracefully, return appropriate error to user\n- **Test:** Mock API to return error responses\n\n### State Errors\n\n**Error 7: Resource Not Found**\n- **Scenario:** Requested resource doesn't exist\n- **Example:** `getUser('nonexistent_id')`\n- **Expected Behavior:** Return 404 Not Found\n- **Test:** Request non-existent IDs\n\n**Error 8: Resource Already Exists**\n- **Scenario:** Trying to create duplicate resource\n- **Example:** `createUser({email: 'existing@example.com'})`\n- **Expected Behavior:** Return 409 Conflict\n- **Test:** Create resource twice with same unique identifier\n\n**Error 9: Invalid State Transition**\n- **Scenario:** Operation not allowed in current state\n- **Example:** `cancelOrder(completedOrder)` when order already completed\n- **Expected Behavior:** Return 400 Bad Request with clear message\n- **Test:** Attempt invalid state transitions\n\n### Concurrency Errors\n\n**Error 10: Race Condition**\n- **Scenario:** Two operations modify same resource simultaneously\n- **Example:** Two requests try to decrement inventory from 1 to 0\n- **Expected Behavior:** One succeeds, other fails (optimistic locking)\n- **Test:** Send concurrent requests\n\n**Error 11: Deadlock**\n- **Scenario:** Two transactions wait for each other's locks\n- **Example:** Transaction A locks row 1, waits for row 2; Transaction B locks row 2, waits for row 1\n- **Expected Behavior:** Database detects and aborts one transaction\n- **Test:** Simulate concurrent conflicting transactions\n\n### Resource Errors\n\n**Error 12: Out of Memory**\n- **Scenario:** Operation requires more memory than available\n- **Example:** Loading 10GB file into memory\n- **Expected Behavior:** Fail gracefully, suggest streaming approach\n- **Test:** Allocate large objects until memory exhausted\n\n**Error 13: Disk Full**\n- **Scenario:** Cannot write because disk is full\n- **Example:** Saving file when no space remaining\n- **Expected Behavior:** Return error, don't leave partial file\n- **Test:** Mock filesystem to return ENOSPC error\n\n**Error 14: Too Many Open Files**\n- **Scenario:** Hit OS limit on open file descriptors\n- **Example:** Opening 1000 database connections\n- **Expected Behavior:** Fail new connections, close idle ones\n- **Test:** Open files until limit reached\n\n### Security Errors\n\n**Error 15: Unauthorized**\n- **Scenario:** User not authenticated\n- **Example:** Accessing API without valid token\n- **Expected Behavior:** Return 401 Unauthorized\n- **Test:** Send requests without authentication\n\n**Error 16: Forbidden**\n- **Scenario:** User authenticated but not authorized\n- **Example:** Regular user trying to access admin endpoint\n- **Expected Behavior:** Return 403 Forbidden\n- **Test:** Send requests with insufficient permissions\n\n**Error 17: CSRF Attack**\n- **Scenario:** Malicious site makes request on user's behalf\n- **Example:** Form submission without CSRF token\n- **Expected Behavior:** Reject request, require valid token\n- **Test:** Submit forms without CSRF token\n\n### Business Logic Errors\n\n**Error 18: Insufficient Balance**\n- **Scenario:** User tries to spend more than available\n- **Example:** Withdraw $1000 when balance is $500\n- **Expected Behavior:** Reject transaction, return clear error\n- **Test:** Attempt operations exceeding limits\n\n**Error 19: Expired Resource**\n- **Scenario:** Resource has expired and is no longer valid\n- **Example:** Using expired coupon code\n- **Expected Behavior:** Return error indicating expiration\n- **Test:** Use resources past expiration date\n\n## Test Cases Generated\n\n**From error analysis:**\n1. Test with wrong type for each parameter (expect TypeError)\n2. Test with out-of-range values (expect RangeError)\n3. Test with missing required fields (expect ValidationError)\n4. Test with database down (expect ConnectionError, retry logic)\n5. Test with API timeout (expect TimeoutError)\n6. Test with non-existent resource (expect 404)\n7. Test with duplicate resource (expect 409)\n8. Test concurrent modifications (expect one to fail)\n9. Test without authentication (expect 401)\n10. Test with insufficient permissions (expect 403)\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "testing,error-conditions,[feature]"
```

### Step 4: Apply Property-Based Testing

Define properties that should always hold:

```bash
engram context create \
  --title "Property-Based Testing: [Feature]" \
  --content "# Property-Based Testing Strategy\n\n## What is Property-Based Testing?\n\nInstead of testing specific examples, test properties that should hold for ALL inputs.\n\n**Example-based test:**\n```javascript\ntest('reverse of reverse equals original', () => {\n  expect(reverse(reverse([1, 2, 3]))).toEqual([1, 2, 3]);\n});\n```\n\n**Property-based test:**\n```javascript\ntest('reverse of reverse equals original for ANY array', () => {\n  fc.assert(\n    fc.property(fc.array(fc.anything()), (arr) => {\n      expect(reverse(reverse(arr))).toEqual(arr);\n    })\n  );\n  // Tests with random arrays: [], [null], [1, \"a\", {}], etc.\n});\n```\n\n## Properties for [Feature]\n\n### Property 1: Idempotency\n\n**Property:** Applying operation twice gives same result as applying once\n\n**Formula:** `f(f(x)) = f(x)`\n\n**Example:** Deduplicating array\n```javascript\nfc.assert(\n  fc.property(fc.array(fc.integer()), (arr) => {\n    const deduped = deduplicate(arr);\n    const dedupedTwice = deduplicate(deduped);\n    expect(dedupedTwice).toEqual(deduped);\n  })\n);\n```\n\n**Why this matters:** Deduplication should be stable - applying it again shouldn't change result\n\n### Property 2: Reversibility (Inverse)\n\n**Property:** Operation can be reversed to get original input\n\n**Formula:** `f(g(x)) = x` where g is inverse of f\n\n**Example:** Encode/decode\n```javascript\nfc.assert(\n  fc.property(fc.string(), (str) => {\n    const encoded = encode(str);\n    const decoded = decode(encoded);\n    expect(decoded).toEqual(str);\n  })\n);\n```\n\n**Why this matters:** Encoding must be lossless\n\n### Property 3: Preservation (Invariant)\n\n**Property:** Some property is preserved through operation\n\n**Formula:** `property(x) = property(f(x))`\n\n**Example:** Sorting preserves length\n```javascript\nfc.assert(\n  fc.property(fc.array(fc.integer()), (arr) => {\n    const sorted = sort(arr);\n    expect(sorted.length).toEqual(arr.length);\n  })\n);\n```\n\n**Why this matters:** Sorting shouldn't lose or add elements\n\n### Property 4: Commutativity\n\n**Property:** Order of operations doesn't matter\n\n**Formula:** `f(x, y) = f(y, x)`\n\n**Example:** Addition\n```javascript\nfc.assert(\n  fc.property(fc.integer(), fc.integer(), (a, b) => {\n    expect(add(a, b)).toEqual(add(b, a));\n  })\n);\n```\n\n**Why this matters:** Addition should work in any order\n\n### Property 5: Associativity\n\n**Property:** Grouping doesn't matter\n\n**Formula:** `f(f(x, y), z) = f(x, f(y, z))`\n\n**Example:** String concatenation\n```javascript\nfc.assert(\n  fc.property(fc.string(), fc.string(), fc.string(), (a, b, c) => {\n    expect(concat(concat(a, b), c)).toEqual(concat(a, concat(b, c)));\n  })\n);\n```\n\n### Property 6: Identity\n\n**Property:** There exists an identity element that doesn't change result\n\n**Formula:** `f(x, identity) = x`\n\n**Example:** Multiplication by 1\n```javascript\nfc.assert(\n  fc.property(fc.integer(), (n) => {\n    expect(multiply(n, 1)).toEqual(n);\n  })\n);\n```\n\n### Property 7: Ordering (Partial Order)\n\n**Property:** Output maintains ordering relationship\n\n**Formula:** If `x < y` then `f(x) < f(y)`\n\n**Example:** Sorting produces ordered output\n```javascript\nfc.assert(\n  fc.property(fc.array(fc.integer()), (arr) => {\n    const sorted = sort(arr);\n    for (let i = 0; i < sorted.length - 1; i++) {\n      expect(sorted[i]).toBeLessThanOrEqual(sorted[i + 1]);\n    }\n  })\n);\n```\n\n### Property 8: Equivalence to Known Good Implementation\n\n**Property:** Output matches reference implementation\n\n**Formula:** `f(x) = reference(x)`\n\n**Example:** Custom sort vs Array.sort\n```javascript\nfc.assert(\n  fc.property(fc.array(fc.integer()), (arr) => {\n    const ourSort = customSort([...arr]);\n    const refSort = [...arr].sort((a, b) => a - b);\n    expect(ourSort).toEqual(refSort);\n  })\n);\n```\n\n## Property-Based Test Implementation\n\n**Using fast-check (JavaScript):**\n\n```javascript\nconst fc = require('fast-check');\n\ndescribe('User validation', () => {\n  test('valid email property', () => {\n    fc.assert(\n      fc.property(\n        fc.emailAddress(),  // Generate random valid emails\n        (email) => {\n          const result = validateEmail(email);\n          expect(result.valid).toBe(true);\n        }\n      )\n    );\n  });\n\n  test('invalid email property', () => {\n    fc.assert(\n      fc.property(\n        fc.string(),  // Generate random strings (mostly invalid emails)\n        fc.pre((str) => !str.includes('@')),  // Filter: no @ symbol\n        (str) => {\n          const result = validateEmail(str);\n          expect(result.valid).toBe(false);\n        }\n      )\n    );\n  });\n\n  test('round-trip property: serialize then deserialize', () => {\n    fc.assert(\n      fc.property(\n        fc.record({\n          id: fc.uuid(),\n          name: fc.string(),\n          age: fc.integer({ min: 0, max: 120 }),\n        }),\n        (user) => {\n          const serialized = JSON.stringify(user);\n          const deserialized = JSON.parse(serialized);\n          expect(deserialized).toEqual(user);\n        }\n      )\n    );\n  });\n});\n```\n\n**Using Hypothesis (Python):**\n\n```python\nfrom hypothesis import given, strategies as st\nimport pytest\n\n@given(st.integers(), st.integers())\ndef test_addition_commutative(a, b):\n    assert add(a, b) == add(b, a)\n\n@given(st.lists(st.integers()))\ndef test_sort_preserves_length(arr):\n    sorted_arr = sort(arr)\n    assert len(sorted_arr) == len(arr)\n\n@given(st.text())\ndef test_encode_decode_roundtrip(text):\n    encoded = encode(text)\n    decoded = decode(encoded)\n    assert decoded == text\n```\n\n## Advantages\n\n- **Finds edge cases automatically**: Framework generates many test cases\n- **Shrinks failures**: When test fails, framework finds minimal failing case\n- **Documents invariants**: Properties describe what function MUST do\n- **Confidence**: Tests thousands of cases instead of handful\n\n## When to Use\n\n- **Parsers**: Input/output transformations with properties\n- **Encoders**: Must be reversible (encode/decode)\n- **Math functions**: Many mathematical properties\n- **Data structures**: Invariants must hold (sorted stays sorted)\n- **APIs**: Request/response contracts\n\n## Limitations\n\n- **Hard to express some properties**: Not all behavior has clear properties\n- **Slower than unit tests**: Runs many iterations\n- **Learning curve**: Takes time to think in properties\n- **May need custom generators**: For complex domain objects" \
  --source "edge-cases" \
  --tags "testing,property-based-testing,[feature]"
```

### Step 5: Apply Fuzzing

Define fuzzing strategy:

```bash
engram context create \
  --title "Fuzzing Strategy: [Feature]" \
  --content "# Fuzzing Strategy\n\n## What is Fuzzing?\n\nFuzzing (fuzz testing) is automated testing with random, malformed, or unexpected inputs to find crashes, hangs, or security vulnerabilities.\n\n## Types of Fuzzing\n\n### 1. Random Fuzzing (Dumb Fuzzing)\n\n**Approach:** Generate completely random inputs\n\n**Example:**\n```javascript\n// Generate random strings\nfor (let i = 0; i < 10000; i++) {\n  const input = randomString(1000);\n  try {\n    parseInput(input);\n  } catch (error) {\n    if (error instanceof CrashError) {\n      console.log(`Crash found with input: ${input}`);\n    }\n  }\n}\n```\n\n**Pros:** Simple, no setup needed\n**Cons:** Low code coverage, misses structured inputs\n\n### 2. Mutation-Based Fuzzing\n\n**Approach:** Start with valid inputs, mutate them\n\n**Example:**\n```javascript\n// Start with valid JSON\nconst validInput = '{\"name\": \"John\", \"age\": 30}';\n\nconst mutations = [\n  mutate_by_bit_flip(validInput),\n  mutate_by_byte_insert(validInput),\n  mutate_by_byte_delete(validInput),\n  mutate_by_byte_swap(validInput),\n];\n\nmutations.forEach(input => {\n  try {\n    parseJSON(input);\n  } catch (error) {\n    // Expected to throw, but shouldn't crash\n    assert(!(error instanceof CrashError));\n  }\n});\n```\n\n**Pros:** Finds issues near valid inputs\n**Cons:** Still requires valid starting corpus\n\n### 3. Generation-Based Fuzzing (Grammar-Based)\n\n**Approach:** Use grammar to generate structured inputs\n\n**Example:**\n```javascript\n// Define grammar for SQL\nconst sqlGrammar = {\n  query: ['SELECT <columns> FROM <table> WHERE <condition>'],\n  columns: ['*', '<column>', '<column>, <column>'],\n  column: ['id', 'name', 'email'],\n  table: ['users', 'orders', 'products'],\n  condition: ['<column> = <value>', '<column> > <value>'],\n  value: ['1', \"'test'\", 'NULL'],\n};\n\n// Generate 1000 SQL queries\nfor (let i = 0; i < 1000; i++) {\n  const query = generate(sqlGrammar, 'query');\n  try {\n    executeSQL(query);\n  } catch (error) {\n    checkForSecurityVulnerability(error, query);\n  }\n}\n```\n\n**Pros:** High code coverage, structured inputs\n**Cons:** Requires defining grammar\n\n### 4. Coverage-Guided Fuzzing (Smart Fuzzing)\n\n**Approach:** Use code coverage to guide input generation\n\n**How it works:**\n1. Run function with input\n2. Measure code coverage\n3. If new code path discovered, keep input for mutation\n4. Mutate inputs that found new paths\n5. Repeat\n\n**Tools:**\n- **AFL (American Fuzzy Lop)**: C/C++ fuzzing\n- **libFuzzer**: LLVM fuzzing\n- **jazzer**: Java fuzzing\n- **atheris**: Python fuzzing\n\n**Example (using libFuzzer):**\n```cpp\nextern \"C\" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {\n  std::string input(reinterpret_cast<const char*>(data), size);\n  parseInput(input);  // Test function\n  return 0;\n}\n\n// Compile: clang++ -fsanitize=fuzzer,address fuzz.cpp\n// Run: ./a.out\n// Automatically generates inputs, discovers new code paths\n```\n\n**Pros:** Very effective, finds deep bugs\n**Cons:** Requires instrumentation, slower\n\n## Fuzzing Targets for [Feature]\n\n### Target 1: Input Parser\n\n**What to fuzz:** JSON/XML/CSV parser\n\n**Strategy:** Mutation-based + grammar-based\n\n**Test cases:**\n- Malformed JSON: `{\"key\": ]`\n- Deeply nested: 10000 levels deep\n- Large input: 100MB JSON\n- Unicode edge cases: `{\"key\": \"\\uD800\"}` (unpaired surrogate)\n- Repeated keys: `{\"key\": 1, \"key\": 2}`\n\n**Expected behavior:**\n- ✓ Return error (don't crash)\n- ✓ Handle gracefully (don't hang)\n- ✓ No memory leaks\n\n### Target 2: API Endpoint\n\n**What to fuzz:** HTTP request handler\n\n**Strategy:** Generation-based (HTTP grammar)\n\n**Test cases:**\n- Invalid methods: `HACK /api/users`\n- Malformed headers: `Content-Length: abc`\n- Huge headers: 10MB of headers\n- SQL injection: `GET /api/users?id=1' OR '1'='1`\n- XSS: `GET /api/search?q=<script>alert(1)</script>`\n- Path traversal: `GET /api/files?path=../../etc/passwd`\n- Buffer overflow: Very long URL (> 10KB)\n\n**Expected behavior:**\n- ✓ Return 400 Bad Request (don't crash)\n- ✓ No SQL injection\n- ✓ No XSS in error messages\n- ✓ No path traversal\n\n### Target 3: File Upload\n\n**What to fuzz:** File upload handler\n\n**Strategy:** Mutation-based (mutate valid files)\n\n**Test cases:**\n- Empty file: 0 bytes\n- Huge file: 10GB\n- Wrong MIME type: image.png with PDF content\n- Malformed image: Corrupted PNG header\n- Zip bomb: Small zip that extracts to huge size\n- Path traversal in filename: `../../etc/passwd`\n- Long filename: 10000 characters\n\n**Expected behavior:**\n- ✓ Reject files outside size limits\n- ✓ Validate MIME type\n- ✓ Handle corrupted files gracefully\n- ✓ Prevent zip bombs\n- ✓ Sanitize filenames\n\n## Fuzzing Implementation\n\n**JavaScript (using jazzer.js):**\n\n```javascript\nconst { FuzzedDataProvider } = require('@jazzer.js/core');\n\nmodule.exports.fuzz = function(data) {\n  const provider = new FuzzedDataProvider(data);\n  \n  // Generate random inputs\n  const str = provider.consumeString(1000);\n  const num = provider.consumeIntegral(4);\n  const bool = provider.consumeBoolean();\n  \n  // Test function\n  try {\n    myFunction(str, num, bool);\n  } catch (error) {\n    // Errors are ok, crashes are not\n    if (error instanceof FatalError) {\n      throw error;  // Report crash\n    }\n  }\n};\n```\n\n**Python (using atheris):**\n\n```python\nimport atheris\nimport sys\n\ndef fuzz(data):\n    fdp = atheris.FuzzedDataProvider(data)\n    \n    # Generate random inputs\n    string_input = fdp.ConsumeUnicodeNoSurrogates(100)\n    int_input = fdp.ConsumeInt(4)\n    \n    # Test function\n    try:\n        my_function(string_input, int_input)\n    except ValueError:\n        pass  # Expected error\n    except Exception as e:\n        # Unexpected error - may be a bug\n        raise\n\nif __name__ == \"__main__\":\n    atheris.Setup(sys.argv, fuzz)\n    atheris.Fuzz()\n```\n\n## Continuous Fuzzing\n\n**OSS-Fuzz (for open source projects):**\n- Continuous fuzzing service by Google\n- Runs 24/7 on cloud infrastructure\n- Reports bugs privately\n- Free for open source\n\n**Setup:**\n1. Create `projects/[project]/` directory in OSS-Fuzz repo\n2. Add Dockerfile\n3. Add build script\n4. Add fuzz targets\n5. Submit PR\n\n**Integration with CI/CD:**\n\n```yaml\n# .github/workflows/fuzz.yml\nname: Fuzz Testing\non:\n  schedule:\n    - cron: '0 2 * * *'  # Run nightly\n\njobs:\n  fuzz:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v2\n      - name: Run fuzzer\n        run: |\n          ./fuzz_targets/run_all.sh --duration=3600  # 1 hour\n      - name: Upload crashes\n        if: failure()\n        uses: actions/upload-artifact@v2\n        with:\n          name: crash-inputs\n          path: crashes/\n```\n\n## Analyzing Fuzz Results\n\n**When fuzzer finds crash:**\n\n1. **Minimize input**: Use fuzzer's minimization to find smallest crashing input\n   ```bash\n   # AFL minimize\n   afl-tmin -i crash_input -o minimized_input -- ./target\n   ```\n\n2. **Reproduce**: Verify crash is reproducible\n   ```bash\n   ./target < minimized_input\n   ```\n\n3. **Debug**: Run under debugger to find root cause\n   ```bash\n   gdb ./target\n   (gdb) run < minimized_input\n   (gdb) backtrace\n   ```\n\n4. **Fix**: Patch the bug\n\n5. **Regression test**: Add minimized input to test suite\n\n6. **Continue fuzzing**: Ensure fix doesn't introduce new bugs\n\n## Metrics\n\n**Track fuzzing effectiveness:**\n- **Executions per second**: Higher is better (> 1000 execs/sec)\n- **Code coverage**: Percentage of code reached (aim for > 80%)\n- **Unique crashes**: Number of distinct bugs found\n- **Time to first crash**: How quickly fuzzer finds bugs\n\n## Best Practices\n\n- **Start early**: Fuzz during development, not just before release\n- **Fuzz continuously**: Integrate with CI/CD for ongoing testing\n- **Prioritize targets**: Focus on parsers, network code, file handlers\n- **Use sanitizers**: Run with AddressSanitizer, UBSan to catch memory bugs\n- **Corpus management**: Build good corpus of valid and edge-case inputs\n- **Fix crashes promptly**: Don't let crash backlog grow" \
  --source "edge-cases" \
  --tags "testing,fuzzing,[feature]"
```

### Step 6: Link Edge Case Analysis to Feature

```bash
# Link all edge case analyses to feature task
engram relationship create \
  --source-id [FEATURE_TASK_ID] --source-type task \
  --target-id [INVENTORY_ID] --target-type context \
  --relationship-type documents --agent default

engram relationship create \
  --source-id [FEATURE_TASK_ID] --source-type task \
  --target-id [BOUNDARY_ANALYSIS_ID] --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id [FEATURE_TASK_ID] --source-type task \
  --target-id [ERROR_ANALYSIS_ID] --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id [FEATURE_TASK_ID] --source-type task \
  --target-id [PROPERTY_TESTING_ID] --target-type context \
  --relationship-type documents --agent default

engram relationship create \
  --source-id [FEATURE_TASK_ID] --source-type task \
  --target-id [FUZZING_ID] --target-type context \
  --relationship-type documents --agent default
```

## Example

Identify edge cases for user input validation function.

### Step 1: Inventory

```bash
INVENTORY=$(engram context create \
  --title "Edge Case Inventory: validateEmail()" \
  --content "Function: validateEmail(email: string): boolean\nPurpose: Validate email format\n\nInput: email (string)\nValid: username@domain.tld\nInvalid: anything else\n\nEdge Cases:\n- Empty: \"\"\n- Null: null\n- Undefined: undefined\n- No @: \"userexample.com\"\n- Multiple @: \"user@@example.com\"\n- No domain: \"user@\"\n- No TLD: \"user@example\"\n- Special chars: \"user+tag@example.com\" (valid!)\n- Unicode: \"用户@例え.jp\" (valid!)\n- Very long: 320 characters (max valid length)\n- Whitespace: \" user@example.com \"\n- Case: \"USER@EXAMPLE.COM\" (should work)\n- IP address: \"user@192.168.1.1\" (valid!)" \
  --source "edge-cases" \
  --tags "testing,edge-cases,validation" \
  --json | jq -r '.id')
```

### Step 2: Boundary Analysis

```bash
BOUNDARY=$(engram reasoning create \
  --title "Boundary Value Analysis: validateEmail()" \
  --task-id email-validation-123 \
  --content "Test Values:\n- Empty: \"\" (fail)\n- Single char: \"a\" (fail)\n- Minimum valid: \"a@b.c\" (pass)\n- Maximum length: 320 chars (pass)\n- Over max: 321 chars (fail)\n- No @: \"user.example.com\" (fail)\n- Multiple @: \"user@@example.com\" (fail)\n- Special: \"user+tag@example.com\" (pass)\n- Unicode: \"用户@例え.jp\" (pass or fail, define requirement)" \
  --confidence 0.90 \
  --tags "testing,boundary-analysis,validation" \
  --json | jq -r '.id')
```

### Step 3: Error Conditions

```bash
ERRORS=$(engram reasoning create \
  --title "Error Condition Analysis: validateEmail()" \
  --task-id email-validation-123 \
  --content "Errors:\n1. TypeError: email is not string (pass number, object, array)\n2. Empty string: \"\" (return false)\n3. Malformed: Missing @ (return false)\n4. Malformed: Missing domain (return false)\n5. SQL injection attempt: \"'; DROP TABLE users; --\" (return false, don't crash)" \
  --confidence 0.90 \
  --tags "testing,error-conditions,validation" \
  --json | jq -r '.id')
```

### Step 4: Property-Based Testing

```bash
PROPERTY=$(engram context create \
  --title "Property-Based Testing: validateEmail()" \
  --content "Properties:\n1. All valid emails from fc.emailAddress() return true\n2. Strings without @ return false\n3. Adding invalid chars makes valid email invalid\n\nTest:\nfc.assert(\n  fc.property(fc.emailAddress(), (email) => {\n    expect(validateEmail(email)).toBe(true);\n  })\n);" \
  --source "edge-cases" \
  --tags "testing,property-based-testing,validation" \
  --json | jq -r '.id')
```

### Step 5: Fuzzing

```bash
FUZZING=$(engram context create \
  --title "Fuzzing Strategy: validateEmail()" \
  --content "Strategy: Random string generation\n\nTest:\nfor (let i = 0; i < 10000; i++) {\n  const input = randomString(100);\n  try {\n    validateEmail(input);\n  } catch (error) {\n    // Should never crash, only return true/false\n    fail('Crashed on input: ' + input);\n  }\n}\n\nExpected: No crashes, always returns boolean" \
  --source "edge-cases" \
  --tags "testing,fuzzing,validation" \
  --json | jq -r '.id')
```

## Querying Edge Case Analysis

```bash
# Get all edge case inventories
engram context list | grep "Edge Case Inventory"

# Get boundary analyses
engram reasoning list | grep "Boundary Value Analysis"

# Get error condition analyses
engram reasoning list | grep "Error Condition Analysis"

# Find all property-based testing strategies
engram context list | grep "Property-Based Testing"

# Find fuzzing strategies
engram context list | grep "Fuzzing Strategy"
```

## Related Skills

This skill integrates with:
- `engram-test-strategy` - Integrate edge case tests into overall test plan
- `engram-security-testing` - Many edge cases are security vulnerabilities
- `engram-code-review` - Review for edge case handling
- `engram-input-validation` - Design validation based on edge cases
- `engram-error-handling` - Handle edge case errors gracefully
- `engram-performance-testing` - Edge cases often reveal performance issues
