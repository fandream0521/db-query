use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::error::AppError;
use crate::models::database::DatabaseConnection;
use crate::models::request::CreateDatabaseRequest;
use crate::models::schema::SchemaMetadata;
use crate::services::database_service::DatabaseService;
use crate::services::schema_service::SchemaService;
use std::sync::Arc;

pub type SharedDatabaseService = Arc<DatabaseService>;
pub type SharedSchemaService = Arc<SchemaService>;

/// GET /api/v1/dbs
/// List all database connections
pub async fn list_databases(
    State((service, _)): State<(SharedDatabaseService, SharedSchemaService)>,
) -> Result<Json<Vec<DatabaseConnection>>, AppError> {
    let connections = service.list_connections()?;
    Ok(Json(connections))
}

/// GET /api/v1/dbs/{name}
/// Get database metadata (schema information)
pub async fn get_database_metadata(
    State((_, schema_service)): State<(SharedDatabaseService, SharedSchemaService)>,
    Path(name): Path<String>,
) -> Result<Json<SchemaMetadata>, AppError> {
    let metadata = schema_service.get_schema_metadata(&name).await?;
    Ok(Json(metadata))
}

/// PUT /api/v1/dbs/{name}
/// Create or update a database connection
pub async fn upsert_database(
    State((service, _)): State<(SharedDatabaseService, SharedSchemaService)>,
    Path(name): Path<String>,
    Json(request): Json<CreateDatabaseRequest>,
) -> Result<Json<DatabaseConnection>, AppError> {
    // Test connection first
    DatabaseService::test_connection(&request.url).await?;
    
    // Store connection
    let connection = service.store_connection(&name, &request)?;
    Ok(Json(connection))
}

/// DELETE /api/v1/dbs/{name}
/// Delete a database connection
pub async fn delete_database(
    State((service, _)): State<(SharedDatabaseService, SharedSchemaService)>,
    Path(name): Path<String>,
) -> Result<StatusCode, AppError> {
    service.delete_connection(&name)?;
    Ok(StatusCode::NO_CONTENT)
}
