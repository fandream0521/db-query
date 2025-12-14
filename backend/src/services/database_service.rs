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
        tracing::info!("[test_connection] Starting connection test for URL: {}", url);
        
        tracing::debug!("[test_connection] Validating database URL format...");
        if !validate_database_url(url) {
            tracing::error!("[test_connection] Invalid database URL format: {}", url);
            return Err(AppError::ValidationError(
                "Invalid database URL format".to_string(),
            ));
        }
        tracing::debug!("[test_connection] URL format validation passed");

        // For PostgreSQL connections
        if url.starts_with("postgres://") || url.starts_with("postgresql://") {
            tracing::info!("[test_connection] Detected PostgreSQL connection string");
            
            tracing::debug!("[test_connection] Parsing PostgreSQL connection options...");
            let options = match url.parse::<sqlx::postgres::PgConnectOptions>() {
                Ok(opts) => {
                    tracing::debug!("[test_connection] Connection options parsed successfully");
                    // Log connection details (mask password in URL for security)
                    let masked_url = if let Some(at_pos) = url.find('@') {
                        if let Some(colon_pos) = url[..at_pos].rfind(':') {
                            format!("{}:***@{}", &url[..colon_pos], &url[at_pos+1..])
                        } else {
                            format!("***@{}", &url[at_pos+1..])
                        }
                    } else {
                        url.to_string()
                    };
                    tracing::info!("[test_connection] Connecting to: {}", masked_url);
                    opts
                }
                Err(e) => {
                    tracing::error!("[test_connection] Failed to parse connection options: {:?}", e);
                    return Err(AppError::from(e));
                }
            };
            
            // Set connection timeout (15 seconds total)
            let connect_timeout = Duration::from_secs(15);
            tracing::info!("[test_connection] Connection timeout set to {} seconds", connect_timeout.as_secs());
            
            tracing::info!("[test_connection] Attempting to connect to PostgreSQL database...");
            let start_time = std::time::Instant::now();
            
            // Wrap connection in timeout
            let connect_result = timeout(
                Duration::from_secs(15),
                PgPool::connect_with(options)
            ).await;
            
            let elapsed = start_time.elapsed();
            tracing::info!("[test_connection] Connection attempt took {:.2} seconds", elapsed.as_secs_f64());
            
            let pool = match connect_result {
                Ok(Ok(p)) => {
                    tracing::info!("[test_connection] PostgreSQL connection pool created successfully");
                    p
                }
                Ok(Err(e)) => {
                    tracing::error!("[test_connection] Failed to connect to PostgreSQL: {:?}", e);
                    tracing::error!("[test_connection] Error details: {}", e);
                    return Err(AppError::ConnectionError(format!(
                        "Failed to connect to PostgreSQL: {}. Connection took {:.2} seconds",
                        e, elapsed.as_secs_f64()
                    )));
                }
                Err(_) => {
                    tracing::error!("[test_connection] Connection timeout after {} seconds", elapsed.as_secs());
                    return Err(AppError::ConnectionError(format!(
                        "Connection timeout after {} seconds. Please check if PostgreSQL is running and accessible.",
                        elapsed.as_secs()
                    )));
                }
            };

            // Test connection with a simple query (with timeout)
            tracing::debug!("[test_connection] Executing test query: SELECT 1");
            let query_start = std::time::Instant::now();
            let query_result = timeout(
                Duration::from_secs(5),
                sqlx::query("SELECT 1").execute(&pool)
            ).await;
            
            let query_elapsed = query_start.elapsed();
            tracing::info!("[test_connection] Query execution took {:.2} seconds", query_elapsed.as_secs_f64());
            
            match query_result {
                Ok(Ok(_)) => {
                    tracing::info!("[test_connection] Test query executed successfully");
                }
                Ok(Err(e)) => {
                    tracing::error!("[test_connection] Test query failed: {:?}", e);
                    return Err(AppError::from(e));
                }
                Err(_) => {
                    tracing::error!("[test_connection] Test query timeout after {} seconds", query_elapsed.as_secs());
                    return Err(AppError::ConnectionError(
                        "Test query timeout. Database may be slow or unresponsive.".to_string()
                    ));
                }
            }
            
            tracing::info!("[test_connection] PostgreSQL connection test completed successfully");
        } else {
            tracing::warn!("[test_connection] Unsupported database type, skipping connection test");
        }

        tracing::info!("[test_connection] Connection test completed successfully");
        Ok(())
    }

    /// Store database connection in SQLite
    pub fn store_connection(
        &self,
        name: &str,
        request: &CreateDatabaseRequest,
    ) -> Result<DatabaseConnection, AppError> {
        tracing::info!("[store_connection] Starting to store connection - name: {}", name);
        
        tracing::debug!("[store_connection] Validating database name...");
        if !validate_database_name(name) {
            tracing::error!("[store_connection] Invalid database name: {}", name);
            return Err(AppError::ValidationError(
                "Invalid database name".to_string(),
            ));
        }
        tracing::debug!("[store_connection] Database name validation passed");

        tracing::debug!("[store_connection] Validating database URL...");
        if !validate_database_url(&request.url) {
            tracing::error!("[store_connection] Invalid database URL format: {}", request.url);
            return Err(AppError::ValidationError(
                "Invalid database URL format".to_string(),
            ));
        }
        tracing::debug!("[store_connection] Database URL validation passed");

        let now = chrono::Utc::now().to_rfc3339();
        tracing::debug!("[store_connection] Acquiring SQLite connection lock...");
        let conn = self.sqlite_conn.lock().unwrap();
        tracing::debug!("[store_connection] SQLite connection lock acquired");
        
        // Try to update first, if not exists, insert
        tracing::debug!("[store_connection] Attempting to update existing connection...");
        let updated = match conn.execute(
            "UPDATE databases SET url = ?2, updated_at = ?3 WHERE name = ?1",
            rusqlite::params![name, request.url, now],
        ) {
            Ok(count) => {
                tracing::debug!("[store_connection] Update query executed, affected rows: {}", count);
                count
            }
            Err(e) => {
                tracing::error!("[store_connection] Failed to execute update query: {:?}", e);
                return Err(AppError::from(e));
            }
        };

        if updated == 0 {
            // Insert new connection
            tracing::info!("[store_connection] No existing connection found, inserting new connection...");
            match conn.execute(
                "INSERT INTO databases (name, url, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)",
                rusqlite::params![name, request.url, now],
            ) {
                Ok(_) => {
                    tracing::info!("[store_connection] New connection inserted successfully");
                }
                Err(e) => {
                    tracing::error!("[store_connection] Failed to insert new connection: {:?}", e);
                    return Err(AppError::from(e));
                }
            }
        } else {
            tracing::info!("[store_connection] Existing connection updated successfully");
        }

        // Retrieve the stored connection (query within the same lock to avoid deadlock)
        tracing::debug!("[store_connection] Retrieving stored connection...");
        let mut stmt = match conn.prepare("SELECT name, url, created_at, updated_at FROM databases WHERE name = ?1") {
            Ok(s) => {
                tracing::debug!("[store_connection] Prepared SELECT statement");
                s
            }
            Err(e) => {
                tracing::error!("[store_connection] Failed to prepare SELECT statement: {:?}", e);
                return Err(AppError::from(e));
            }
        };

        let connection = match stmt.query_row([name], |row| {
            tracing::debug!("[store_connection] Processing retrieved row...");
            Ok(DatabaseConnection {
                name: row.get(0)?,
                url: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        }) {
            Ok(c) => {
                tracing::info!("[store_connection] Connection retrieved successfully - name: {}", name);
                c
            }
            Err(e) => {
                tracing::error!("[store_connection] Failed to retrieve stored connection: {:?}", e);
                return Err(AppError::from(e));
            }
        };

        // Lock is automatically released when conn goes out of scope
        tracing::debug!("[store_connection] Releasing SQLite lock...");
        Ok(connection)
    }

    /// Get database connection by name
    pub fn get_connection(&self, name: &str) -> Result<DatabaseConnection, AppError> {
        tracing::debug!("[get_connection] Retrieving connection - name: {}", name);
        tracing::debug!("[get_connection] Acquiring SQLite lock...");
        let conn = match self.sqlite_conn.lock() {
            Ok(c) => {
                tracing::debug!("[get_connection] SQLite lock acquired");
                c
            }
            Err(e) => {
                tracing::error!("[get_connection] Failed to acquire SQLite lock: {:?}", e);
                return Err(AppError::DatabaseError(format!("Failed to acquire lock: {:?}", e)));
            }
        };
        
        tracing::debug!("[get_connection] Preparing SQL statement...");
        let mut stmt = match conn.prepare("SELECT name, url, created_at, updated_at FROM databases WHERE name = ?1") {
            Ok(s) => {
                tracing::debug!("[get_connection] SQL statement prepared successfully");
                s
            }
            Err(e) => {
                tracing::error!("[get_connection] Failed to prepare statement: {:?}", e);
                return Err(AppError::from(e));
            }
        };

        tracing::debug!("[get_connection] Executing query for name: {}", name);
        let connection = match stmt.query_row([name], |row| {
            tracing::debug!("[get_connection] Processing row data...");
            let name_val: String = match row.get(0) {
                Ok(v) => {
                    tracing::debug!("[get_connection] Got name: {}", v);
                    v
                }
                Err(e) => {
                    tracing::error!("[get_connection] Failed to get name: {:?}", e);
                    return Err(e);
                }
            };
            let url_val: String = match row.get(1) {
                Ok(v) => {
                    tracing::debug!("[get_connection] Got url: {}", v);
                    v
                }
                Err(e) => {
                    tracing::error!("[get_connection] Failed to get url: {:?}", e);
                    return Err(e);
                }
            };
            let created_at_val: String = match row.get(2) {
                Ok(v) => {
                    tracing::debug!("[get_connection] Got created_at: {}", v);
                    v
                }
                Err(e) => {
                    tracing::error!("[get_connection] Failed to get created_at: {:?}", e);
                    return Err(e);
                }
            };
            let updated_at_val: String = match row.get(3) {
                Ok(v) => {
                    tracing::debug!("[get_connection] Got updated_at: {}", v);
                    v
                }
                Err(e) => {
                    tracing::error!("[get_connection] Failed to get updated_at: {:?}", e);
                    return Err(e);
                }
            };
            tracing::debug!("[get_connection] Creating DatabaseConnection object...");
            Ok(DatabaseConnection {
                name: name_val,
                url: url_val,
                created_at: created_at_val,
                updated_at: updated_at_val,
            })
        }) {
            Ok(c) => {
                tracing::debug!("[get_connection] Query executed successfully");
                c
            }
            Err(e) => {
                tracing::error!("[get_connection] Query failed: {:?}", e);
                return Err(AppError::from(e));
            }
        };

        tracing::info!("[get_connection] Connection retrieved successfully - name: {}", name);
        Ok(connection)
    }

    /// List all database connections
    pub fn list_connections(&self) -> Result<Vec<DatabaseConnection>, AppError> {
        let conn = self.sqlite_conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT name, url, created_at, updated_at FROM databases ORDER BY name")?;

        let connections = stmt
            .query_map([], |row| Ok(DatabaseConnection::from(row)))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(connections)
    }

    /// Delete database connection
    pub fn delete_connection(&self, name: &str) -> Result<(), AppError> {
        let conn = self.sqlite_conn.lock().unwrap();
        let deleted = conn
            .execute("DELETE FROM databases WHERE name = ?1", [name])?;

        if deleted == 0 {
            return Err(AppError::NotFound(format!("Database '{}' not found", name)));
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
        assert_eq!(connections.len(), 2);
        assert!(connections.iter().any(|c| c.name == "db1"));
        assert!(connections.iter().any(|c| c.name == "db2"));

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
