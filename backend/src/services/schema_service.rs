use crate::error::AppError;
use crate::models::schema::{SchemaMetadata, TableInfo, ViewInfo, ColumnInfo};
use crate::services::database_service::DatabaseService;
use rusqlite::Connection;
use serde_json;
use sqlx::{PgPool, Row};
use std::sync::{Arc, Mutex};

pub struct SchemaService {
    sqlite_conn: Arc<Mutex<Connection>>,
    db_service: Arc<DatabaseService>,
}

impl SchemaService {
    pub fn new(sqlite_conn: Arc<Mutex<Connection>>, db_service: Arc<DatabaseService>) -> Self {
        Self {
            sqlite_conn,
            db_service,
        }
    }

    /// Retrieve schema metadata for a database
    pub async fn get_schema_metadata(&self, db_name: &str) -> Result<SchemaMetadata, AppError> {
        // Check if database exists
        self.db_service.get_connection(db_name)?;

        // Check cache first, but only use it if all tables have row_count
        if let Ok(cached) = self.get_cached_metadata(db_name) {
            // Check if cached data has row_count for all tables
            let has_all_row_counts = cached.tables.iter().all(|table| table.row_count.is_some());
            if has_all_row_counts {
                return Ok(cached);
            }
            // If cache is missing row_count, clear it and fetch fresh data
            self.clear_cache(db_name)?;
        }

        // Retrieve from target database
        let connection = self.db_service.get_connection(db_name)?;
        let mut metadata = self.retrieve_from_database(&connection.url, db_name).await?;
        metadata.db_name = db_name.to_string();

        // Cache the metadata
        self.cache_metadata(db_name, &metadata)?;

        Ok(metadata)
    }

