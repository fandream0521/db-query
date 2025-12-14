use crate::error::AppError;
use crate::models::database::DatabaseConnection;
use crate::models::request::CreateDatabaseRequest;
use crate::utils::validation::{validate_database_name, validate_database_url};
use rusqlite::Connection;
use sqlx::PgPool;
use std::sync::{Arc, Mutex};

pub struct DatabaseService {
    sqlite_conn: Arc<Mutex<Connection>>,
}

impl DatabaseService {
    pub fn new(sqlite_conn: Arc<Mutex<Connection>>) -> Self {
        Self { sqlite_conn }
    }

    /// Test database connection
    pub async fn test_connection(url: &str) -> Result<(), AppError> {
        if !validate_database_url(url) {
            return Err(AppError::ValidationError(
                "Invalid database URL format".to_string(),
            ));
        }

        // For PostgreSQL connections
        if url.starts_with("postgres://") || url.starts_with("postgresql://") {
            let options = url.parse::<sqlx::postgres::PgConnectOptions>()?;
            let pool = PgPool::connect_with(options)
                .await?;

            // Test connection with a simple query
            sqlx::query("SELECT 1").execute(&pool).await?;
        }

        Ok(())
    }

    /// Store database connection in SQLite
    pub fn store_connection(
        &self,
        name: &str,
        request: &CreateDatabaseRequest,
    ) -> Result<DatabaseConnection, AppError> {
        if !validate_database_name(name) {
            return Err(AppError::ValidationError(
                "Invalid database name".to_string(),
            ));
        }

        if !validate_database_url(&request.url) {
            return Err(AppError::ValidationError(
                "Invalid database URL format".to_string(),
            ));
        }

        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.sqlite_conn.lock().unwrap();
        
        // Try to update first, if not exists, insert
        let updated = conn.execute(
            "UPDATE databases SET url = ?2, updated_at = ?3 WHERE name = ?1",
            rusqlite::params![name, request.url, now],
        )?;

        if updated == 0 {
            // Insert new connection
            conn.execute(
                "INSERT INTO databases (name, url, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)",
                rusqlite::params![name, request.url, now],
            )?;
        }

        // Retrieve the stored connection
        self.get_connection(name)
    }

    /// Get database connection by name
    pub fn get_connection(&self, name: &str) -> Result<DatabaseConnection, AppError> {
        let conn = self.sqlite_conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT name, url, created_at, updated_at FROM databases WHERE name = ?1")?;

        let connection = stmt.query_row([name], |row| {
            Ok(DatabaseConnection::from(row))
        })?;

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
