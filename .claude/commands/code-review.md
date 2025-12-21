---
description: Perform deep code review for Rust and TypeScript code with architecture, design patterns, and code quality analysis.
handoffs:
  - label: Apply Review Suggestions
    agent: general-purpose
    prompt: Apply the code improvements suggested in the review report
    send: true
  - label: Generate Refactoring Plan
    agent: Plan
    prompt: Create a detailed refactoring plan based on review findings
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Outline

The text the user typed after the command trigger **is** the file path or directory to review. Assume you always have it available in this conversation even if `$ARGUMENTS` appears literally below.

### Execution Flow

1. **Parse Review Scope from User Input**:
   - Extract file paths or directory patterns from arguments
   - If empty, ask user to specify the scope (specific files, directory, or entire codebase)
   - Valid inputs:
     - Single file: `backend/src/services/database_service.rs`
     - Multiple files: `backend/src/api/*.rs`
     - Directory: `backend/src/services`
     - Glob pattern: `**/*.rs` or `frontend/src/components/**/*.tsx`
   - Examples:
     - Review single file: `/code-review backend/src/services/database_service.rs`
     - Review all services: `/code-review backend/src/services/*.rs`
     - Review frontend components: `/code-review frontend/src/components`

2. **Identify Review Target Files**:

   a. Use Glob tool to find all matching files based on user input:
      - For Rust: `**/*.rs` pattern
      - For TypeScript: `**/*.ts`, `**/*.tsx` patterns
      - Exclude test files, generated files, and build artifacts by default

   b. Categorize files by type:
      - **Backend Rust**: Models, Services, API handlers, Database layer
      - **Frontend TypeScript**: Components, API clients, Types, Utilities

   c. Determine review depth:
      - **Quick review** (default, 1-3 files): Focus on critical issues only
      - **Standard review** (4-10 files): Include architecture and design patterns
      - **Deep review** (11+ files): Comprehensive analysis including cross-file patterns

   d. Confirm scope with user if more than 15 files found:
      - Show file count and estimated review time
      - Ask if user wants to proceed or narrow scope

3. **Create Review Structure**:

   a. Generate unique review ID based on timestamp and scope:
      - Format: `review-YYYYMMDD-HHMMSS-{short-scope-name}`
      - Example: `review-20250121-143022-database-services`

   b. Create review directory structure:
      ```
      .claude/reviews/{review-id}/
      ├── report.md              # Main review report
      ├── findings/              # Detailed findings by category
      │   ├── architecture.md
      │   ├── design-patterns.md
      │   ├── code-quality.md
      │   └── suggestions.md
      └── checklists/
          ├── improvements.md    # Prioritized improvement checklist
          └── validation.md      # Validation checklist for fixes
      ```

   c. Use PowerShell command to create directories:
      ```powershell
      $reviewId = "review-{timestamp}-{scope}"
      New-Item -ItemType Directory -Path ".claude/reviews/$reviewId/findings" -Force
      New-Item -ItemType Directory -Path ".claude/reviews/$reviewId/checklists" -Force
      ```

4. **Read and Analyze Code**:

   a. For each file in scope, use Read tool to load content

   b. Perform multi-dimensional analysis (see Analysis Dimensions section below)

   c. Document findings in structured format:
      ```markdown
      ### Finding: {Title}

      **File**: {file_path}:{line_number}
      **Severity**: Critical | High | Medium | Low
      **Category**: Architecture | Design | Code Quality | Performance | Security
      **Principle Violated**: {SOLID/DRY/KISS/YAGNI principle}

      **Issue**:
      {Description of the problem}

      **Current Code**:
      ```{language}
      {Code snippet showing the issue}
      ```

      **Why This Matters**:
      {Impact on maintainability, performance, or architecture}

      **Recommendation**:
      {Specific actionable suggestion}

      **Improved Code**:
      ```{language}
      {Code example showing the fix}
      ```

      **Effort**: Low | Medium | High
      **Priority**: P0 (Critical) | P1 (High) | P2 (Medium) | P3 (Low)
      ```

