# Architecture and Design Findings

**Review ID**: review-20251221-120916-backend
**Date**: 2025-12-21
**Category**: Architecture & Design

---

## Summary

- **Total Issues**: 3
- **Critical**: 0
- **High**: 1
- **Medium**: 2
- **Low**: 0

---

## Findings

### Finding: Per-Request Database Connection Pool Creation

**File**: backend/src/api/queries.rs:35, 79
**Severity**: High
**Category**: Architecture
**Principle Violated**: Performance & Resource Management

**Issue**:
The application creates a new PostgreSQL connection pool for every single request in both `execute_query` and `execute_natural_language_query` handlers. Connection pools are expensive resources meant to be created once and reused.

**Current Code**:
```rust
// In execute_query
pub async fn execute_query(
    State((service, _)): State<(SharedDatabaseService, SharedSchemaService)>,
    Path(name): Path<String>,
    Json(request): Json<QueryRequest>,
) -> Result<Json<QueryResponse>, AppError> {
    // ... validation ...
    let connection = service.get_connection(&name)?;

    // Creates new pool on EVERY request!
    let pool = PgPool::connect(&connection.url).await?;

    let response = QueryExecutor::execute_query(&pool, &validated_sql).await?;
    Ok(Json(response))
}
```

**Why This Matters**:
- **Performance Impact**: Creating a new connection pool involves TCP handshakes, authentication, and pool initialization overhead on every request
- **Resource Waste**: Pools are never reused; each pool creates new database connections
- **Scalability Issue**: Under high load, this will create hundreds of database connections, exhausting PostgreSQL connection limits
- **Cost**: Database connection setup is one of the most expensive operations in database access

**Recommendation**:
Implement a connection pool cache that stores `Arc<PgPool>` instances keyed by database name. Reuse pools across requests for the same database.

**Improved Code**:
```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Add to shared state
pub struct ConnectionPoolCache {
    pools: Arc<RwLock<HashMap<String, Arc<PgPool>>>>,
}

impl ConnectionPoolCache {
    pub fn new() -> Self {
        Self {
            pools: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_or_create(&self, name: &str, url: &str) -> Result<Arc<PgPool>, AppError> {
        // Try read lock first (fast path)
        {
            let pools = self.pools.read().await;
            if let Some(pool) = pools.get(name) {
                return Ok(Arc::clone(pool));
            }
        }

        // Need to create new pool (slow path)
        let mut pools = self.pools.write().await;

        // Double-check after acquiring write lock
        if let Some(pool) = pools.get(name) {
            return Ok(Arc::clone(pool));
        }

        // Create and cache the pool
        let pool = Arc::new(PgPool::connect(url).await?);
        pools.insert(name.to_string(), Arc::clone(&pool));

        Ok(pool)
    }

    pub async fn remove(&self, name: &str) {
        let mut pools = self.pools.write().await;
        if let Some(pool) = pools.remove(name) {
            pool.close().await;
        }
    }
}

// Updated handler
pub async fn execute_query(
    State((service, _, pool_cache)): State<(SharedDatabaseService, SharedSchemaService, Arc<ConnectionPoolCache>)>,
    Path(name): Path<String>,
    Json(request): Json<QueryRequest>,
) -> Result<Json<QueryResponse>, AppError> {
    // ... validation ...
    let connection = service.get_connection(&name)?;

    // Reuse existing pool or create once
    let pool = pool_cache.get_or_create(&name, &connection.url).await?;

    let response = QueryExecutor::execute_query(&pool, &validated_sql).await?;
    Ok(Json(response))
}
```

**Effort**: Medium
**Priority**: P1 (High)

---

### Finding: Schema Service Creates Duplicate Database Connection

**File**: backend/src/services/schema_service.rs:40, 57
**Severity**: Medium
**Category**: Architecture
**Principle Violated**: DRY (Don't Repeat Yourself)

**Issue**:
The `retrieve_from_database` method connects to the target PostgreSQL database separately, even though the application already manages database connections. This creates unnecessary code coupling and duplicate connection logic.

**Current Code**:
```rust
async fn retrieve_from_database(&self, url: &str, db_name: &str) -> Result<SchemaMetadata, AppError> {
    // Validates URL again (already validated in DatabaseService)
    if !url.starts_with("postgres://") && !url.starts_with("postgresql://") {
        return Err(AppError::ValidationError(
            "Only PostgreSQL databases are supported".to_string(),
        ));
    }

    // Creates its own connection pool
    let pool = PgPool::connect(url).await?;
    // ...
}
```

**Why This Matters**:
- Duplicates database connection management
- Inconsistent with the overall architecture where services should coordinate
- Makes pool caching solution harder to implement
- URL validation duplicated from DatabaseService

**Recommendation**:
Pass the connection pool as a parameter or use the proposed ConnectionPoolCache. Remove duplicate validation.

**Improved Code**:
```rust
// With pool cache
async fn retrieve_from_database(
    &self,
    pool: &PgPool,  // Accept pool instead of creating
    db_name: &str
) -> Result<SchemaMetadata, AppError> {
    // No need for URL validation or pool creation
    let table_names: Vec<String> = sqlx::query_scalar(
        "SELECT table_name FROM information_schema.tables
         WHERE table_schema = 'public' AND table_type = 'BASE TABLE'
         ORDER BY table_name"
    )
    .fetch_all(pool)
    .await?;
    // ... rest of implementation
}
```

**Effort**: Low
**Priority**: P2 (Medium)

---

### Finding: Shared State Type Aliases in API Module

**File**: backend/src/api/databases.rs:14-15
**Severity**: Medium
**Category**: Architecture
**Principle Violated**: Module Organization

**Issue**:
Shared state type aliases are defined in the API module, but they're used across the application (imported in main.rs). Type definitions that are used application-wide should be in a more central location.

**Current Code**:
```rust
// In api/databases.rs
pub type SharedDatabaseService = Arc<DatabaseService>;
pub type SharedSchemaService = Arc<SchemaService>;

// In main.rs
use api::databases::{SharedDatabaseService, SharedSchemaService, list_databases, ...};
```

**Why This Matters**:
- Type aliases are coupling unrelated concerns (API handlers shouldn't define app-wide types)
- Violates separation of concerns
- Makes imports confusing (importing types from a handler module)
- Would be better in a dedicated types module or at service level

**Recommendation**:
Move shared state type aliases to either `types.rs` or define them alongside the service definitions.

**Improved Code**:
```rust
// In src/types.rs
use crate::services::database_service::DatabaseService;
use crate::services::schema_service::SchemaService;
use std::sync::Arc;

pub type SharedDatabaseService = Arc<DatabaseService>;
pub type SharedSchemaService = Arc<SchemaService>;

// In main.rs
use crate::types::{SharedDatabaseService, SharedSchemaService};
use crate::api::databases::{list_databases, get_database_metadata, ...};
```

**Effort**: Low
**Priority**: P2 (Medium)

---

## Positive Observations

1. **Clean Layer Separation**: The API → Service → Data layer architecture is well-implemented with proper dependency direction
2. **Proper Use of Arc**: Services are correctly wrapped in Arc for thread-safe sharing
3. **Good Error Handling**: Custom AppError enum provides structured error responses
4. **Axum State Pattern**: Correctly uses Axum's State extractor for dependency injection

---
