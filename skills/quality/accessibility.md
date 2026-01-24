---
name: engram-accessibility
description: "Audit applications for WCAG compliance, screen reader support, keyboard navigation, and test with assistive technologies."
---

# Accessibility (Engram-Integrated)

## Overview

Systematically audit web applications and interfaces for accessibility compliance with WCAG guidelines, ensure screen reader compatibility, verify keyboard navigation, test with assistive technologies, and document accessibility conformance. Store accessibility audits, remediation plans, and testing results in Engram to track progress toward inclusive design.

## When to Use

Use this skill when:
- Building new user interfaces or components
- Preparing for accessibility compliance certification (WCAG 2.1 AA/AAA)
- User reports accessibility issues or cannot use features
- Legal or procurement requires accessibility conformance
- Conducting code reviews for frontend changes
- Planning product roadmap and need to assess accessibility debt
- Testing with assistive technologies before release

## The Pattern

### Step 1: Conduct Accessibility Audit

Systematically check against WCAG principles (Perceivable, Operable, Understandable, Robust):

```bash
engram context create \
  --title "Accessibility Audit: [Page/Component Name]" \
  --content "## Audit Scope

**Component:** [e.g., Dashboard, Checkout flow, Settings page]
**URL/Path:** [URL or component path]
**WCAG Level Target:** [A / AA / AAA]
**Audit Date:** $(date -u +%Y-%m-%dT%H:%M:%SZ)
**Auditor:** [Name/Agent]

## Audit Tools

**Automated:**
- axe DevTools (Chrome extension)
- WAVE (Web Accessibility Evaluation Tool)
- Lighthouse Accessibility Score
- Pa11y CI

**Manual:**
- Screen reader testing (NVDA, JAWS, VoiceOver)
- Keyboard-only navigation
- Color contrast analyzer
- Browser zoom testing (up to 200%)

## WCAG 2.1 Compliance Audit

### Principle 1: Perceivable

**1.1 Text Alternatives (Level A)**

**1.1.1 Non-text Content:**
- [ ] All images have alt text
- [ ] Decorative images use alt=\"\" or aria-hidden=\"true\"
- [ ] Icons have accessible labels
- [ ] Charts/graphs have text descriptions
- **Issues found:** [N]
  - [Specific issue: e.g., Logo missing alt text]
  - [Another issue]

**1.2 Time-based Media (Level A)**

**1.2.1 Audio-only and Video-only:**
- [ ] Audio content has transcript
- [ ] Video content has captions
- **Issues found:** [N]
  - [Specific issue]

**1.3 Adaptable (Level A)**

**1.3.1 Info and Relationships:**
- [ ] Semantic HTML used (header, nav, main, article)
- [ ] Headings in logical order (h1 → h2 → h3, no skipping)
- [ ] Form labels associated with inputs
- [ ] Lists use ul/ol/dl elements
- [ ] Tables have th headers with scope
- **Issues found:** [N]
  - [Specific issue: e.g., Skip nav heading levels h1 → h3]
  - [Another issue]

**1.3.2 Meaningful Sequence:**
- [ ] Reading order matches visual order
- [ ] Tab order is logical
- **Issues found:** [N]

**1.3.3 Sensory Characteristics:**
- [ ] Instructions don't rely only on shape/color/position
- **Issues found:** [N]

**1.4 Distinguishable (Level AA)**

**1.4.1 Use of Color:**
- [ ] Color not sole indicator of information
- [ ] Links distinguishable without color (underline/weight)
- **Issues found:** [N]
  - [Specific issue: e.g., Error messages only red, no icon]

**1.4.3 Contrast (Minimum):**
- [ ] Normal text: 4.5:1 contrast ratio
- [ ] Large text (18pt+): 3:1 contrast ratio
- [ ] UI components: 3:1 contrast ratio
- **Issues found:** [N]
  - [Specific issue: e.g., Button text #777 on #fff = 3.2:1 (FAIL)]
  - [Another issue]

**1.4.4 Resize Text:**
- [ ] Page usable at 200% zoom
- [ ] No horizontal scrolling at 200% zoom (responsive)
- **Issues found:** [N]

**1.4.10 Reflow (Level AA - WCAG 2.1):**
- [ ] Content reflows at 320px width (mobile)
- [ ] No two-dimensional scrolling required
- **Issues found:** [N]

**1.4.11 Non-text Contrast (Level AA - WCAG 2.1):**
- [ ] Form inputs have 3:1 contrast with background
- [ ] Buttons have 3:1 contrast
- [ ] Focus indicators have 3:1 contrast
- **Issues found:** [N]
  - [Specific issue: e.g., Input border #ddd on #fff = 1.3:1 (FAIL)]

**1.4.13 Content on Hover or Focus (Level AA - WCAG 2.1):**
- [ ] Tooltips dismissible (ESC key)
- [ ] Hover content doesn't obscure trigger
- [ ] Content remains visible while hovered
- **Issues found:** [N]

### Principle 2: Operable

**2.1 Keyboard Accessible (Level A)**

**2.1.1 Keyboard:**
- [ ] All functionality available via keyboard
- [ ] No keyboard trap (can tab out of all elements)
- [ ] Custom widgets have keyboard support
- **Issues found:** [N]
  - [Specific issue: e.g., Modal cannot be closed with ESC key]
  - [Another issue]

**2.1.2 No Keyboard Trap:**
- [ ] Focus can move away from all components
- **Issues found:** [N]

**2.1.4 Character Key Shortcuts (Level A - WCAG 2.1):**
- [ ] Single-key shortcuts can be disabled or remapped
- [ ] Shortcuts only active when component focused
- **Issues found:** [N]

**2.2 Enough Time (Level A)**

**2.2.1 Timing Adjustable:**
- [ ] Time limits can be extended/disabled
- [ ] User warned before timeout
- **Issues found:** [N]

**2.2.2 Pause, Stop, Hide:**
- [ ] Auto-updating content can be paused
- [ ] Carousels have pause button
- **Issues found:** [N]
  - [Specific issue: e.g., Carousel auto-advances, no pause button]

**2.3 Seizures and Physical Reactions (Level A)**

**2.3.1 Three Flashes or Below Threshold:**
- [ ] No content flashes more than 3 times per second
- **Issues found:** [N]

**2.4 Navigable (Level AA)**

**2.4.1 Bypass Blocks:**
- [ ] Skip navigation link present
- [ ] Skip link is keyboard accessible
- **Issues found:** [N]

**2.4.2 Page Titled:**
- [ ] Page has descriptive title
- [ ] Title reflects current page/view
- **Issues found:** [N]

**2.4.3 Focus Order:**
- [ ] Tab order is logical
- [ ] Focus doesn't jump unexpectedly
- **Issues found:** [N]
  - [Specific issue: e.g., Tab order jumps from header to footer, skipping main content]

**2.4.4 Link Purpose (In Context):**
- [ ] Link text describes destination
- [ ] No \"click here\" or \"read more\" without context
- **Issues found:** [N]

**2.4.5 Multiple Ways:**
- [ ] Multiple ways to find pages (nav, search, sitemap)
- **Issues found:** [N]

**2.4.6 Headings and Labels:**
- [ ] Headings describe topic
- [ ] Form labels descriptive
- **Issues found:** [N]

**2.4.7 Focus Visible:**
- [ ] Keyboard focus indicator visible
- [ ] Focus indicator meets 3:1 contrast ratio
- [ ] Focus indicator not removed with CSS
- **Issues found:** [N]
  - [Specific issue: e.g., :focus outline removed globally]

### Principle 3: Understandable

**3.1 Readable (Level A)**

**3.1.1 Language of Page:**
- [ ] HTML lang attribute set
- [ ] Lang attribute matches content language
- **Issues found:** [N]

**3.1.2 Language of Parts:**
- [ ] Foreign language sections have lang attribute
- **Issues found:** [N]

**3.2 Predictable (Level A)**

**3.2.1 On Focus:**
- [ ] Focus doesn't trigger unexpected context change
- **Issues found:** [N]

**3.2.2 On Input:**
- [ ] Input doesn't trigger unexpected context change
- [ ] Form doesn't submit on select change without warning
- **Issues found:** [N]

**3.2.3 Consistent Navigation:**
- [ ] Navigation in same order across pages
- **Issues found:** [N]

**3.2.4 Consistent Identification:**
- [ ] Icons/buttons have consistent labels across pages
- **Issues found:** [N]

**3.3 Input Assistance (Level AA)**

**3.3.1 Error Identification:**
- [ ] Errors identified in text (not just color)
- [ ] Error messages descriptive
- **Issues found:** [N]
  - [Specific issue: e.g., \"Invalid input\" without specifying which field]

**3.3.2 Labels or Instructions:**
- [ ] Form fields have visible labels
- [ ] Required fields indicated
- [ ] Input format specified (e.g., \"MM/DD/YYYY\")
- **Issues found:** [N]

**3.3.3 Error Suggestion:**
- [ ] Error messages suggest correction
- **Issues found:** [N]

**3.3.4 Error Prevention (Legal, Financial, Data):**
- [ ] Reversible submissions
- [ ] Confirmation step for important actions
- **Issues found:** [N]

### Principle 4: Robust

**4.1 Compatible (Level A)**

**4.1.1 Parsing:**
- [ ] HTML validates (no duplicate IDs, proper nesting)
- [ ] ARIA attributes valid
- **Issues found:** [N]
  - [Specific issue: e.g., Duplicate id=\"submit-btn\" on two buttons]

**4.1.2 Name, Role, Value:**
- [ ] Custom widgets have proper ARIA roles
- [ ] Form inputs have accessible names
- [ ] Component states exposed to assistive tech
- **Issues found:** [N]

**4.1.3 Status Messages (Level AA - WCAG 2.1):**
- [ ] Status messages use role=\"status\" or aria-live
- [ ] Success/error messages announced to screen readers
- **Issues found:** [N]
  - [Specific issue: e.g., \"Saved successfully\" toast not announced]

## Summary

**Total Issues:** [N]
**Critical (blocks use):** [N]
**Serious (major barrier):** [N]
**Moderate (inconvenience):** [N]
**Minor (small annoyance):** [N]

**WCAG Compliance:**
- Level A: [Pass/Fail] - [N] issues
- Level AA: [Pass/Fail] - [N] issues
- Level AAA: [N/A or Pass/Fail] - [N] issues

**Lighthouse Accessibility Score:** [N]/100

**Recommendation:** [Compliant / Needs remediation / Major barriers present]" \
  --source "accessibility-audit" \
  --tags "accessibility,audit,wcag,[component-name]"
```

