# Code Improvement Checklist: review-20251221-120916-backend

**Purpose**: Track progress on addressing code review findings
**Created**: 2025-12-21
**Review Report**: [report.md](../report.md)

---

## Critical Issues (P0) - Fix Immediately

**No critical issues found!** The codebase is in good shape overall.

---

## High Priority Issues (P1) - Address in Next Sprint

### 1. Per-Request Connection Pool Creation

- [ ] **Issue**: Creating new PostgreSQL connection pools on every request
- [ ] **File**: backend/src/api/queries.rs:35, 79
- [ ] **Action**: Implement ConnectionPoolCache with Arc<RwLock<HashMap<String, Arc<PgPool>>>>
- [ ] **Estimated Effort**: Medium (4-6 hours)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________
- [ ] **Notes**: This is the highest impact fix - will dramatically improve performance

### 2. Refactor test_connection Function (111 lines)

- [ ] **Issue**: Function too long, violates Single Responsibility Principle
- [ ] **File**: backend/src/services/database_service.rs:21-132
- [ ] **Action**: Extract helper functions: validate_connection_url, is_postgres_url, test_postgres_connection, create_pg_pool_with_timeout, execute_test_query
- [ ] **Estimated Effort**: Medium (3-4 hours)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 3. Refactor store_connection Function (100 lines)

- [ ] **Issue**: Function too long, combines validation, upsert, and retrieval
- [ ] **File**: backend/src/services/database_service.rs:135-235
- [ ] **Action**: Extract validate_connection_input and upsert_connection, reuse get_connection
- [ ] **Estimated Effort**: Low (1-2 hours)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 4. Fix SQL Injection Risk in Schema Service

- [ ] **Issue**: String formatting used for table names in SQL queries
- [ ] **File**: backend/src/services/schema_service.rs:107
- [ ] **Action**: Use PostgreSQL's quote_ident function or add identifier validation
- [ ] **Estimated Effort**: Low (1 hour)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________
- [ ] **Notes**: Security issue - should be fixed even though current risk is low

### 5. Consolidate LLM Configuration

- [ ] **Issue**: LLMService created in handler, violates Dependency Inversion Principle
- [ ] **File**: backend/src/api/queries.rs:64-67
- [ ] **Action**: Create LLMService in main.rs from Config, add to shared state
- [ ] **Estimated Effort**: Low (1-2 hours)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 6. Move Type Aliases to Proper Module

- [ ] **Issue**: SharedDatabaseService and SharedSchemaService defined in API module
- [ ] **File**: backend/src/api/databases.rs:14-15
- [ ] **Action**: Move type aliases to types.rs
- [ ] **Estimated Effort**: Low (30 minutes)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

---

## Medium Priority Issues (P2) - Plan for Next 2-3 Sprints

### 7. Add Builder Pattern for Config

- [ ] **Issue**: Config struct has 5 fields but no builder pattern
- [ ] **File**: backend/src/config.rs:4-13
- [ ] **Action**: Implement ConfigBuilder with sensible defaults
- [ ] **Estimated Effort**: Medium (2-3 hours)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 8. Implement TryFrom for DatabaseConnection

- [ ] **Issue**: From<&Row> implementation uses unwrap() which can panic
- [ ] **File**: backend/src/models/database.rs:31-40
- [ ] **Action**: Replace with TryFrom<&Row> implementation
- [ ] **Estimated Effort**: Low (30 minutes)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 9. Handle Mutex Poisoning Properly

- [ ] **Issue**: .lock().unwrap() will panic if mutex is poisoned
- [ ] **File**: Multiple locations (database_service.rs, schema_service.rs)
- [ ] **Action**: Use .map_err() or switch to parking_lot::Mutex
- [ ] **Estimated Effort**: Low (1 hour for all locations)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 10. Duplicate Connection Creation in SchemaService

