use crate::error::AppError;
use crate::models::query::QueryResponse;
use sqlx::{PgPool, Row, Column};
use std::time::Instant;

pub struct QueryExecutor;

impl QueryExecutor {
    /// Execute a SQL query against PostgreSQL database
    pub async fn execute_query(
        pool: &PgPool,
        sql: &str,
    ) -> Result<QueryResponse, AppError> {
        let start_time = Instant::now();
        
        // Execute query
        let rows = sqlx::query(sql)
            .fetch_all(pool)
            .await?;
        
        let execution_time = start_time.elapsed().as_millis() as u64;
        
        // Get column names - try from first row, or use metadata query if empty
        let columns = if let Some(first_row) = rows.first() {
            first_row.columns()
                .iter()
                .map(|col| col.name().to_string())
                .collect()
        } else {
            // For empty results, get column metadata by executing with LIMIT 0
            Self::get_column_names_from_metadata(pool, sql).await?
        };
        
        // Convert rows to JSON values
        let mut result_rows = Vec::new();
        for row in rows {
            let mut row_values = Vec::new();
            for col_name in &columns {
                // Try to get value as JSON
                // sqlx doesn't directly support getting as JSON, so we'll use try_get with different types
                let value = Self::get_value_as_json(&row, col_name);
                row_values.push(value);
            }
            result_rows.push(row_values);
        }
        
        let row_count = result_rows.len();
        
        Ok(QueryResponse {
            columns,
            rows: result_rows,
            row_count,
            execution_time_ms: execution_time,
        })
    }
    
    /// Get column names from query metadata (for empty result sets)
    async fn get_column_names_from_metadata(
        pool: &PgPool,
        sql: &str,
    ) -> Result<Vec<String>, AppError> {
        // Execute with LIMIT 0 to get metadata without data
        let metadata_sql = if !sql.to_uppercase().contains("LIMIT") {
            format!("{} LIMIT 0", sql)
        } else {
            // Wrap in subquery to add LIMIT 0
            format!("SELECT * FROM ({}) AS _ LIMIT 0", sql)
        };
        
        let metadata_rows = sqlx::query(&metadata_sql)
            .fetch_all(pool)
            .await;
        
        if let Ok(rows) = metadata_rows {
            if let Some(row) = rows.first() {
                return Ok(row.columns().iter().map(|c| c.name().to_string()).collect());
            }
        }
        
        // Fallback: return empty
        Ok(Vec::new())
    }
    
    /// Get value from row as JSON, trying different types
    fn get_value_as_json(row: &sqlx::postgres::PgRow, col_name: &str) -> serde_json::Value {
        // Try different types in order of likelihood
        // String
        if let Ok(val) = row.try_get::<Option<String>, _>(col_name) {
            return val.map(serde_json::Value::String)
                .unwrap_or(serde_json::Value::Null);
        }
        
        // Integer
        if let Ok(val) = row.try_get::<Option<i64>, _>(col_name) {
            return val.map(|v| serde_json::Value::Number(v.into()))
                .unwrap_or(serde_json::Value::Null);
        }
        
        // Float
        if let Ok(val) = row.try_get::<Option<f64>, _>(col_name) {
            return val.and_then(|v| serde_json::Number::from_f64(v))
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null);
        }
        
        // Boolean
        if let Ok(val) = row.try_get::<Option<bool>, _>(col_name) {
            return val.map(serde_json::Value::Bool)
                .unwrap_or(serde_json::Value::Null);
        }
        
        // Try as JSON value directly (PostgreSQL json/jsonb)
        if let Ok(val) = row.try_get::<Option<serde_json::Value>, _>(col_name) {
            return val.unwrap_or(serde_json::Value::Null);
        }
        
        // Fallback: try to get as text and parse
        if let Ok(val) = row.try_get::<Option<String>, _>(col_name) {
            if let Some(text) = val {
                // Try to parse as JSON
                if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&text) {
                    return json_val;
                }
                return serde_json::Value::String(text);
            }
        }
        
        // Default to null if we can't determine the type
        serde_json::Value::Null
    }
}