### Step 2: Test with Screen Readers

```bash
engram reasoning create \
  --title "Screen Reader Testing: [Page/Component]" \
  --task-id [TASK_ID] \
  --content "## Screen Reader Testing

### Testing Environment

**Screen Readers Tested:**
- NVDA 2024.1 (Windows + Firefox)
- JAWS 2024 (Windows + Chrome)
- VoiceOver (macOS + Safari)
- TalkBack (Android + Chrome)

**Test Scenarios:**
1. Navigate page using headings (H key)
2. Navigate using landmarks (D key)
3. Navigate form fields (F key)
4. Read all content (Arrow keys / Swipe)
5. Interact with custom widgets

### NVDA Testing Results

**Navigation:**
- [ ] Page structure announced correctly
- [ ] Landmarks accessible (banner, main, navigation, contentinfo)
- [ ] Headings in logical order
- **Issues:** [N]
  - [Specific issue: e.g., Main landmark missing, jumps directly to content]

**Forms:**
- [ ] Labels announced with fields
- [ ] Required fields indicated
- [ ] Error messages associated and announced
- [ ] Radio/checkbox groups have group labels
- **Issues:** [N]
  - [Specific issue: e.g., \"Required\" not announced for email field]

**Interactive Elements:**
- [ ] Buttons announce role and label
- [ ] Links announce as links
- [ ] Custom widgets announce role (e.g., \"Tab, selected\")
- [ ] State changes announced (expanded/collapsed)
- **Issues:** [N]
  - [Specific issue: e.g., Accordion announced as \"button\" not \"button expanded\"]

**Dynamic Content:**
- [ ] New content announced (aria-live)
- [ ] Loading states announced
- [ ] Toasts/notifications announced
- **Issues:** [N]
  - [Specific issue: e.g., \"Loading...\" not announced when fetching data]

### JAWS Testing Results

**Similar categories as NVDA...**
**Issues:** [N]
- [JAWS-specific issues]

### VoiceOver Testing Results

**Rotor Navigation:**
- [ ] Headings accessible via rotor
- [ ] Landmarks accessible via rotor
- [ ] Forms accessible via rotor
- **Issues:** [N]

**Touch Gestures (iOS):**
- [ ] Swipe left/right navigates correctly
- [ ] Double-tap activates elements
- [ ] Custom gestures work (if applicable)
- **Issues:** [N]

### TalkBack Testing Results

**Android-Specific:**
- [ ] Navigation drawer accessible
- [ ] Bottom navigation accessible
- [ ] Floating action buttons labeled
- **Issues:** [N]

## Critical Findings

**Blocking Issues (Cannot use feature):**
1. [Issue preventing core functionality]
2. [Another blocking issue]

**Major Issues (Significant barrier):**
1. [Issue causing major inconvenience]
2. [Another major issue]

## Remediation Priority

**P0 (Immediate fix):**
- [Blocking issue with remediation]

**P1 (Fix before release):**
- [Major issue with remediation]

**P2 (Fix in next sprint):**
- [Moderate issue with remediation]" \
  --confidence 0.85 \
  --tags "accessibility,screen-reader,[component-name]"
```

