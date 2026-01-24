---
name: engram-code-quality
description: "Review code for maintainability, readability, test coverage, adherence to style guides, naming conventions, and complexity metrics."
---

# Code Quality Review (Engram-Integrated)

## Overview

Systematically review code for maintainability, readability, test coverage, adherence to style guides and naming conventions, and measure complexity metrics. Identify code smells, technical debt, and improvement opportunities. Store quality assessments, refactoring recommendations, and technical debt tracking in Engram to maintain high code standards over time.

## When to Use

Use this skill when:
- Conducting code reviews before merging pull requests
- Planning technical debt reduction sprints
- Onboarding new team members (establish code standards)
- Evaluating codebase health during project kickoff
- After rapid prototyping phase (cleaning up quick-and-dirty code)
- Preparing for major refactoring work
- Establishing coding standards for new project

## The Pattern

### Step 1: Automated Code Quality Analysis

Run automated tools to assess code quality metrics:

```bash
engram context create \
  --title "Code Quality Report: [Component/Module]" \
  --content "## Analysis Scope

**Component:** [e.g., user authentication module, dashboard frontend]
**Files analyzed:** [N] files, [N] lines of code
**Language:** [Python, TypeScript, Java, etc.]
**Version:** [git SHA]
**Analysis Date:** $(date -u +%Y-%m-%dT%H:%M:%SZ)

## Code Quality Tools

**Linting:**
- Tool: [e.g., ESLint, Pylint, RuboCop, Clippy]
- Config: [eslintrc.json, .pylintrc]
- Command: \`[lint command]\`

**Code formatting:**
- Tool: [e.g., Prettier, Black, gofmt]
- Config: [.prettierrc, pyproject.toml]
- Status: [Formatted / N violations]

**Complexity analysis:**
- Tool: [e.g., Radon, SonarQube, Code Climate]
- Metrics: Cyclomatic complexity, cognitive complexity, maintainability index

**Test coverage:**
- Tool: [e.g., Jest, pytest-cov, JaCoCo]
- Command: \`[coverage command]\`

**Type checking (if applicable):**
- Tool: [TypeScript, mypy, Flow]
- Strictness: [strict / normal / loose]

## Linting Results

**Total issues:** [N]
- Errors: [N] (must fix)
- Warnings: [N] (should fix)
- Info: [N] (consider fixing)

### Error-level Issues

**1. [Rule name] - [file:line]**
- **Severity:** Error
- **Rule:** [e.g., no-unused-vars, undefined-variable]
- **Code:**
  \`\`\`[language]
  [problematic code]
  \`\`\`
- **Issue:** [What's wrong]
- **Fix:** [How to fix]

**2. [Another error]**
- [Same structure]

### Warning-level Issues

**Most common warnings:**
- [Rule name]: [N] occurrences
- [Another rule]: [N] occurrences
- [Third rule]: [N] occurrences

**Sample warning:**
**[Rule name] - [file:line]**
- **Issue:** [Description]
- **Fix:** [Recommendation]

## Code Formatting

**Formatting violations:** [N]
- Indentation: [N]
- Line length (>80 chars): [N]
- Whitespace: [N]
- Import ordering: [N]

**Recommendation:** Run \`[formatter command]\` to auto-fix

## Complexity Metrics

### Cyclomatic Complexity

**Thresholds:**
- 1-10: Simple, low risk
- 11-20: Moderate complexity
- 21-50: High complexity, difficult to test
- 51+: Very high complexity, unmaintainable

**Functions by complexity:**
- Simple (1-10): [N] functions ([N%])
- Moderate (11-20): [N] functions ([N%])
- High (21-50): [N] functions ([N%])
- Very high (51+): [N] functions ([N%])

**Most complex functions:**

**1. process_order() - orders/processor.py:145**
- **Complexity:** 47
- **Lines:** 380
- **Risk:** High - difficult to test and maintain
- **Recommendation:** Break into smaller functions

**2. render_dashboard() - ui/dashboard.tsx:89**
- **Complexity:** 38
- **Lines:** 520
- **Risk:** High
- **Recommendation:** Extract widget rendering into separate components

**3. [Third most complex]**
- [Same structure]

### Cognitive Complexity

**Average cognitive complexity:** [N]
**Functions above threshold (>15):** [N]

**Most cognitively complex:**
- [Function name]: [N] (due to nested conditionals and loops)

### Maintainability Index

**Score:** [0-100] (100 = most maintainable)
- 0-25: Difficult to maintain
- 26-50: Moderate maintainability
- 51-75: Good maintainability
- 76-100: Excellent maintainability

**Current score:** [N] - [Assessment]

**Files with low maintainability (<50):**
- [file]: [score]
- [another file]: [score]

## Test Coverage

**Overall coverage:** [N%]

**Coverage by type:**
- Line coverage: [N%]
- Branch coverage: [N%]
- Function coverage: [N%]

**Coverage target:** [N%] (typically 80-90%)

**Uncovered code:**

**Critical paths with low coverage (<50%):**
- [module/file]: [N%] coverage
  - [function_name]: 0% - no tests
  - [another_function]: 25% - partial tests
- [another module]: [N%] coverage

**Files with 0% coverage:**
- [file]
- [another file]
- [third file]

**Recommendation:** Prioritize testing for [critical module with low coverage]

## Type Safety (if applicable)

**Type checking:** [Pass/Fail]
**Type errors:** [N]

**Common type issues:**
- [Issue type]: [N] occurrences (e.g., implicit any in TypeScript)
- [Another issue]: [N] occurrences

**Type coverage:** [N%] of code has explicit types

## Code Duplication

**Tool:** [e.g., jscpd, Simian, PMD CPD]

**Duplication found:**
- [N] duplicated blocks
- [N] lines of duplicated code ([N%] of total)

**Largest duplicated blocks:**
- [file1:line] and [file2:line]: [N] lines duplicated
- [file3:line] and [file4:line]: [N] lines duplicated

**Recommendation:** Extract duplicated code into shared functions/components

## Dependency Analysis

**Total dependencies:** [N]
- Direct: [N]
- Transitive: [N]

**Outdated dependencies:** [N]
- [package]: [current version] → [latest version]
- [another package]: [current version] → [latest version]

**Unused dependencies:** [N]
- [package]
- [another package]

**Large dependencies (>500 KB):**
- [package]: [size]
- Impact on bundle size

**Recommendation:** Audit and remove unused dependencies

## Code Smells Detected

**God classes/modules:**
- [class/module]: [N] lines, [N] methods/functions
- Responsibility: [too many responsibilities listed]

**Long methods:**
- [method]: [N] lines (threshold: 50)
- [another method]: [N] lines

**Long parameter lists:**
- [function]: [N] parameters (threshold: 5)
- [another function]: [N] parameters

**Feature envy:**
- [method in ClassA]: Accesses ClassB members [N] times
- Recommendation: Move method to ClassB

**Dead code:**
- [N] unused functions
- [N] unused variables
- [N] unused imports

## Summary

**Overall code quality:** [Excellent / Good / Fair / Poor]

**Strengths:**
- [What the code does well]
- [Another strength]

**Weaknesses:**
- [Area needing improvement]
- [Another weakness]

**Critical issues:** [N]
**Total technical debt:** [N hours estimated to fix all issues]

**Priority improvements:**
1. [Highest priority improvement]
2. [Second priority]
3. [Third priority]

**Reports:**
- Linting report: [path/to/lint-report.txt]
- Coverage report: [path/to/coverage/index.html]
- Complexity report: [path/to/complexity.json]" \
  --source "code-quality-report" \
  --tags "code-quality,analysis,[component-name]"
```

