use axum::{
    extract::{Path, State},
    Json,
};
use crate::error::AppError;
use crate::models::query::{QueryRequest, QueryResponse};
use crate::models::natural_language::NaturalLanguageQueryRequest;
use crate::types::{SharedDatabaseService, SharedSchemaService, SharedLLMService, SharedConnectionPoolCache};
use crate::services::query_executor::QueryExecutor;
use crate::services::sql_validator::validate_sql;

/// POST /api/v1/dbs/{name}/query
/// Execute a SQL query against the specified database
pub async fn execute_query(
    State((db_service, _, _, pool_cache)): State<(
        SharedDatabaseService,
        SharedSchemaService,
        SharedLLMService,
        SharedConnectionPoolCache,
    )>,
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
    let connection = db_service.get_connection(&name)?;

    // Validate SQL syntax and ensure it's SELECT only
    let validated_sql = validate_sql(&request.sql)?;

    // Get or create connection pool
    let pool = pool_cache.get_or_create(&name, &connection.url).await?;

    // Execute query
    let response = QueryExecutor::execute_query(&pool, &validated_sql).await?;

    Ok(Json(response))
}

/// POST /api/v1/dbs/{name}/query/natural
/// Execute a natural language query (generates SQL and executes it)
pub async fn execute_natural_language_query(
    State((db_service, schema_service, llm_service, pool_cache)): State<(
        SharedDatabaseService,
        SharedSchemaService,
        SharedLLMService,
        SharedConnectionPoolCache,
    )>,
    Path(name): Path<String>,
    Json(request): Json<NaturalLanguageQueryRequest>,
) -> Result<Json<QueryResponse>, AppError> {
    // Validate prompt is not empty
    if request.prompt.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Natural language query cannot be empty".to_string(),
        ));
    }

    // Get database connection
    let connection = db_service.get_connection(&name)?;

    // Get schema metadata for context
    let schema = schema_service.get_schema_metadata(&name).await?;

    // Convert natural language to SQL using LLM service from state
    let sql = llm_service
        .natural_language_to_sql(&request.prompt, &schema)
        .await?;

    // Validate the generated SQL
    let validated_sql = validate_sql(&sql)?;

    // Get or create connection pool
    let pool = pool_cache.get_or_create(&name, &connection.url).await?;

    // Execute query
    let response = QueryExecutor::execute_query(&pool, &validated_sql).await?;

    Ok(Json(response))
}