### Step 3: Test Keyboard Navigation

```bash
engram reasoning create \
  --title "Keyboard Navigation Testing: [Page/Component]" \
  --task-id [TASK_ID] \
  --content "## Keyboard Navigation Testing

### Testing Method

**Test without mouse/trackpad** - keyboard only

**Key bindings tested:**
- Tab / Shift+Tab: Move focus forward/backward
- Enter / Space: Activate buttons/links
- Arrow keys: Navigate within widgets
- Escape: Close dialogs/menus
- Home / End: Jump to start/end
- Page Up / Page Down: Scroll

### Focus Management

**Initial Focus:**
- [ ] Skip link receives focus first (or first interactive element)
- [ ] Focus not lost on page load
- **Issues:** [N]

**Tab Order:**
- [ ] Tab order matches visual order
- [ ] All interactive elements focusable
- [ ] No unexpected focus jumps
- [ ] Hidden elements not in tab order
- **Issues:** [N]
  - [Specific issue: e.g., Tab order goes header → footer → main content]
  - [Another issue]

**Focus Indicators:**
- [ ] Visible focus outline on all elements
- [ ] Focus outline meets 3:1 contrast ratio
- [ ] Focus outline not removed or barely visible
- [ ] Custom focus styles for brand consistency (if outline removed)
- **Issues:** [N]
  - [Specific issue: e.g., outline: none in CSS with no custom indicator]

**Keyboard Traps:**
- [ ] Can tab out of all components
- [ ] Modal dialogs trap focus (but can ESC to close)
- [ ] No infinite loops
- **Issues:** [N]

### Interactive Component Testing

**Forms:**
- [ ] Tab through all fields in order
- [ ] Labels visible and associated
- [ ] Error messages receive focus
- [ ] Submit with Enter key
- **Issues:** [N]

**Buttons:**
- [ ] Activatable with Enter or Space
- [ ] Focus visible on all buttons
- **Issues:** [N]

**Links:**
- [ ] Activatable with Enter
- [ ] Skip link works (jumps to main content)
- **Issues:** [N]

**Dropdowns/Select:**
- [ ] Open with Space or Enter
- [ ] Navigate options with Arrow keys
- [ ] Select with Enter
- [ ] Close with Escape
- **Issues:** [N]

**Custom Dropdowns:**
- [ ] role=\"combobox\" or role=\"listbox\"
- [ ] Arrow keys navigate options
- [ ] Enter selects option
- [ ] ESC closes without selecting
- [ ] Typing filters/jumps to option
- **Issues:** [N]
  - [Specific issue: e.g., Custom dropdown not keyboard accessible]

**Modals/Dialogs:**
- [ ] Focus moves to modal on open
- [ ] Focus trapped within modal
- [ ] ESC key closes modal
- [ ] Focus returns to trigger on close
- **Issues:** [N]

**Accordions:**
- [ ] Arrow keys navigate between headers
- [ ] Enter/Space toggles expanded state
- [ ] Focus visible on headers
- **Issues:** [N]

**Tabs:**
- [ ] Arrow keys navigate between tabs
- [ ] Tab key moves to tab panel
- [ ] Home/End keys jump to first/last tab
- **Issues:** [N]

**Carousels:**
- [ ] Arrow keys or buttons navigate slides
- [ ] Pause button accessible
- [ ] All slides reachable via keyboard
- **Issues:** [N]

**Data Tables:**
- [ ] Tab through interactive cells
- [ ] Sort controls keyboard accessible
- [ ] Pagination keyboard accessible
- **Issues:** [N]

**Tooltips:**
- [ ] Appear on focus (not just hover)
- [ ] Dismissible with ESC
- [ ] Don't block underlying content
- **Issues:** [N]

**Context Menus:**
- [ ] Triggered with keyboard (Shift+F10 or context menu key)
- [ ] Arrow keys navigate menu
- [ ] Enter activates menu item
- [ ] ESC closes menu
- **Issues:** [N]

### Complex Widget Testing

**Rich Text Editor:**
- [ ] All toolbar buttons keyboard accessible
- [ ] Tab moves to editing area
- [ ] Keyboard shortcuts for formatting (if applicable)
- **Issues:** [N]

**Date Picker:**
- [ ] Keyboard opens calendar
- [ ] Arrow keys navigate dates
- [ ] Enter selects date
- [ ] ESC closes calendar
- **Issues:** [N]

**Drag and Drop:**
- [ ] Keyboard alternative provided (move up/down buttons)
- **Issues:** [N]
  - [Specific issue: e.g., No keyboard alternative for drag-drop sorting]

## Summary

**Total keyboard issues:** [N]
**Critical:** [N]
**Serious:** [N]
**Moderate:** [N]
**Minor:** [N]

**Keyboard Accessibility:** [Pass/Fail]

**Critical Fixes Required:**
1. [Fix for blocking issue]
2. [Fix for another issue]" \
  --confidence 0.85 \
  --tags "accessibility,keyboard-navigation,[component-name]"
```

