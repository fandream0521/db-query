# Validation Checklist: review-20251221-120916-backend

**Purpose**: Ensure fixes don't introduce regressions and meet quality standards
**Created**: 2025-12-21
**Review Report**: [../report.md](../report.md)

---

## Pre-Fix Validation

Before implementing any fix from the improvement checklist:

- [ ] **Understand the root cause** of the issue
- [ ] **Review the recommended solution** in the detailed findings
- [ ] **Consider impact** on other parts of the codebase
- [ ] **Check if similar issues exist elsewhere** that should be fixed together
- [ ] **Plan the refactoring approach** - write down steps before coding
- [ ] **Create a feature branch** for the fix (e.g., `fix/connection-pool-caching`)
- [ ] **Read related code** to understand context and dependencies

---

## Post-Fix Validation

After implementing each fix, verify the following:

### Code Quality

- [ ] **Function length** remains under 150 lines (or significantly reduced if that was the issue)
- [ ] **Parameter count** remains under 7
- [ ] **No new code duplication** introduced
- [ ] **Naming follows project conventions**:
  - snake_case for functions and variables
  - PascalCase for types and traits
  - SCREAMING_SNAKE_CASE for constants
- [ ] **Comments added** where logic is non-obvious (but prefer self-documenting code)
- [ ] **No #[allow(dead_code)]** or #[allow(unused)] added without good reason
- [ ] **Proper error handling** - no unwrap() in production code paths
- [ ] **Logging is appropriate** - not too verbose, uses structured fields

### Architecture