5. **Analysis Dimensions**:

   a. **Architecture and Design** (`findings/architecture.md`):
      - **Layered Architecture**:
        - Is the separation of concerns clear? (API → Service → Data layer)
        - Are dependencies pointing in the right direction?
        - Is there coupling between layers that should be independent?
      - **Interface Design**:
        - Are public APIs intuitive and well-documented?
        - Do interfaces follow single responsibility principle?
        - Are trait bounds (Rust) or interface contracts (TypeScript) appropriate?
      - **Extensibility**:
        - Can new features be added without modifying existing code (Open/Closed)?
        - Are there hard-coded values that should be configurable?
        - Is the design flexible for future requirements?
      - **Module Structure**:
        - Are modules organized logically?
        - Is visibility (pub/private) used appropriately?
        - Are module dependencies minimized?

   b. **KISS Principle** (`findings/design-patterns.md`):
      - **Simplicity Check**:
        - Is the solution unnecessarily complex?
        - Are there over-engineered abstractions?
        - Can the code be simplified without losing functionality?
      - **Cognitive Load**:
        - Is the code easy to understand on first read?
        - Are there too many levels of indirection?
        - Is the control flow straightforward?
      - **Premature Optimization**:
        - Are there optimizations that hurt readability without proven benefit?
        - Is there unnecessary performance tuning?

   c. **Code Quality Principles** (`findings/code-quality.md`):
      - **DRY (Don't Repeat Yourself)**:
        - Is there duplicated code that should be extracted?
        - Are there similar patterns that could be abstracted?
        - Check for copy-paste code with minor variations
      - **YAGNI (You Aren't Gonna Need It)**:
        - Is there unused code or features?
        - Are there abstractions for future needs that don't exist yet?
        - Is the code solving problems that haven't occurred?
      - **SOLID Principles**:
        - **Single Responsibility**: Does each function/struct/module do one thing?
        - **Open/Closed**: Can behavior be extended without modification?
        - **Liskov Substitution**: Are subtypes/implementations truly substitutable?
        - **Interface Segregation**: Are interfaces focused and minimal?
        - **Dependency Inversion**: Do high-level modules depend on abstractions?

   d. **Function and Parameter Standards**:
      - **Function Length**:
        - Flag functions over 150 lines as needing refactoring
        - Identify functions that do multiple things
        - Suggest extraction of helper functions or methods
      - **Parameter Count**:
        - Flag functions with more than 7 parameters
        - Suggest using builder pattern or parameter objects
        - Check for boolean parameters (flag smell)

   e. **Builder Pattern Usage** (Rust specific):
      - **When to Use Builder**:
        - Structs with 4+ fields
        - Optional configuration parameters
        - Complex initialization logic
      - **Builder Quality**:
        - Are builders implemented correctly?
        - Do builders provide sensible defaults?
        - Are required vs optional fields clear?
      - **Missing Builders**:
        - Identify structs that would benefit from builder pattern
        - Check for constructors with many parameters

   f. **Language-Specific Best Practices**:

      **Rust-Specific**:
      - **Error Handling**:
        - Are custom error types used appropriately?
        - Is `Result<T, E>` preferred over panics?
        - Are error contexts preserved and descriptive?
      - **Ownership and Lifetimes**:
        - Are borrows used efficiently?
        - Are unnecessary clones avoided?
        - Are lifetimes necessary or can they be elided?
      - **Type Safety**:
        - Are newtypes used for domain concepts?
        - Are generic bounds minimal and appropriate?
        - Is unsafe code justified and well-documented?
      - **Async Patterns**:
        - Are async functions used appropriately?
        - Is there blocking code in async contexts?
        - Are spawned tasks handled correctly?
      - **Naming Conventions**:
        - snake_case for functions and variables
        - PascalCase for types and traits
        - SCREAMING_SNAKE_CASE for constants

      **TypeScript-Specific**:
      - **Type Safety**:
        - Are `any` types avoided?
        - Are union types used appropriately?
        - Are type guards implemented where needed?
      - **React Patterns** (if applicable):
        - Are hooks used correctly (dependencies array, effect cleanup)?
        - Are components properly memoized when needed?
        - Is prop drilling avoided (context or state management)?
      - **Async Patterns**:
        - Are promises handled correctly (async/await vs .then)?
        - Is error handling present for async operations?
        - Are race conditions avoided?
      - **Naming Conventions**:
        - camelCase for variables and functions
        - PascalCase for classes, components, and types
        - UPPER_CASE for constants

6. **Generate Review Report** (`report.md`):

   Create comprehensive review report with this structure:

   ```markdown
   # Code Review Report: {Review ID}

   **Date**: {YYYY-MM-DD HH:MM:SS}
   **Scope**: {Files or directories reviewed}
   **Reviewer**: Claude Code
   **Review Type**: {Quick | Standard | Deep}

   ---

   ## Executive Summary

   ### Overall Health Score: {X}/100

   **Rating Scale**:
   - 90-100: Excellent - Production ready
   - 70-89: Good - Minor improvements recommended
   - 50-69: Fair - Significant improvements needed
   - Below 50: Poor - Major refactoring required

   **Score Breakdown**:
   - Architecture & Design: {X}/25
   - Code Quality & Principles: {X}/25
   - Best Practices: {X}/25
   - Maintainability: {X}/25

   ### Key Findings

   - **Critical Issues**: {count} - Must fix before production
   - **High Priority**: {count} - Should fix in next sprint
   - **Medium Priority**: {count} - Address in near future
   - **Low Priority**: {count} - Nice to have improvements

   ### Top 3 Priorities

   1. {Most critical finding}
   2. {Second most critical finding}
   3. {Third most critical finding}

   ---

   ## Detailed Findings

   ### Architecture and Design

   {Link to findings/architecture.md with summary}

   - Total issues: {count}
   - Critical: {count} | High: {count} | Medium: {count} | Low: {count}

   **Notable Findings**:
   - {Brief description of top 3 architecture issues}

   ### Design Patterns and KISS

   {Link to findings/design-patterns.md with summary}

   - Total issues: {count}
   - Over-engineering instances: {count}
   - Complexity hotspots: {count}

   **Notable Findings**:
   - {Brief description of top 3 complexity issues}

   ### Code Quality Principles

   {Link to findings/code-quality.md with summary}

   - DRY violations: {count}
   - YAGNI violations: {count}
   - SOLID violations: {count}
   - Functions over 150 lines: {count}
   - Functions with 7+ parameters: {count}

   **Notable Findings**:
   - {Brief description of top 3 code quality issues}

   ### Language-Specific Best Practices

   **Rust**:
   - Error handling issues: {count}
   - Ownership/lifetime improvements: {count}
   - Missing builder patterns: {count}

   **TypeScript**:
   - Type safety issues: {count}
   - React pattern violations: {count}
   - Async pattern issues: {count}

   ---

   ## Statistics

   ### Code Metrics

   | Metric | Value | Threshold | Status |
   |--------|-------|-----------|--------|
   | Total Files Reviewed | {count} | - | - |
   | Total Lines of Code | {count} | - | - |
   | Average Function Length | {lines} | 150 | {✓ or ✗} |
   | Longest Function | {lines} | 150 | {✓ or ✗} |
   | Max Parameters | {count} | 7 | {✓ or ✗} |
   | Functions >150 lines | {count} | 0 | {✓ or ✗} |
   | Functions >7 params | {count} | 0 | {✓ or ✗} |

   ### Issue Distribution

   | Severity | Count | Percentage |
   |----------|-------|------------|
   | Critical | {count} | {%} |
   | High     | {count} | {%} |
   | Medium   | {count} | {%} |
   | Low      | {count} | {%} |

   ### Category Distribution

   | Category | Count |
   |----------|-------|
   | Architecture | {count} |
   | Design Patterns | {count} |
   | Code Quality | {count} |
   | Performance | {count} |
   | Security | {count} |
   | Best Practices | {count} |

   ---

   ## Recommendations

   ### Immediate Actions (P0 - Critical)

   {List of critical issues that must be addressed}

   ### Short-term Improvements (P1 - High)

   {List of high-priority improvements for next sprint}

   ### Medium-term Enhancements (P2 - Medium)

   {List of medium-priority improvements}

   ### Long-term Considerations (P3 - Low)

   {List of nice-to-have improvements}

   ---

   ## Positive Observations

   {Highlight good practices, well-designed code, and exemplary patterns found}

   ---

   ## Next Steps

   1. Review this report with the development team
   2. Prioritize fixes based on severity and effort
   3. Use the improvement checklist: `checklists/improvements.md`
   4. Track progress using the validation checklist: `checklists/validation.md`
   5. Consider running focused reviews on high-complexity areas

   ---

   ## Appendices

   - [Architecture Findings](findings/architecture.md)
   - [Design Pattern Findings](findings/design-patterns.md)
   - [Code Quality Findings](findings/code-quality.md)
   - [Improvement Suggestions](findings/suggestions.md)
   - [Improvement Checklist](checklists/improvements.md)
   - [Validation Checklist](checklists/validation.md)
   ```

7. **Create Improvement Checklist** (`checklists/improvements.md`):

   ```markdown
   # Code Improvement Checklist: {Review ID}

   **Purpose**: Track progress on addressing code review findings
   **Created**: {DATE}
   **Review Report**: [Link to report.md]

   ---

   ## Critical Issues (P0) - Fix Immediately

   {For each critical finding:}

   ### {Finding Title}

   - [ ] **Issue**: {Brief description}
   - [ ] **File**: {file_path}:{line_number}
   - [ ] **Action**: {Specific fix needed}
   - [ ] **Estimated Effort**: {Low/Medium/High}
   - [ ] **Assigned To**: _________
   - [ ] **Completed**: _________

   ---

   ## High Priority Issues (P1) - Address in Next Sprint

   {For each high-priority finding:}

   ### {Finding Title}

   - [ ] **Issue**: {Brief description}
   - [ ] **File**: {file_path}:{line_number}
   - [ ] **Action**: {Specific fix needed}
   - [ ] **Estimated Effort**: {Low/Medium/High}
   - [ ] **Assigned To**: _________
   - [ ] **Completed**: _________

   ---

   ## Medium Priority Issues (P2) - Plan for Future Sprint

   {List medium-priority items with checkbox format}

   ---

   ## Low Priority Issues (P3) - Nice to Have

   {List low-priority items with checkbox format}

   ---

   ## Progress Tracking

   | Priority | Total | Completed | In Progress | Not Started |
   |----------|-------|-----------|-------------|-------------|
   | P0       | {n}   | {n}       | {n}         | {n}         |
   | P1       | {n}   | {n}       | {n}         | {n}         |
   | P2       | {n}   | {n}       | {n}         | {n}         |
   | P3       | {n}   | {n}       | {n}         | {n}         |

   ---

   ## Notes

   - Mark items as completed when the fix is implemented and tested
   - Use validation checklist to verify fixes don't introduce regressions
   - Update this checklist as priorities change or new issues are discovered
   ```

8. **Create Validation Checklist** (`checklists/validation.md`):

   ```markdown
   # Validation Checklist: {Review ID}

   **Purpose**: Ensure fixes don't introduce regressions and meet quality standards
   **Created**: {DATE}

   ---

   ## Pre-Fix Validation

   Before implementing any fix from the improvement checklist:

   - [ ] Understand the root cause of the issue
   - [ ] Review the recommended solution
   - [ ] Consider impact on other parts of the codebase
   - [ ] Check if similar issues exist elsewhere
   - [ ] Plan the refactoring approach

   ---

   ## Post-Fix Validation

   After implementing each fix:

   ### Code Quality

   - [ ] Function length remains under 150 lines
   - [ ] Parameter count remains under 7
   - [ ] No new code duplication introduced
   - [ ] Naming follows project conventions
   - [ ] Comments added where logic is non-obvious

   ### Architecture

   - [ ] Layer separation maintained
   - [ ] Dependencies point in correct direction
   - [ ] No new circular dependencies introduced
   - [ ] Interface contracts preserved

   ### Testing

   - [ ] Existing tests still pass
   - [ ] New tests added for changes (if applicable)
   - [ ] Edge cases covered
   - [ ] Error handling tested

   ### Build and Integration

   - [ ] Code compiles without warnings
   - [ ] All tests pass (`cargo test` or `npm test`)
   - [ ] No new linter errors
   - [ ] Type checking passes (TypeScript)

   ### Language-Specific Checks

   **Rust**:
   - [ ] No new clippy warnings
   - [ ] Error handling uses Result<T, E> appropriately
   - [ ] Ownership and borrowing are optimal
   - [ ] Unsafe code is avoided or justified

   **TypeScript**:
   - [ ] No `any` types introduced
   - [ ] Strict mode checks pass
   - [ ] React hooks dependencies correct (if applicable)
   - [ ] Promises properly handled

   ---

   ## Final Review

   Before closing the improvement task:

   - [ ] Code reviewed by another developer
   - [ ] Documentation updated if APIs changed
   - [ ] CHANGELOG or commit message describes changes
   - [ ] No unintended side effects observed
   - [ ] Performance impact assessed (if relevant)

   ---

   ## Regression Prevention

   - [ ] Similar patterns checked across codebase
   - [ ] Lessons learned documented
   - [ ] Consider adding linter rules to prevent recurrence
   - [ ] Update coding guidelines if needed
   ```

9. **Calculate Health Score**:

   Score calculation formula:

   ```
   Health Score = Base Score - Deductions

   Base Score: 100

   Deductions:
   - Critical Issue: -5 points each
   - High Priority Issue: -2 points each
   - Medium Priority Issue: -1 point each
   - Low Priority Issue: -0.5 points each
   - Function over 150 lines: -1 point each
   - Function with 7+ parameters: -1 point each
   - Missing builder pattern (4+ params): -1 point each

   Minimum Score: 0 (floor)
   ```

   Category Scores (each out of 25):

   - **Architecture**: Based on layer separation, interface design, extensibility
   - **Code Quality**: Based on DRY, YAGNI, SOLID violations
   - **Best Practices**: Based on language-specific patterns, naming, error handling
   - **Maintainability**: Based on function length, parameter count, complexity

10. **Finalize and Report**:

    a. Write all findings to respective markdown files

    b. Generate the main report with executive summary

    c. Create both checklists (improvements and validation)

    d. Calculate and display health score

    e. Present to user:
       ```
       ✅ Code Review Complete: {Review ID}

       📊 Overall Health Score: {X}/100 ({Rating})

       📁 Review Location: .claude/reviews/{review-id}/

       🔍 Findings Summary:
       - Critical: {count}
       - High: {count}
       - Medium: {count}
       - Low: {count}

       📄 Reports Generated:
       - Main Report: report.md
       - Architecture Findings: findings/architecture.md
       - Design Patterns: findings/design-patterns.md
       - Code Quality: findings/code-quality.md
       - Improvement Checklist: checklists/improvements.md
       - Validation Checklist: checklists/validation.md

       🎯 Top 3 Priorities:
       1. {Finding 1}
       2. {Finding 2}
       3. {Finding 3}

       📋 Next Steps:
       - Review the detailed report at .claude/reviews/{review-id}/report.md
       - Use the improvement checklist to track fixes
       - Run validation checklist after implementing changes
       - Consider running "/code-review" again after fixes to verify improvements
       ```

---

## General Guidelines

### Review Philosophy

- **Constructive, Not Critical**: Focus on improvements, not blame
- **Actionable**: Every finding must have a clear fix or recommendation
- **Balanced**: Highlight both issues and good practices
- **Context-Aware**: Consider project phase (prototype vs production)
- **Pragmatic**: Prioritize issues by impact, not perfection

### Severity Guidelines

**Critical (P0)** - Must fix before production:
- Security vulnerabilities
- Data loss risks
- Severe performance bottlenecks
- Broken core functionality
- Major architectural flaws preventing extensibility

**High (P1)** - Should fix in next sprint:
- Significant SOLID violations
- Major code duplication (3+ occurrences)
- Functions significantly over 150 lines
- Missing error handling in critical paths
- Poor interface design affecting multiple components

**Medium (P2)** - Address in near future:
- Minor SOLID violations
- Small code duplication (2 occurrences)
- Functions slightly over 150 lines
- Missing builder patterns
- Suboptimal naming conventions

**Low (P3)** - Nice to have:
- Code style inconsistencies
- Missing comments on complex logic
- Opportunities for minor refactoring
- Documentation improvements

### Builder Pattern Detection (Rust)

Recommend builder pattern when:
1. Struct has 4+ fields
2. Constructor has 5+ parameters
3. Some parameters are optional
4. Complex initialization logic exists
5. Configuration-style structs

Example of when builder is needed:
```rust
// BAD: Too many parameters
pub fn new(
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
    max_connections: u32,
    timeout: Duration,
) -> Self { ... }

// GOOD: Use builder pattern
DatabaseConfig::builder()
    .host("localhost")
    .port(5432)
    .credentials("user", "pass")
    .database("mydb")
    .max_connections(10)
    .timeout(Duration::from_secs(30))
    .build()
```

### Function Length Analysis

When flagging functions over 150 lines:
1. Count actual logic lines (exclude comments, blank lines, closing braces)
2. Suggest logical breakpoints for extraction
3. Propose helper function names based on what each section does
4. Consider single responsibility: is the function doing multiple things?

Example suggestion format:
```
Function `process_request` is 245 lines (95 lines over limit).

Suggested refactoring:
- Extract lines 20-60 into `validate_input(&self, request: &Request) -> Result<(), Error>`
- Extract lines 75-130 into `fetch_data(&self, params: &Params) -> Result<Data, Error>`
- Extract lines 140-200 into `transform_response(&self, data: Data) -> Response`

This would reduce main function to ~50 lines and improve testability.
```

### PowerShell Command Usage

All shell commands in the review process should use PowerShell syntax:

```powershell
# Create directories
New-Item -ItemType Directory -Path ".claude/reviews/$reviewId" -Force

# Check if file exists
Test-Path "path/to/file"

# Get file stats
Get-ChildItem "path" -Recurse -File | Measure-Object -Property Length -Sum

# Count lines in file
(Get-Content "file.rs").Count

# Find files
Get-ChildItem -Path "backend/src" -Filter "*.rs" -Recurse

# Search content
Select-String -Path "*.rs" -Pattern "pattern"
```

### Review Best Practices

1. **Start Broad, Then Deep**:
   - First pass: Architecture and design issues
   - Second pass: Code quality and principles
   - Third pass: Line-by-line details

2. **Provide Examples**:
   - Always show current code vs improved code
   - Explain WHY the change matters
   - Reference specific principles (DRY, SOLID, etc.)

3. **Consider Context**:
   - Prototype code needs different standards than production
   - Performance-critical code may sacrifice some readability
   - Legacy code improvements should be incremental

4. **Use Project Conventions**:
   - Reference project's CLAUDE.md for specific conventions
   - Follow established patterns in the codebase
   - Don't enforce personal style preferences

5. **Measure Impact**:
   - High-impact issues first (security, correctness, performance)
   - Then structural issues (architecture, design)
   - Finally style and minor improvements

### False Positive Prevention

Be careful not to flag:
- Generated code (proto files, bindings, etc.)
- Third-party code or vendored dependencies
- Test fixtures or mock data
- Intentional complexity (algorithms, parsers, state machines)
- Domain-driven complexity that mirrors business logic

Always verify context before marking something as an issue.

---

## Notes for AI Execution

- Use `Read` tool for all file content access
- Use `Glob` tool for finding files (don't use bash find)
- Use `Grep` tool for searching patterns across files
- Calculate health scores numerically based on finding counts
- Be objective: flag real issues, acknowledge good code
- Provide specific line numbers for all findings
- Generate actionable recommendations with code examples
- Consider the project's specific conventions from CLAUDE.md
- If review scope is too large (20+ files), warn user and suggest focused reviews
- Track progress by updating checklists as findings are addressed
