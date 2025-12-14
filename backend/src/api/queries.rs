use axum::{
    extract::{Path, State},
    Json,
};
use crate::error::AppError;
use crate::models::query::{QueryRequest, QueryResponse};
use crate::api::databases::{SharedDatabaseService, SharedSchemaService};
use crate::services::query_executor::QueryExecutor;
use crate::services::sql_validator::validate_sql;
use sqlx::PgPool;

/// POST /api/v1/dbs/{name}/query
/// Execute a SQL query against the specified database
pub async fn execute_query(
    State((service, _)): State<(SharedDatabaseService, SharedSchemaService)>,
    Path(name): Path<String>,
    Json(request): Json<QueryRequest>,
) -> Result<Json<QueryResponse>, AppError> {
    // Validate SQL is not empty
    if request.sql.trim().is_empty() {
        return Err(AppError::ValidationError(
            "SQL query cannot be empty".to_string(),
        ));
    }

    // Get database connection
    let connection = service.get_connection(&name)?;

    // Validate SQL syntax and ensure it's SELECT only
    let validated_sql = validate_sql(&request.sql)?;

    // Create PostgreSQL connection pool
    let pool = PgPool::connect(&connection.url).await?;

    // Execute query
    let response = QueryExecutor::execute_query(&pool, &validated_sql).await?;

    Ok(Json(response))
}

/// POST /api/v1/dbs/{name}/query/natural
/// Execute a natural language query (generates SQL and executes it)
/// Implementation will be added in Phase 6 (US4)
pub async fn execute_natural_language_query(
    State((_service, _)): State<(SharedDatabaseService, SharedSchemaService)>,
    Path(_name): Path<String>,
    Json(_request): Json<serde_json::Value>,
) -> Result<Json<QueryResponse>, AppError> {
    Err(AppError::InternalError(
        "Natural language queries not yet implemented".to_string(),
    ))
}