### Step 2: Manual Code Review

Conduct manual review for aspects automation cannot catch:

```bash
engram reasoning create \
  --title "Manual Code Review: [Component/PR]" \
  --task-id [TASK_ID] \
  --content "## Review Scope

**Component/PR:** [Name or PR number]
**Author:** [Developer name]
**Files changed:** [N] files, [+N/-N] lines
**Reviewer:** [Reviewer name/agent]
**Review Date:** $(date -u +%Y-%m-%dT%H:%M:%SZ)

## Code Review Checklist

### Correctness

- [ ] Code implements requirements correctly
- [ ] Edge cases handled (null, empty, boundary values)
- [ ] Error handling appropriate and complete
- [ ] No obvious bugs or logic errors
- [ ] Concurrency issues considered (if applicable)

**Issues found:** [N]

**Issue 1: Off-by-one Error in Pagination**
- **Location:** api/pagination.py:67
- **Code:**
  \`\`\`python
  def get_page(items, page, page_size):
      start = page * page_size
      end = start + page_size
      return items[start:end]
  \`\`\`
- **Issue:** First page is page 0, inconsistent with UI (1-indexed)
- **Impact:** Users see page 1, but backend returns page 0 results
- **Fix:**
  \`\`\`python
  def get_page(items, page, page_size):
      # Page is 1-indexed
      start = (page - 1) * page_size
      end = start + page_size
      return items[start:end]
  \`\`\`

### Readability

- [ ] Variable names descriptive and meaningful
- [ ] Function names describe what they do (verb for actions)
- [ ] Class names are nouns describing what they represent
- [ ] Magic numbers extracted into named constants
- [ ] Complex logic has explanatory comments
- [ ] No commented-out code

**Issues found:** [N]

**Issue 2: Poor Variable Naming**
- **Location:** utils/calculator.py:23
- **Code:**
  \`\`\`python
  def calc(a, b, c):
      x = a * b
      y = x + c
      z = y * 0.1
      return z
  \`\`\`
- **Issue:** Single-letter names don't explain purpose
- **Fix:**
  \`\`\`python
  def calculate_tax(subtotal, quantity, shipping_cost):
      total_before_tax = subtotal * quantity
      total_with_shipping = total_before_tax + shipping_cost
      tax_amount = total_with_shipping * TAX_RATE
      return tax_amount
  \`\`\`

**Issue 3: Magic Numbers**
- **Location:** billing/discounts.py:45
- **Code:**
  \`\`\`python
  if order_total > 100:
      discount = order_total * 0.15
  \`\`\`
- **Issue:** 100 and 0.15 are magic numbers
- **Fix:**
  \`\`\`python
  BULK_ORDER_THRESHOLD = 100  # dollars
  BULK_DISCOUNT_RATE = 0.15   # 15%
  
  if order_total > BULK_ORDER_THRESHOLD:
      discount = order_total * BULK_DISCOUNT_RATE
  \`\`\`

### Maintainability

- [ ] Functions/methods under 50 lines
- [ ] Classes have single responsibility
- [ ] Code follows DRY principle (no duplication)
- [ ] Appropriate use of abstractions
- [ ] Easy to modify without breaking other code

**Issues found:** [N]

**Issue 4: God Class**
- **Location:** models/user.py:UserManager
- **Issue:** UserManager handles user CRUD, authentication, permissions, notifications, billing
- **Lines:** 1200 lines, 45 methods
- **Impact:** Difficult to understand, test, and modify
- **Recommendation:** Split into:
  - UserRepository (CRUD)
  - AuthenticationService
  - PermissionService
  - UserNotificationService
  - UserBillingService

**Issue 5: Code Duplication**
- **Location:** api/users.py and api/admins.py
- **Duplicated:**
  \`\`\`python
  # Same 50 lines of validation logic in both files
  if not email:
      raise ValueError('Email required')
  if not re.match(EMAIL_REGEX, email):
      raise ValueError('Invalid email')
  if len(password) < 8:
      raise ValueError('Password too short')
  # ... 40 more lines
  \`\`\`
- **Fix:** Extract to \`validators.validate_user_input()\`

### Design

- [ ] Appropriate design patterns used
- [ ] Separation of concerns (business logic vs presentation)
- [ ] Dependency injection where appropriate
- [ ] Interface/contract usage for flexibility
- [ ] Avoid tight coupling between modules

**Issues found:** [N]

**Issue 6: Tight Coupling**
- **Location:** services/order_processor.py
- **Issue:** OrderProcessor directly instantiates EmailService, PaymentGateway, InventorySystem
- **Impact:** Cannot test OrderProcessor without real dependencies, difficult to swap implementations
- **Fix:** Use dependency injection
  \`\`\`python
  # Before
  class OrderProcessor:
      def __init__(self):
          self.email = EmailService()
          self.payment = PaymentGateway()
          self.inventory = InventorySystem()
  
  # After
  class OrderProcessor:
      def __init__(self, email_service, payment_gateway, inventory_system):
          self.email = email_service
          self.payment = payment_gateway
          self.inventory = inventory_system
  \`\`\`

### Testing

- [ ] Unit tests for new functionality
- [ ] Integration tests for API endpoints
- [ ] Edge cases tested
- [ ] Error cases tested
- [ ] Tests are clear and maintainable
- [ ] Mocks used appropriately

**Issues found:** [N]

**Issue 7: Missing Tests for Error Cases**
- **Location:** tests/test_orders.py
- **Issue:** Tests only happy path, no tests for validation errors, payment failures, inventory out of stock
- **Coverage:** 60% (missing error paths)
- **Recommendation:** Add tests for:
  - Invalid order data (missing fields, negative quantities)
  - Payment gateway failure
  - Insufficient inventory
  - Concurrent order conflicts

**Issue 8: Poor Test Quality**
- **Location:** tests/test_user_service.py:test_create_user
- **Code:**
  \`\`\`python
  def test_create_user():
      result = create_user('test', 'test@example.com', 'password')
      assert result is not None
  \`\`\`
- **Issue:** Test too vague, doesn't verify actual behavior
- **Fix:**
  \`\`\`python
  def test_create_user_returns_user_with_correct_fields():
      username = 'testuser'
      email = 'test@example.com'
      password = 'securepass123'
      
      user = create_user(username, email, password)
      
      assert user.username == username
      assert user.email == email
      assert user.password_hash is not None
      assert user.password_hash != password  # Hashed, not plaintext
      assert user.created_at is not None
      assert user.is_active is True
  \`\`\`

### Performance

- [ ] No obvious performance issues
- [ ] Database queries optimized (no N+1 queries)
- [ ] Appropriate use of caching
- [ ] Avoid unnecessary computation in loops
- [ ] Large data handled efficiently

**Issues found:** [N]

**Issue 9: N+1 Query**
- **Location:** api/orders.py:get_orders_with_items
- **Code:**
  \`\`\`python
  orders = db.query('SELECT * FROM orders WHERE user_id = ?', user_id)
  for order in orders:
      order.items = db.query('SELECT * FROM order_items WHERE order_id = ?', order.id)
  \`\`\`
- **Impact:** 1 + N queries for N orders
- **Fix:** Use JOIN
  \`\`\`python
  query = '''
      SELECT o.*, oi.*
      FROM orders o
      LEFT JOIN order_items oi ON oi.order_id = o.id
      WHERE o.user_id = ?
  '''
  results = db.query(query, user_id)
  orders = group_by_order(results)
  \`\`\`

### Security

- [ ] No security vulnerabilities (see security review skill)
- [ ] Input validation and sanitization
- [ ] Output encoding to prevent XSS
- [ ] Authentication and authorization checked
- [ ] No sensitive data in logs

**Issues found:** [N]

**Issue 10: Logging Sensitive Data**
- **Location:** api/auth.py:45
- **Code:**
  \`\`\`python
  logger.info(f'Login attempt: username={username}, password={password}')
  \`\`\`
- **Issue:** Passwords logged in plaintext
- **Impact:** Password exposure if logs leaked
- **Fix:**
  \`\`\`python
  logger.info(f'Login attempt: username={username}')
  \`\`\`

### Documentation

- [ ] Complex logic has explanatory comments
- [ ] Public APIs have docstrings
- [ ] README updated if needed
- [ ] Breaking changes documented

**Issues found:** [N]

**Issue 11: Missing Docstrings**
- **Location:** utils/crypto.py
- **Issue:** Cryptographic functions lack documentation
- **Fix:** Add docstrings explaining parameters, return values, and usage examples

## Summary

**Total issues:** [N]
- Correctness: [N]
- Readability: [N]
- Maintainability: [N]
- Design: [N]
- Testing: [N]
- Performance: [N]
- Security: [N]
- Documentation: [N]

**Recommendation:** [Approve / Request changes / Reject]

**Required changes before approval:**
1. [Critical issue to fix]
2. [Another critical issue]

**Suggested improvements (non-blocking):**
1. [Nice-to-have improvement]
2. [Another suggestion]

**Positive feedback:**
- [What the code does well]
- [Another strength]" \
  --confidence 0.85 \
  --tags "code-quality,manual-review,[component-name]"
```