    /// Retrieve schema from `PostgreSQL` database
    async fn retrieve_from_database(&self, url: &str, db_name: &str) -> Result<SchemaMetadata, AppError> {
        if !url.starts_with("postgres://") && !url.starts_with("postgresql://") {
            return Err(AppError::ValidationError(
                "Only PostgreSQL databases are supported".to_string(),
            ));
        }

        let pool = PgPool::connect(url).await?;

        // Get all tables
        let table_names: Vec<String> = sqlx::query_scalar(
            "SELECT table_name FROM information_schema.tables
             WHERE table_schema = 'public' AND table_type = 'BASE TABLE'
             ORDER BY table_name"
        )
        .fetch_all(&pool)
        .await?;

        let mut tables = Vec::new();
        for table_name in table_names {
            tables.push(Self::fetch_table_info(&pool, &table_name).await?);
        }

        // Get all views
        let view_names: Vec<String> = sqlx::query_scalar(
            "SELECT table_name FROM information_schema.views
             WHERE table_schema = 'public'
             ORDER BY table_name"
        )
        .fetch_all(&pool)
        .await?;

        let mut views = Vec::new();
        for view_name in view_names {
            views.push(Self::fetch_view_info(&pool, &view_name).await?);
        }

        Ok(SchemaMetadata {
            db_name: db_name.to_string(),
            tables,
            views,
            updated_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Fetch information for a single table
    async fn fetch_table_info(pool: &PgPool, table_name: &str) -> Result<TableInfo, AppError> {
        // Get columns for this table
        let columns: Vec<(String, String, bool, Option<String>)> = sqlx::query(
            "SELECT column_name, data_type, is_nullable = 'YES' as nullable, column_default
             FROM information_schema.columns
             WHERE table_schema = 'public' AND table_name = $1
             ORDER BY ordinal_position"
        )
        .bind(table_name)
        .map(|row: sqlx::postgres::PgRow| {
            (
                row.get(0),
                row.get(1),
                row.get(2),
                row.get(3),
            )
        })
        .fetch_all(pool)
        .await?;

        // Get primary key columns
        let primary_key: Vec<String> = sqlx::query_scalar(
            "SELECT kcu.column_name
             FROM information_schema.table_constraints tc
             JOIN information_schema.key_column_usage kcu
                 ON tc.constraint_name = kcu.constraint_name
             WHERE tc.table_schema = 'public'
                 AND tc.table_name = $1
                 AND tc.constraint_type = 'PRIMARY KEY'
             ORDER BY kcu.ordinal_position"
        )
        .bind(table_name)
        .fetch_all(pool)
        .await?;

        // Get row count for this table
        let row_count = Self::fetch_table_row_count(pool, table_name).await;

        let column_infos: Vec<ColumnInfo> = columns
            .into_iter()
            .map(|(name, data_type, nullable, default_value)| {
                ColumnInfo {
                    name,
                    data_type,
                    nullable,
                    default_value,
                }
            })
            .collect();

        Ok(TableInfo {
            name: table_name.to_string(),
            columns: column_infos,
            primary_key: if primary_key.is_empty() { None } else { Some(primary_key) },
            row_count,
        })
    }

    /// Fetch row count for a table (with validation and error handling)
    async fn fetch_table_row_count(pool: &PgPool, table_name: &str) -> Option<u64> {
        // Validate table name to prevent SQL injection
        if !is_valid_identifier(table_name) {
            tracing::warn!(table = %table_name, "invalid table name, skipping row count");
            return None;
        }

        match sqlx::query_scalar::<_, i64>(
            &format!("SELECT COUNT(*) FROM \"{table_name}\"")
        )
        .fetch_one(pool)
        .await
        {
            Ok(count) => {
                #[allow(clippy::cast_sign_loss)]
                if count >= 0 {
                    Some(count as u64)
                } else {
                    tracing::warn!(table = %table_name, count = count, "negative row count");
                    None
                }
            }
            Err(e) => {
                tracing::warn!(
                    table = %table_name,
                    error = ?e,
                    "failed to get row count"
                );
                None
            }
        }
    }

    /// Fetch information for a single view
    async fn fetch_view_info(pool: &PgPool, view_name: &str) -> Result<ViewInfo, AppError> {
        // Get columns for this view
        let columns: Vec<(String, String, bool, Option<String>)> = sqlx::query(
            "SELECT column_name, data_type, is_nullable = 'YES' as nullable, column_default
             FROM information_schema.columns
             WHERE table_schema = 'public' AND table_name = $1
             ORDER BY ordinal_position"
        )
        .bind(view_name)
        .map(|row: sqlx::postgres::PgRow| {
            (
                row.get(0),
                row.get(1),
                row.get(2),
                row.get(3),
            )
        })
        .fetch_all(pool)
        .await?;

        let column_infos: Vec<ColumnInfo> = columns
            .into_iter()
            .map(|(name, data_type, nullable, default_value)| {
                ColumnInfo {
                    name,
                    data_type,
                    nullable,
                    default_value,
                }
            })
            .collect();

        Ok(ViewInfo {
            name: view_name.to_string(),
            columns: column_infos,
        })
    }

    /// Get cached metadata from `SQLite`
    fn get_cached_metadata(&self, db_name: &str) -> Result<SchemaMetadata, AppError> {
        let conn = self.sqlite_conn.lock()
            .map_err(|e| {
                tracing::error!(error = ?e, "SQLite mutex poisoned");
                AppError::DatabaseError(format!("Failed to acquire lock: {e:?}"))
            })?;

        let mut stmt = conn.prepare(
            "SELECT table_name, table_type, metadata_json FROM schema_metadata WHERE db_name = ?1"
        )?;

        let rows: Vec<(String, String, String)> = stmt
            .query_map([db_name], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        if rows.is_empty() {
            return Err(AppError::NotFound("No cached metadata found".to_string()));
        }

        let mut tables = Vec::new();
        let mut views = Vec::new();

        for (table_name, table_type, metadata_json) in rows {
            let metadata: serde_json::Value = serde_json::from_str(&metadata_json)?;
            let columns: Vec<ColumnInfo> = serde_json::from_value(
                metadata.get("columns").cloned().unwrap_or(serde_json::json!([]))
            )?;

            if table_type == "table" {
                let primary_key = metadata
                    .get("primaryKey")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(std::string::ToString::to_string)).collect());

                let row_count = metadata
                    .get("rowCount")
                    .and_then(serde_json::Value::as_u64);

                tables.push(TableInfo {
                    name: table_name,
                    columns,
                    primary_key,
                    row_count,
                });
            } else {
                views.push(ViewInfo {
                    name: table_name,
                    columns,
                });
            }
        }

        Ok(SchemaMetadata {
            db_name: db_name.to_string(),
            tables,
            views,
            updated_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Clear cache for a database
    fn clear_cache(&self, db_name: &str) -> Result<(), AppError> {
        let conn = self.sqlite_conn.lock()
            .map_err(|e| {
                tracing::error!(error = ?e, "SQLite mutex poisoned");
                AppError::DatabaseError(format!("Failed to acquire lock: {e:?}"))
            })?;
        conn.execute("DELETE FROM schema_metadata WHERE db_name = ?1", [db_name])?;
        Ok(())
    }

    /// Cache metadata in `SQLite`
    fn cache_metadata(&self, db_name: &str, metadata: &SchemaMetadata) -> Result<(), AppError> {
        let conn = self.sqlite_conn.lock()
            .map_err(|e| {
                tracing::error!(error = ?e, "SQLite mutex poisoned");
                AppError::DatabaseError(format!("Failed to acquire lock: {e:?}"))
            })?;
        // Delete existing cache
        conn.execute("DELETE FROM schema_metadata WHERE db_name = ?1", [db_name])?;

        let now = chrono::Utc::now().to_rfc3339();

        // Cache tables
        for table in &metadata.tables {
            let metadata_json = serde_json::json!({
                "columns": table.columns,
                "primaryKey": table.primary_key,
                "rowCount": table.row_count,
            });

            conn.execute(
                "INSERT INTO schema_metadata (db_name, table_name, table_type, metadata_json, updated_at) 
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![
                    db_name,
                    table.name,
                    "table",
                    metadata_json.to_string(),
                    now
                ],
            )?;
        }

        // Cache views
        for view in &metadata.views {
            let metadata_json = serde_json::json!({
                "columns": view.columns,
            });

            conn.execute(
                "INSERT INTO schema_metadata (db_name, table_name, table_type, metadata_json, updated_at) 
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![
                    db_name,
                    view.name,
                    "view",
                    metadata_json.to_string(),
                    now
                ],
            )?;
        }

        Ok(())
    }
}

/// Validate that a string is a valid `PostgreSQL` identifier
/// This prevents SQL injection when using identifiers in format! macros
fn is_valid_identifier(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 63 // PostgreSQL identifier length limit
        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
        && name.chars().next().is_some_and(|c| !c.is_ascii_digit()) // Can't start with digit
}

