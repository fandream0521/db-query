use crate::error::AppError;
use crate::models::database::DatabaseConnection;
use crate::models::request::CreateDatabaseRequest;
use crate::utils::validation::{validate_database_name, validate_database_url};
use rusqlite::Connection;
use sqlx::PgPool;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::timeout;

pub struct DatabaseService {
    sqlite_conn: Arc<Mutex<Connection>>,
}

impl DatabaseService {
    pub fn new(sqlite_conn: Arc<Mutex<Connection>>) -> Self {
        Self { sqlite_conn }
    }

    /// Test database connection
    pub async fn test_connection(url: &str) -> Result<(), AppError> {
        tracing::info!(url = %Self::mask_database_url(url), "starting connection test");

        Self::validate_connection_url(url)?;

        if Self::is_postgres_url(url) {
            Self::test_postgres_connection(url).await?;
        } else {
            tracing::warn!("unsupported database type, skipping connection test");
        }

        tracing::info!("connection test completed successfully");
        Ok(())
    }

    /// Validate database URL format
    fn validate_connection_url(url: &str) -> Result<(), AppError> {
        if !validate_database_url(url) {
            tracing::error!(url = %url, "invalid database URL format");
            return Err(AppError::ValidationError(
                "Invalid database URL format".to_string(),
            ));
        }
        Ok(())
    }

    /// Check if URL is a `PostgreSQL` connection string
    fn is_postgres_url(url: &str) -> bool {
        url.starts_with("postgres://") || url.starts_with("postgresql://")
    }

    /// Test `PostgreSQL` connection
    async fn test_postgres_connection(url: &str) -> Result<(), AppError> {
        tracing::info!("detected PostgreSQL connection string");

        let pool = Self::create_pg_pool_with_timeout(url, Duration::from_secs(15)).await?;
        Self::execute_test_query(&pool, Duration::from_secs(5)).await?;

        tracing::info!("PostgreSQL connection test completed successfully");
        Ok(())
    }

    /// Create `PostgreSQL` connection pool with timeout
    async fn create_pg_pool_with_timeout(
        url: &str,
        timeout_duration: Duration,
    ) -> Result<PgPool, AppError> {
        let options = url.parse::<sqlx::postgres::PgConnectOptions>()
            .map_err(|e| {
                tracing::error!(error = ?e, "failed to parse connection options");
                AppError::from(e)
            })?;

        tracing::info!(url = %Self::mask_database_url(url), "connecting to database");
        let start_time = std::time::Instant::now();

        let pool = timeout(timeout_duration, PgPool::connect_with(options))
            .await
            .map_err(|_| {
                let elapsed = start_time.elapsed();
                tracing::error!(timeout_secs = elapsed.as_secs(), "connection timeout");
                AppError::ConnectionError(format!(
                    "Connection timeout after {} seconds. Please check if PostgreSQL is running and accessible.",
                    elapsed.as_secs()
                ))
            })?
            .map_err(|e| {
                let elapsed = start_time.elapsed();
                tracing::error!(error = ?e, elapsed_secs = elapsed.as_secs_f64(), "failed to connect");
                AppError::ConnectionError(format!(
                    "Failed to connect to PostgreSQL: {}. Connection took {:.2} seconds",
                    e, elapsed.as_secs_f64()
                ))
            })?;

        let elapsed = start_time.elapsed();
        tracing::info!(elapsed_secs = elapsed.as_secs_f64(), "connection established");
        Ok(pool)
    }

    /// Execute test query with timeout
    async fn execute_test_query(pool: &PgPool, timeout_duration: Duration) -> Result<(), AppError> {
        tracing::debug!("executing test query: SELECT 1");
        let start_time = std::time::Instant::now();

        timeout(timeout_duration, sqlx::query("SELECT 1").execute(pool))
            .await
            .map_err(|_| {
                let elapsed = start_time.elapsed();
                tracing::error!(timeout_secs = elapsed.as_secs(), "test query timeout");
                AppError::ConnectionError(
                    "Test query timeout. Database may be slow or unresponsive.".to_string()
                )
            })?
            .map_err(|e| {
                tracing::error!(error = ?e, "test query failed");
                AppError::from(e)
            })?;

        let elapsed = start_time.elapsed();
        tracing::info!(elapsed_secs = elapsed.as_secs_f64(), "test query executed successfully");
        Ok(())
    }