### Step 4: Create Remediation Plan

```bash
engram reasoning create \
  --title "Accessibility Remediation Plan: [Component]" \
  --task-id [TASK_ID] \
  --content "## Issues Summary

**Total issues identified:** [N]
- WCAG Level A: [N] issues
- WCAG Level AA: [N] issues
- Screen reader: [N] issues
- Keyboard navigation: [N] issues

**Severity breakdown:**
- Critical (P0): [N] - blocks usage
- Serious (P1): [N] - major barrier
- Moderate (P2): [N] - inconvenience
- Minor (P3): [N] - small issue

## Remediation Tasks

### P0 - Critical (Must fix immediately)

**Issue 1: [Description]**
- **WCAG Criterion:** [e.g., 2.1.1 Keyboard (Level A)]
- **Impact:** [Who is affected and how]
- **Current behavior:** [What happens now]
- **Expected behavior:** [What should happen]
- **Remediation:**
  \`\`\`html
  <!-- Before -->
  <div class=\"button\" onclick=\"doSomething()\">Click me</div>
  
  <!-- After -->
  <button type=\"button\" onclick=\"doSomething()\">Click me</button>
  \`\`\`
- **Files affected:** [file1.tsx, file2.tsx]
- **Effort estimate:** [N hours]
- **Testing:** [How to verify fix]

**Issue 2: [Description]**
- [Same structure as above]

### P1 - Serious (Fix before release)

**Issue 3: [Description]**
- [Same structure]

**Issue 4: [Description]**
- [Same structure]

### P2 - Moderate (Fix in next sprint)

**Issue 5: [Description]**
- [Same structure]

### P3 - Minor (Backlog)

**Issue 6: [Description]**
- [Same structure]

## Implementation Plan

**Phase 1 (Sprint 1): P0 Critical Issues**
- Week 1: Fix issues 1-3
- Week 2: Fix issues 4-5, test with screen readers
- **Effort:** [N] hours total
- **Milestone:** Core functionality keyboard accessible

**Phase 2 (Sprint 2): P1 Serious Issues**
- Week 3: Fix issues 6-10
- Week 4: Full WCAG audit, fix remaining Level A
- **Effort:** [N] hours total
- **Milestone:** WCAG Level A compliant

**Phase 3 (Sprint 3): P2 Moderate Issues**
- Week 5-6: Fix Level AA issues, improve experience
- **Effort:** [N] hours total
- **Milestone:** WCAG Level AA compliant

**Phase 4 (Backlog): P3 Minor Issues**
- Addressed as time permits
- **Effort:** [N] hours total

## Testing Strategy

**Automated Testing:**
- Add Pa11y CI to PR checks (blocks merge on critical issues)
- Lighthouse CI scores (minimum 90 for accessibility)
- axe-core in Jest/Cypress tests

**Manual Testing:**
- Screen reader testing (NVDA, VoiceOver) before each release
- Keyboard navigation testing on all new components
- High contrast mode testing
- Zoom to 200% testing

**Acceptance Criteria:**
- All P0 and P1 issues resolved
- Pa11y CI passes with 0 errors
- Lighthouse accessibility score ≥ 90
- Manual screen reader test passes
- Manual keyboard test passes

## Ongoing Accessibility

**Process improvements:**
1. Add accessibility checklist to PR template
2. Include accessibility acceptance criteria in all UI tickets
3. Conduct monthly accessibility office hours
4. Train team on ARIA best practices

**Component library:**
- Document accessibility features for each component
- Provide accessible examples
- Include keyboard shortcuts in component docs

**Monitoring:**
- Monthly automated audits (Pa11y + axe)
- Quarterly manual audits with assistive tech
- Track accessibility issues in dedicated backlog

## Resources

**Documentation:**
- WCAG 2.1 Guidelines: https://www.w3.org/WAI/WCAG21/quickref/
- ARIA Authoring Practices: https://www.w3.org/WAI/ARIA/apg/

**Tools:**
- axe DevTools: https://www.deque.com/axe/devtools/
- WAVE: https://wave.webaim.org/
- Pa11y: https://pa11y.org/

**Team Training:**
- WebAIM Articles: https://webaim.org/articles/
- A11ycasts (video series): https://www.youtube.com/playlist?list=PLNYkxOF6rcICWx0C9LVWWVqvHlYJyqw7g" \
  --confidence 0.80 \
  --tags "accessibility,remediation,[component-name]"
```