### Step 3: Style Guide Conformance

Check adherence to team style guide and conventions:

```bash
engram reasoning create \
  --title "Style Guide Conformance: [Component]" \
  --task-id [TASK_ID] \
  --content "## Style Guide

**Language:** [Python, TypeScript, Java, etc.]
**Style guide:** [PEP 8, Airbnb JavaScript, Google Java Style, team-specific]
**Enforcement:** [ESLint, Pylint, Prettier, etc.]

## Naming Conventions

**Functions/Methods:**
- Convention: [snake_case, camelCase, etc.]
- Verbs for actions: \`get_user()\`, \`calculate_total()\`, \`send_email()\`
- Booleans prefixed: \`is_active\`, \`has_permission\`, \`can_edit\`

**Violations:**
- [function_name] in [file]: Should be [correct_name] - [reason]

**Classes:**
- Convention: [PascalCase]
- Nouns describing what they represent: \`User\`, \`OrderProcessor\`, \`PaymentGateway\`

**Violations:**
- [ClassName] in [file]: [issue]

**Variables:**
- Convention: [snake_case, camelCase]
- Descriptive names, avoid single letters except loop counters

**Violations:**
- [var_name] in [file:line]: Too vague, should be [better_name]

**Constants:**
- Convention: [UPPER_SNAKE_CASE]
- Module-level constants defined at top

**Violations:**
- [constant] in [file]: Not uppercase

## Code Organization

**File structure:**
- [Expected structure: imports, constants, classes, functions, main]

**Import ordering:**
- Standard library imports
- Third-party imports
- Local application imports
- Alphabetically within each group

**Violations:**
- [file]: Imports not ordered correctly

**Class organization:**
- [Expected order: constructor, public methods, private methods]

**Violations:**
- [class] in [file]: Private methods not grouped at bottom

## Formatting

**Indentation:**
- Style: [2 spaces, 4 spaces, tabs]
- No mixed tabs and spaces

**Line length:**
- Maximum: [80, 100, 120] characters
- Break long lines at logical points

**Violations:**
- [file]: [N] lines exceed maximum length

**Blank lines:**
- 2 blank lines between top-level functions/classes
- 1 blank line between methods

**Whitespace:**
- No trailing whitespace
- Single space after commas
- No space before commas/colons/semicolons

## Comments and Documentation

**Docstrings:**
- Required for: [all public functions, all classes, all modules]
- Format: [Google style, NumPy style, JSDoc]

**Example:**
\`\`\`python
def calculate_discount(order_total: float, user_tier: str) -> float:
    \"\"\"Calculate discount amount based on order total and user tier.
    
    Args:
        order_total: Total order amount in dollars
        user_tier: User membership tier ('basic', 'premium', 'vip')
    
    Returns:
        Discount amount in dollars
    
    Raises:
        ValueError: If user_tier is not recognized
    \"\"\"
\`\`\`

**Violations:**
- [function] in [file]: Missing docstring

**Inline comments:**
- Explain why, not what (code should be self-documenting)
- Above or beside line, not below

**TODO comments:**
- Format: \`# TODO(username): description\`
- Should be tracked in issue tracker

**Violations:**
- [file:line]: TODO without owner or ticket reference

## Type Annotations (if applicable)

**Required:**
- Function parameters and return types
- Class attributes
- Module-level variables

**Example:**
\`\`\`python
def process_order(
    order_id: str,
    items: List[OrderItem],
    payment: PaymentInfo
) -> OrderResult:
    ...
\`\`\`

**Violations:**
- [function] in [file]: Missing type annotations

## Error Handling

**Exceptions:**
- Raise specific exceptions, not generic Exception
- Include descriptive error messages
- Document exceptions in docstring

**Example:**
\`\`\`python
def withdraw(amount: float) -> None:
    \"\"\"Withdraw amount from account.
    
    Raises:
        InsufficientFundsError: If amount exceeds balance
        ValueError: If amount is negative
    \"\"\"
    if amount < 0:
        raise ValueError(f'Amount must be positive, got {amount}')
    if amount > self.balance:
        raise InsufficientFundsError(
            f'Insufficient funds: balance={self.balance}, requested={amount}'
        )
\`\`\`

**Violations:**
- [function] in [file]: Raises generic Exception

## Summary

**Style guide conformance:** [N]% compliant

**Categories:**
- Naming: [N] violations
- Organization: [N] violations
- Formatting: [N] violations
- Documentation: [N] violations
- Type annotations: [N] violations
- Error handling: [N] violations

**Action items:**
1. Run auto-formatter: \`[command]\`
2. Fix [N] naming violations
3. Add [N] missing docstrings

**Estimated effort:** [N] hours" \
  --confidence 0.80 \
  --tags "code-quality,style-guide,[component-name]"
```