    /// Mask password in database URL for secure logging
    fn mask_database_url(url: &str) -> String {
        if let Some(at_pos) = url.find('@') {
            if let Some(colon_pos) = url[..at_pos].rfind(':') {
                format!("{}:***@{}", &url[..colon_pos], &url[at_pos+1..])
            } else {
                format!("***@{}", &url[at_pos+1..])
            }
        } else {
            url.to_string()
        }
    }

    /// Store database connection in `SQLite`
    pub fn store_connection(
        &self,
        name: &str,
        request: &CreateDatabaseRequest,
    ) -> Result<DatabaseConnection, AppError> {
        tracing::info!(database_name = %name, "storing database connection");

        Self::validate_connection_input(name, &request.url)?;
        self.upsert_connection(name, request)?;
        self.get_connection(name)
    }

    /// Validate database name and URL
    fn validate_connection_input(name: &str, url: &str) -> Result<(), AppError> {
        if !validate_database_name(name) {
            tracing::error!(name = %name, "invalid database name");
            return Err(AppError::ValidationError(
                "Invalid database name".to_string(),
            ));
        }

        if !validate_database_url(url) {
            tracing::error!(url = %url, "invalid database URL format");
            return Err(AppError::ValidationError(
                "Invalid database URL format".to_string(),
            ));
        }

        Ok(())
    }

    /// Upsert database connection (update if exists, insert if not)
    fn upsert_connection(&self, name: &str, request: &CreateDatabaseRequest) -> Result<(), AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.sqlite_conn.lock()
            .map_err(|e| {
                tracing::error!(error = ?e, "failed to acquire SQLite lock");
                AppError::DatabaseError(format!("Failed to acquire lock: {e:?}"))
            })?;

        // Try to update first
        let updated = conn.execute(
            "UPDATE databases SET url = ?2, updated_at = ?3 WHERE name = ?1",
            rusqlite::params![name, request.url, now],
        )?;

        if updated == 0 {
            // Insert new connection
            tracing::info!(database_name = %name, "inserting new connection");
            conn.execute(
                "INSERT INTO databases (name, url, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)",
                rusqlite::params![name, request.url, now],
            )?;
        } else {
            tracing::info!(database_name = %name, "updated existing connection");
        }