### Step 5: Link Accessibility Entities to Task

```bash
# Link all accessibility analysis
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [AUDIT_ID] --target-type context \
  --relationship-type references \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [SCREEN_READER_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [KEYBOARD_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [REMEDIATION_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]
```

## Example

User building a data table component needs accessibility review before release.

### Step 1: Conduct Audit

Agent runs automated tools and finds issues:

```bash
# Run automated audit
npx pa11y http://localhost:3000/data-table

# Results show 8 errors, 3 warnings
```

Agent creates audit context documenting findings (12 total issues: 3 critical, 5 serious, 4 moderate).

### Step 2: Screen Reader Testing

Agent tests with NVDA:

"Testing data table with NVDA. Issues found:
1. Table headers not announced (missing <th> or role=columnheader)
2. Sort buttons not announced as buttons (using <div>)
3. Row selection state not announced (missing aria-selected)
4. Pagination not announced when changed (missing aria-live)"

### Step 3: Keyboard Testing

Agent tests keyboard navigation:

"Issues found:
1. Cannot focus table rows (missing tabindex)
2. Arrow keys don't navigate cells (no keyboard handler)
3. Sort buttons require 5 tabs to reach (inefficient)
4. No keyboard shortcut to jump to pagination"

### Step 4: Create Remediation Plan

