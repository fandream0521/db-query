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
use crate::types::{SharedDatabaseService, SharedSchemaService, SharedLLMService, SharedConnectionPoolCache};

/// GET /api/v1/dbs
/// List all database connections
pub async fn list_databases(
    State((service, _, _, _)): State<(
        SharedDatabaseService,
        SharedSchemaService,
        SharedLLMService,
        SharedConnectionPoolCache,
    )>,
) -> Result<Json<Vec<DatabaseConnection>>, AppError> {
    let connections = service.list_connections()?;
    Ok(Json(connections))
}

/// GET /api/v1/dbs/{name}
/// Get database metadata (schema information)
pub async fn get_database_metadata(
    State((_, schema_service, _, _)): State<(
        SharedDatabaseService,
        SharedSchemaService,
        SharedLLMService,
        SharedConnectionPoolCache,
    )>,
    Path(name): Path<String>,
) -> Result<Json<SchemaMetadata>, AppError> {
    let metadata = schema_service.get_schema_metadata(&name).await?;
    Ok(Json(metadata))
}

/// PUT /api/v1/dbs/{name}
/// Create or update a database connection
pub async fn upsert_database(
    State((service, _, _, _)): State<(
        SharedDatabaseService,
        SharedSchemaService,
        SharedLLMService,
        SharedConnectionPoolCache,
    )>,
    Path(name): Path<String>,
    Json(request): Json<CreateDatabaseRequest>,
) -> Result<Json<DatabaseConnection>, AppError> {
    tracing::info!("[upsert_database] Starting database upsert - name: {}, url: {}", name, request.url);

    // Test connection first
    tracing::info!("[upsert_database] Testing database connection...");
    match DatabaseService::test_connection(&request.url).await {
        Ok(_) => {
            tracing::info!("[upsert_database] Connection test successful");
        }
        Err(e) => {
            tracing::error!("[upsert_database] Connection test failed: {:?}", e);
            return Err(e);
        }
    }

    // Store connection
    tracing::info!("[upsert_database] Storing database connection...");
    let connection = match service.store_connection(&name, &request) {
        Ok(conn) => {
            tracing::info!("[upsert_database] Database connection stored successfully - name: {}", name);
            conn
        }
        Err(e) => {
            tracing::error!("[upsert_database] Failed to store connection: {:?}", e);
            return Err(e);
        }
    };

    tracing::info!("[upsert_database] Database upsert completed successfully - name: {}", name);
    Ok(Json(connection))
}

/// DELETE /api/v1/dbs/{name}
/// Delete a database connection
pub async fn delete_database(
    State((service, _, _, pool_cache)): State<(
        SharedDatabaseService,
        SharedSchemaService,
        SharedLLMService,
        SharedConnectionPoolCache,
    )>,
    Path(name): Path<String>,
) -> Result<StatusCode, AppError> {
    service.delete_connection(&name)?;

    // Remove connection pool from cache
    pool_cache.remove(&name).await;

    Ok(StatusCode::NO_CONTENT)
}