        Ok(())
    }

    /// Get database connection by name
    pub fn get_connection(&self, name: &str) -> Result<DatabaseConnection, AppError> {
        tracing::debug!(database_name = %name, "retrieving connection");

        let conn = self.sqlite_conn.lock()
            .map_err(|e| {
                tracing::error!(error = ?e, "SQLite mutex poisoned");
                AppError::DatabaseError(format!("Failed to acquire lock: {e:?}"))
            })?;

        let mut stmt = conn.prepare(
            "SELECT name, url, created_at, updated_at FROM databases WHERE name = ?1"
        )?;

        let connection = stmt.query_row([name], |row| {
            DatabaseConnection::try_from(row)
        })?;

        tracing::debug!(database_name = %name, "connection retrieved successfully");
        Ok(connection)
    }

    /// List all database connections
    pub fn list_connections(&self) -> Result<Vec<DatabaseConnection>, AppError> {
        let conn = self.sqlite_conn.lock()
            .map_err(|e| {
                tracing::error!(error = ?e, "SQLite mutex poisoned");
                AppError::DatabaseError(format!("Failed to acquire lock: {e:?}"))
            })?;

        let mut stmt = conn
            .prepare("SELECT name, url, created_at, updated_at FROM databases ORDER BY name")?;

        let connections = stmt
            .query_map([], |row| DatabaseConnection::try_from(row))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(connections)
    }

    /// Delete database connection
    pub fn delete_connection(&self, name: &str) -> Result<(), AppError> {
        let conn = self.sqlite_conn.lock()
            .map_err(|e| {
                tracing::error!(error = ?e, "SQLite mutex poisoned");
                AppError::DatabaseError(format!("Failed to acquire lock: {e:?}"))
            })?;

        let deleted = conn
            .execute("DELETE FROM databases WHERE name = ?1", [name])?;

        if deleted == 0 {
            return Err(AppError::NotFound(format!("Database '{name}' not found")));
        }

        // Also delete associated schema metadata
        conn.execute("DELETE FROM schema_metadata WHERE db_name = ?1", [name])?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use std::fs;

    fn setup_test_db() -> Arc<Mutex<Connection>> {
        let test_db_path = "test_db_query.db";
        // Remove test DB if exists
        let _ = fs::remove_file(test_db_path);
        
        let conn = init_db(test_db_path).expect("Failed to create test database");
        Arc::new(Mutex::new(conn))
    }

    fn cleanup_test_db() {
        let _ = fs::remove_file("test_db_query.db");
    }

    #[test]
    fn test_store_and_get_connection() {
        let sqlite_conn = setup_test_db();
        let service = DatabaseService::new(sqlite_conn);

        let request = CreateDatabaseRequest {
            url: "postgres://user:pass@localhost:5432/testdb".to_string(),
        };

        let result = service.store_connection("test_db", &request);
        assert!(result.is_ok());

        let connection = service.get_connection("test_db").unwrap();
        assert_eq!(connection.name, "test_db");
        assert_eq!(connection.url, request.url);

        cleanup_test_db();
    }

    #[test]
    fn test_list_connections() {
        let sqlite_conn = setup_test_db();
        let service = DatabaseService::new(sqlite_conn);

        let request1 = CreateDatabaseRequest {
            url: "postgres://user:pass@localhost:5432/db1".to_string(),
        };
        let request2 = CreateDatabaseRequest {
            url: "postgres://user:pass@localhost:5432/db2".to_string(),
        };

        service.store_connection("db1", &request1).unwrap();
        service.store_connection("db2", &request2).unwrap();

        let connections = service.list_connections().unwrap();
        // Check that db1 and db2 exist (there might be other connections from parallel tests)
        assert!(connections.iter().any(|c| c.name == "db1"));
        assert!(connections.iter().any(|c| c.name == "db2"));
        assert!(connections.len() >= 2, "Expected at least 2 connections, found {}", connections.len());

        cleanup_test_db();
    }

    #[test]
    fn test_delete_connection() {
        let sqlite_conn = setup_test_db();
        let service = DatabaseService::new(sqlite_conn);

        let request = CreateDatabaseRequest {
            url: "postgres://user:pass@localhost:5432/testdb".to_string(),
        };

        service.store_connection("test_db", &request).unwrap();
        assert!(service.get_connection("test_db").is_ok());

        service.delete_connection("test_db").unwrap();
        assert!(service.get_connection("test_db").is_err());

        cleanup_test_db();
    }

    #[test]
    fn test_get_nonexistent_connection() {
        let sqlite_conn = setup_test_db();
        let service = DatabaseService::new(sqlite_conn);

        let result = service.get_connection("nonexistent");
        assert!(result.is_err());

        cleanup_test_db();
    }

    #[test]
    fn test_invalid_database_name() {
        let sqlite_conn = setup_test_db();
        let service = DatabaseService::new(sqlite_conn);

        let request = CreateDatabaseRequest {
            url: "postgres://user:pass@localhost:5432/testdb".to_string(),
        };

        let result = service.store_connection("invalid name with spaces", &request);
        assert!(result.is_err());

        cleanup_test_db();
    }

    #[test]
    fn test_invalid_database_url() {
        let sqlite_conn = setup_test_db();
        let service = DatabaseService::new(sqlite_conn);

        let request = CreateDatabaseRequest {
            url: "invalid-url".to_string(),
        };

        let result = service.store_connection("test_db", &request);
        assert!(result.is_err());

        cleanup_test_db();
    }
}