### Step 4: Technical Debt Assessment

Identify and prioritize technical debt:

```bash
engram reasoning create \
  --title "Technical Debt Assessment: [Component]" \
  --task-id [TASK_ID] \
  --content "## Technical Debt Inventory

**Total debt items:** [N]
**Estimated cost to fix:** [N] hours/days

### High-Priority Debt (Impeding development)

**Debt Item 1: Monolithic User Service**
- **Type:** Design debt
- **Description:** UserService has grown to 2000 lines, handles authentication, permissions, profile, notifications, billing
- **Impact:**
  - Adding features requires understanding entire service
  - Tests slow (45 seconds for full suite)
  - Multiple teams blocked by merge conflicts
  - New developer onboarding takes 2+ weeks
- **Cost to fix:** 40 hours
- **Cost of delay:** 5 hours/week developer time wasted
- **ROI:** Pays back in 8 weeks
- **Recommendation:** Refactor into separate services

**Debt Item 2: [Another high-priority item]**
- [Same structure]

### Medium-Priority Debt (Slowing velocity)

**Debt Item 3: Inadequate Test Coverage**
- **Type:** Testing debt
- **Description:** Core business logic at 45% coverage
- **Impact:**
  - Fear of refactoring (will something break?)
  - Bugs caught in production, not CI
  - Regression rate: 2-3 bugs per release
- **Cost to fix:** 30 hours (write missing tests)
- **Cost of delay:** 10 hours/sprint fixing production bugs
- **ROI:** Pays back in 3 sprints
- **Recommendation:** Allocate 20% of sprint to testing

**Debt Item 4: [Another medium-priority item]**
- [Same structure]

### Low-Priority Debt (Annoyances)

**Debt Item 5: Inconsistent Error Handling**
- **Type:** Code quality debt
- **Description:** Some functions raise exceptions, others return error objects
- **Impact:** Confusing for developers, leads to bugs
- **Cost to fix:** 8 hours
- **Cost of delay:** 1 hour/sprint confusion/bugs
- **ROI:** Pays back in 8 sprints
- **Recommendation:** Address in next refactoring pass

## Technical Debt Quadrant

**Reckless + Deliberate:**
- [Debt items from \"we don't have time\" decisions]
- Example: Skipped testing to meet deadline

**Reckless + Inadvertent:**
- [Debt from lack of knowledge/experience]
- Example: Didn't know about design patterns

**Prudent + Deliberate:**
- [Strategic debt from conscious trade-offs]
- Example: Shipped MVP, planned refactor in Phase 2

**Prudent + Inadvertent:**
- [Debt from learning better approaches]
- Example: Refactored after learning new technique

## Debt Paydown Plan

**Sprint 1-2: High-priority debt**
- Refactor UserService into separate services
- Effort: 40 hours
- Team: 2 engineers for 1 sprint

**Sprint 3-5: Medium-priority debt**
- Increase test coverage to 80%
- Effort: 30 hours
- Team: 1 engineer, 20% time per sprint

**Ongoing: Prevent new debt**
- Code review checklist includes debt prevention
- Quarterly code quality reviews
- Refactoring time in every sprint (20%)

## Monitoring

**Metrics to track:**
- Test coverage (target: 80%)
- Cyclomatic complexity (target: avg <10)
- Lines of code per file (target: <500)
- Technical debt ratio (SonarQube)
- Time to onboard new developer

**Review cadence:** Quarterly technical debt assessment" \
  --confidence 0.75 \
  --tags "code-quality,technical-debt,[component-name]"
```