- [ ] **Layer separation maintained** (API → Service → Data)
- [ ] **Dependencies point in correct direction** (high-level doesn't depend on low-level details)
- [ ] **No new circular dependencies** introduced
- [ ] **Interface contracts preserved** - public APIs still work the same way
- [ ] **Shared state pattern followed** - services passed via Arc in Axum state
- [ ] **No tight coupling** introduced between modules

### Testing

- [ ] **Existing unit tests still pass** (`cargo test`)
- [ ] **New tests added** for the changes (if applicable)
  - [ ] Happy path tested
  - [ ] Error paths tested
  - [ ] Edge cases covered
- [ ] **Test coverage** hasn't decreased
- [ ] **Integration points tested** (if changing service interfaces)
- [ ] **Concurrency tested** (if changing shared state or locks)

### Build and Integration

- [ ] **Code compiles** without warnings (`cargo build`)
- [ ] **All tests pass** (`cargo test`)
- [ ] **No new clippy warnings** (`cargo clippy`)
  - If you added clippy config, all existing code passes
- [ ] **Code formatted** properly (`cargo fmt`)
- [ ] **Type checking passes** (already verified by cargo build)
- [ ] **Documentation builds** (`cargo doc --no-deps`)

### Rust-Specific Checks

- [ ] **No new clippy warnings** introduced
- [ ] **Error handling uses Result<T, E>** appropriately (not panic)
- [ ] **Ownership and borrowing are optimal**:
  - [ ] No unnecessary clones
  - [ ] Borrows used instead of moves where possible
  - [ ] No lifetime issues
- [ ] **Unsafe code is avoided** (or justified and documented if necessary)
- [ ] **Async code is correct**:
  - [ ] No blocking calls in async functions
  - [ ] Futures are properly awaited
  - [ ] No unhandled task panics
- [ ] **Mutex usage is correct**:
  - [ ] Locks not held across await points
  - [ ] Deadlock potential considered
  - [ ] Poisoning handled appropriately

### Performance (if applicable)

- [ ] **No performance regression** introduced
- [ ] **Connection pools reused** (not created per-request)
- [ ] **Database queries optimized** (use indexes, avoid N+1)
- [ ] **String allocations minimized** in hot paths
- [ ] **No unnecessary clones or copies**

### Security (if applicable)

- [ ] **Input validation** present for user-controlled data
- [ ] **SQL injection** prevented (parameterized queries or validation)
- [ ] **Passwords/secrets not logged** (masked or omitted)
- [ ] **No sensitive data in error messages** that go to clients
- [ ] **CORS configuration** still appropriate

---

## Fix-Specific Validation

### For Connection Pool Caching (#1)

After implementing ConnectionPoolCache:

- [ ] **Pools are reused** across requests to the same database
- [ ] **Pool eviction works** when database is deleted
- [ ] **Concurrent access is safe** (RwLock used correctly)
- [ ] **Memory leaks prevented** (pools cleaned up when no longer needed)
- [ ] **Connection limits respected** (max_connections configured)
- [ ] **Performance improved** - measure before/after with load test
- [ ] **No connection leaks** under high load

### For Function Refactoring (#2, #3)

After extracting helper functions:

- [ ] **Main function is clear and concise** (< 20 lines preferred)
- [ ] **Helper functions are reusable** (not too specific)
- [ ] **Each helper has single responsibility**
- [ ] **Function names are descriptive** (verb-noun pattern)
- [ ] **Parameters are minimal** (extract into structs if > 4)
- [ ] **Original behavior preserved** (same inputs produce same outputs)
- [ ] **Error handling unchanged** (same error types returned)

### For SQL Injection Fix (#4)

After fixing SQL injection risk:

- [ ] **Table names validated** or parameterized
- [ ] **quote_ident used** (or equivalent validation)
- [ ] **Injection attempt rejected** (test with malicious input like `"; DROP TABLE users--`)
- [ ] **Legitimate names still work** (test with normal names, names with underscores, etc.)
- [ ] **Error messages informative** but don't reveal SQL structure

### For Configuration Changes (#5, #7, #13)

After changing Config or adding builder:

- [ ] **All config values accessible** where needed
- [ ] **Defaults are sensible** and documented
- [ ] **Environment variables still work** (backward compatible)
- [ ] **Builder API is ergonomic** (fluent interface)
- [ ] **Tests can create test configs easily**
- [ ] **No config fields marked #[allow(dead_code)]**

### For Error Handling Changes (#8, #9)

After improving error handling:

- [ ] **No unwrap() or expect()** in production paths (test-only is OK)
- [ ] **TryFrom used instead of From** where conversion can fail
- [ ] **Mutex poisoning handled** gracefully
- [ ] **Error messages are helpful** to developers and users
- [ ] **Error types consistent** with existing AppError patterns

---

## Final Review

Before closing the improvement task and marking it as complete:

- [ ] **Code reviewed by another developer** (or self-review after 24 hours)
- [ ] **Documentation updated** if APIs changed:
  - [ ] Function docs updated
  - [ ] Module docs updated
  - [ ] CLAUDE.md updated if conventions changed
- [ ] **CHANGELOG or commit message** describes changes clearly
- [ ] **No unintended side effects** observed in manual testing
- [ ] **Performance impact assessed** (if relevant):
  - [ ] Load testing done for performance fixes
  - [ ] Memory usage checked for caching changes
  - [ ] Benchmarks run if available
- [ ] **Database migrations considered** (if schema changed)
- [ ] **API compatibility maintained** (for library code)

---

## Regression Prevention

After completing the fix:

- [ ] **Similar patterns checked** across codebase
  - If you fixed a mistake in one place, check if it exists elsewhere
  - If you improved a pattern, apply it consistently
- [ ] **Lessons learned documented**
  - Add to project's coding guidelines if applicable
  - Share with team in code review or standup
- [ ] **Consider adding linter rules** to prevent recurrence
  - clippy lints for Rust patterns
  - Custom lints if needed
- [ ] **Update coding guidelines** if needed (CLAUDE.md)
  - Document new patterns or conventions
  - Add examples of good/bad code

---

## Testing Strategy

### Unit Testing

- [ ] **Test the specific fix** with dedicated unit tests
- [ ] **Test error cases** that were previously untested
- [ ] **Test edge cases**:
  - Empty inputs
  - Null/None values
  - Boundary conditions
  - Concurrent access (if applicable)

### Integration Testing

- [ ] **Test end-to-end flows** that use the fixed code
- [ ] **Test API endpoints** if handler code changed
- [ ] **Test database interactions** if service code changed
- [ ] **Test error propagation** through the layers

### Manual Testing

- [ ] **Run the application** locally and test the affected feature
- [ ] **Test with real database** (not just mocks)
- [ ] **Test error scenarios**:
  - Network failures
  - Database connection errors
  - Invalid input
  - Timeouts
- [ ] **Test under load** (if performance-related fix):
  - Use wrk, ab, or similar tool
  - Verify connection pooling works
  - Check for resource leaks

---

## Sign-Off

### Developer Sign-Off

- [ ] I have completed all applicable items in this checklist
- [ ] I have tested the changes thoroughly
- [ ] I have reviewed my own code for quality
- [ ] I understand the impact of my changes

**Developer**: _________
**Date**: _________

### Reviewer Sign-Off (if applicable)

- [ ] I have reviewed the changes
- [ ] I have verified the checklist items were completed
- [ ] The changes meet quality standards
- [ ] I approve these changes for merge

**Reviewer**: _________
**Date**: _________

---

## Rollback Plan

If issues are discovered after deployment:

1. **Identify the issue** - what's broken, what's the impact?
2. **Check if quick fix possible** - can it be fixed forward in < 30 minutes?
3. **If not, revert the changes**:
   ```bash
   git revert <commit-hash>
   git push
   ```
4. **Investigate root cause** in a non-production environment
5. **Create new fix** with proper testing
6. **Re-deploy when confident**

**Always prefer rolling forward** (quick fix) over rollback if possible, but don't hesitate to rollback if impact is high.

---

## Health Score Validation

After completing a set of fixes, re-run the code review:

```bash
claude /code-review ./backend
```

**Expected score improvements**:
- After P1 fixes: ~80/100 (+6 points)
- After P1 + P2 fixes: ~85/100 (+11 points)
- After all fixes: ~90+/100 (+16+ points)

If score doesn't improve as expected:
- [ ] Review what was actually fixed vs. what was planned
- [ ] Check if new issues were introduced
- [ ] Verify the fixes were implemented correctly
- [ ] Consider if priorities need adjustment

---

## Notes

Use this checklist as a living document:
- Add items as you discover new validation needs
- Remove items that don't apply to your project
- Customize for your team's workflow and standards
- Keep it realistic - don't make it so long that it's ignored

**The goal is quality, not bureaucracy.** Use your judgment on which items apply to each fix.