Agent prioritizes fixes:

"Created remediation plan with 12 issues:
- P0 (3 issues): Table headers, sort button roles, row focus - 8 hours
- P1 (5 issues): ARIA states, keyboard navigation - 16 hours  
- P2 (4 issues): Keyboard shortcuts, optimizations - 8 hours

Phase 1 (this sprint): Fix P0 + P1 for WCAG Level A/AA compliance.
Phase 2 (next sprint): Fix P2 for enhanced experience.

All issues stored in Engram with remediation code examples."

## Querying Accessibility Analysis

```bash
# Get accessibility audits
engram context list | grep "Accessibility Audit:"

# Get screen reader testing results
engram reasoning list | grep "Screen Reader Testing:"

# Get keyboard navigation testing
engram reasoning list | grep "Keyboard Navigation Testing:"

# Get remediation plans
engram reasoning list | grep "Accessibility Remediation Plan:"

# Get all accessibility work for a component
engram relationship connected --entity-id [TASK_ID] | grep -i "accessibility"
```

## Related Skills

This skill integrates with:
- `engram-code-quality` - Review code for accessibility best practices
- `engram-test-driven-development` - Write accessibility tests before implementation
- `engram-assumption-validation` - Test assumptions about screen reader behavior
- `engram-system-design` - Design accessible components from the start
- `engram-security-review` - Ensure accessibility features don't introduce security issues