### Step 5: Link Code Quality Entities to Task

```bash
# Link code quality analysis
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [REPORT_ID] --target-type context \
  --relationship-type references \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [MANUAL_REVIEW_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [STYLE_GUIDE_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [TECH_DEBT_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]
```

## Example

User submits PR with new payment processing feature, needs code review.

### Step 1: Automated Analysis

Agent runs linters and coverage:

```bash
# Run linting
npm run lint
# 15 errors, 23 warnings

# Run tests with coverage
npm test -- --coverage
# Coverage: 68% (below 80% target)
```

Agent creates quality report documenting 38 total issues.

### Step 2: Manual Review

Agent reviews code manually:

"Manual review found:
- Correctness: Off-by-one error in refund calculation (customer charged wrong amount)
- Readability: Function named 'proc' should be 'process_payment'
- Maintainability: 200-line function should be split into 5 smaller functions
- Testing: No tests for payment failure cases
- Performance: N+1 query fetching transaction history

Total: 5 critical issues, 12 suggestions. Requesting changes before approval."

### Step 3: Style Guide Check

Agent checks style conformance:

"Style guide violations:
- 8 functions missing docstrings
- 12 lines exceed 100 character limit
- 3 variables named with single letters (x, y, z)
- Imports not alphabetically sorted

Recommend: Run Prettier to auto-fix formatting, manually add docstrings."

### Step 4: Technical Debt Assessment

Agent identifies debt introduced:

"New technical debt added:
- Payment logic tightly coupled to Stripe API (hard to swap providers)
- Retry logic duplicated in 3 places (DRY violation)

Existing debt worsened:
- PaymentService now 800 lines (was 600) - approaching unmaintainable

Recommend: Extract retry logic into decorator, plan PaymentService refactor next sprint."

## Querying Code Quality Reviews

```bash
# Get quality reports
engram context list | grep "Code Quality Report:"

# Get manual reviews
engram reasoning list | grep "Manual Code Review:"

# Get style guide conformance
engram reasoning list | grep "Style Guide Conformance:"

# Get technical debt assessments
engram reasoning list | grep "Technical Debt Assessment:"

# Get all quality work for a component
engram relationship connected --entity-id [TASK_ID] | grep -i "quality"
```

## Related Skills

This skill integrates with:
- `engram-refactoring-strategy` - Plan refactoring to address quality issues
- `engram-security-review` - Security is part of code quality
- `engram-performance-analysis` - Performance is part of code quality
- `engram-test-driven-development` - TDD produces higher quality code
- `engram-requesting-code-review` - Structure review requests effectively