- [ ] **Issue**: retrieve_from_database creates its own connection
- [ ] **File**: backend/src/services/schema_service.rs:40, 57
- [ ] **Action**: Pass connection pool as parameter or use ConnectionPoolCache
- [ ] **Estimated Effort**: Low (1 hour, depends on #1)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 11. Configure Connection Pool Options

- [ ] **Issue**: PgPool uses default settings (no limits, timeouts, etc.)
- [ ] **File**: backend/src/api/queries.rs:35, 79
- [ ] **Action**: Use PgPoolOptions to configure max_connections, timeouts, lifetimes
- [ ] **Estimated Effort**: Low (30 minutes, combine with #1)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 12. Create DatabaseUrl Newtype with Auto-Masking

- [ ] **Issue**: Database URLs with passwords could be logged without masking
- [ ] **File**: backend/src/models/database.rs
- [ ] **Action**: Create DatabaseUrl wrapper with Display impl that masks passwords
- [ ] **Estimated Effort**: Medium (2 hours)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 13. Remove Unused Code

- [ ] **Issue**: Timestamp type never used, Config fields marked #[allow(dead_code)]
- [ ] **File**: backend/src/types.rs:6-34, backend/src/config.rs:5-11
- [ ] **Action**: Delete Timestamp type, remove database_url field, fix LLM field usage
- [ ] **Estimated Effort**: Low (30 minutes, combine with #5)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 14. Reduce Logging Verbosity

- [ ] **Issue**: Excessive debug logging makes code hard to read
- [ ] **File**: backend/src/services/database_service.rs (multiple locations)
- [ ] **Action**: Use structured logging with fields, remove step-by-step logs
- [ ] **Estimated Effort**: Low (1-2 hours)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 15. Use From Trait in get_connection

- [ ] **Issue**: Manual row extraction duplicates From<&Row> implementation
- [ ] **File**: backend/src/services/database_service.rs:265-313
- [ ] **Action**: Use DatabaseConnection::from(row) consistently
- [ ] **Estimated Effort**: Low (15 minutes)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

---

## Low Priority Issues (P3) - Nice to Have

### 16. Extract URL Masking Utility

- [ ] **Issue**: URL masking logic is inline and not reusable
- [ ] **File**: backend/src/services/database_service.rs:42-50
- [ ] **Action**: Create mask_database_url() function in utils/validation.rs
- [ ] **Estimated Effort**: Low (30 minutes)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 17. Add Integration Tests

- [ ] **Issue**: No integration tests for API handlers
- [ ] **File**: tests/ (missing)
- [ ] **Action**: Add integration tests using axum::test helpers
- [ ] **Estimated Effort**: High (8-16 hours)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 18. Add Clippy Configuration

- [ ] **Issue**: No clippy lints configured for automatic quality checks
- [ ] **File**: .cargo/config.toml (missing)
- [ ] **Action**: Add clippy lint configuration
- [ ] **Estimated Effort**: Low (30 minutes)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 19. Add Module Documentation

- [ ] **Issue**: Module files have placeholder comments instead of proper docs
- [ ] **File**: All mod.rs files
- [ ] **Action**: Replace placeholder comments with proper module-level documentation
- [ ] **Estimated Effort**: Low (1-2 hours)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 20. Add Type Safety for Database Names

- [ ] **Issue**: Database names are stringly-typed
- [ ] **File**: Multiple locations
- [ ] **Action**: Create DatabaseName newtype with validation
- [ ] **Estimated Effort**: Medium (3-4 hours for widespread changes)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 21. Optimize Logging String Allocations

- [ ] **Issue**: String formatting in logs happens even when level is disabled
- [ ] **File**: Multiple locations
- [ ] **Action**: Use structured logging fields instead of string formatting
- [ ] **Estimated Effort**: Low (1 hour)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 22. Add Error Path Testing

- [ ] **Issue**: Tests only cover happy paths, not error scenarios
- [ ] **File**: backend/src/services/database_service.rs:359-478
- [ ] **Action**: Add tests for timeouts, network failures, concurrent access
- [ ] **Estimated Effort**: Medium (4-6 hours)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

---

## Progress Tracking

| Priority | Total | Completed | In Progress | Not Started |
|----------|-------|-----------|-------------|-------------|
| P0       | 0     | 0         | 0           | 0           |
| P1       | 6     | 0         | 0           | 6           |
| P2       | 9     | 0         | 0           | 9           |
| P3       | 7     | 0         | 0           | 7           |
| **Total** | **22** | **0**     | **0**       | **22**     |

### Completion Percentage: 0%

---

## Sprint Planning Suggestion

### Sprint 1 (Focus: Performance & Security)
- [ ] #1: Connection pool caching (HIGH IMPACT)
- [ ] #4: SQL injection fix (SECURITY)
- [ ] #5: LLM configuration consolidation
- [ ] #6: Move type aliases

**Expected Outcome**: Health score increase to ~80/100, major performance improvement

### Sprint 2 (Focus: Maintainability)
- [ ] #2: Refactor test_connection
- [ ] #3: Refactor store_connection
- [ ] #8: TryFrom implementation
- [ ] #9: Mutex poisoning handling
- [ ] #15: Use From trait consistently

**Expected Outcome**: Health score increase to ~85/100, better code maintainability

### Sprint 3 (Focus: Code Quality)
- [ ] #7: Builder pattern for Config
- [ ] #10: Schema service connection duplication
- [ ] #11: Connection pool options
- [ ] #13: Remove unused code
- [ ] #14: Reduce logging verbosity

**Expected Outcome**: Health score increase to ~90+/100, cleaner codebase

---

## Notes

- Mark items as completed when the fix is implemented and tested
- Use validation checklist to verify fixes don't introduce regressions
- Update this checklist as priorities change or new issues are discovered
- Consider running `/code-review` again after each sprint to track progress
- Some items (#1, #5, #13) can be combined as they're related

---

## Quick Wins (Can be done in < 1 hour)

For immediate satisfaction and quick progress:
1. #6: Move type aliases (30 min)
2. #8: TryFrom implementation (30 min)
3. #15: Use From trait in get_connection (15 min)
4. #16: Extract URL masking utility (30 min)
5. #18: Add clippy configuration (30 min)

**Total: ~2.5 hours for 5 improvements**
