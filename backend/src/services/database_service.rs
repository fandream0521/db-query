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

