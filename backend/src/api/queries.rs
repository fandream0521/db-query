use axum::{
    extract::{Path, State},
    Json,
};
use crate::error::AppError;
use crate::models::query::{QueryRequest, QueryResponse};
use crate::models::natural_language::NaturalLanguageQueryRequest;
use crate::api::databases::{SharedDatabaseService, SharedSchemaService};
use crate::services::query_executor::QueryExecutor;
use crate::services::sql_validator::validate_sql;
use crate::services::llm_service::LLMService;
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
pub async fn execute_natural_language_query(
    State((db_service, schema_service)): State<(SharedDatabaseService, SharedSchemaService)>,
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

    // Initialize LLM service (get config from environment)
    let llm_api_key = std::env::var("LLM_API_KEY")
        .unwrap_or_else(|_| "".to_string());
    let llm_api_url = std::env::var("LLM_API_URL")
        .unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".to_string());
    let llm_service = LLMService::new(llm_api_key, llm_api_url);

    // Convert natural language to SQL
    let sql = llm_service
        .natural_language_to_sql(&request.prompt, &schema)
        .await?;

    // Validate the generated SQL
    let validated_sql = validate_sql(&sql)?;

    // Create PostgreSQL connection pool
    let pool = PgPool::connect(&connection.url).await?;

    // Execute query
    let response = QueryExecutor::execute_query(&pool, &validated_sql).await?;

    Ok(Json(response))
}

