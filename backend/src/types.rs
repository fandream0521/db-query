// Shared type definitions

use crate::error::AppError;
use crate::services::database_service::DatabaseService;
use crate::services::schema_service::SchemaService;
use crate::services::llm_service::LLMService;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

// Shared service types for Axum state
pub type SharedDatabaseService = Arc<DatabaseService>;
pub type SharedSchemaService = Arc<SchemaService>;
pub type SharedLLMService = Arc<LLMService>;

/// Connection pool cache for reusing PostgreSQL connection pools
pub struct ConnectionPoolCache {
    pools: Arc<RwLock<HashMap<String, Arc<PgPool>>>>,
}

impl ConnectionPoolCache {
    pub fn new() -> Self {
        Self {
            pools: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get an existing pool or create a new one
    pub async fn get_or_create(&self, name: &str, url: &str) -> Result<Arc<PgPool>, AppError> {
        // Try read lock first (fast path)
        {
            let pools = self.pools.read().await;
            if let Some(pool) = pools.get(name) {
                tracing::debug!(database_name = %name, "reusing existing connection pool");
                return Ok(Arc::clone(pool));
            }
        }

        // Need to create new pool (slow path)
        let mut pools = self.pools.write().await;

        // Double-check after acquiring write lock (another task might have created it)
        if let Some(pool) = pools.get(name) {
            tracing::debug!(database_name = %name, "reusing connection pool (after write lock)");
            return Ok(Arc::clone(pool));
        }

        // Create new pool with configuration
        tracing::info!(database_name = %name, "creating new connection pool");
        let pool_options = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5) // Limit per database
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600)) // 10 minutes
            .max_lifetime(Duration::from_secs(3600)); // 1 hour

        let pool = Arc::new(pool_options.connect(url).await?);
        pools.insert(name.to_string(), Arc::clone(&pool));

        tracing::info!(database_name = %name, "connection pool created and cached");
        Ok(pool)
    }

    /// Remove a pool from the cache (e.g., when database is deleted)
    pub async fn remove(&self, name: &str) {
        let mut pools = self.pools.write().await;
        if let Some(pool) = pools.remove(name) {
            tracing::info!(database_name = %name, "removing connection pool from cache");
            pool.close().await;
        }
    }

    /// Get the number of cached pools (for debugging/monitoring)
    #[allow(dead_code)]
    pub async fn len(&self) -> usize {
        let pools = self.pools.read().await;
        pools.len()
    }
}

impl Default for ConnectionPoolCache {
    fn default() -> Self {
        Self::new()
    }
}

pub type SharedConnectionPoolCache = Arc<ConnectionPoolCache>;

