use axum::{
    http::Method,
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use tokio::net::TcpListener;
use std::sync::Arc;

mod db;
mod error;
mod config;
mod api;
mod models;
mod services;
mod utils;
mod types;

use api::databases::{SharedDatabaseService, SharedSchemaService, list_databases, get_database_metadata, upsert_database, delete_database};
use api::queries::{execute_query, execute_natural_language_query};
use db::init_db;
use config::Config;
use services::database_service::DatabaseService;
use services::schema_service::SchemaService;

#[tokio::main]
async fn main() {
    // Initialize logging with more detailed output
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    
    // Initialize SQLite database
    let sqlite_conn = init_db(&config.sqlite_db_path)
        .expect("Failed to initialize SQLite database");
    
    // Wrap SQLite connection in Arc<Mutex<>> for sharing between services
    let sqlite_conn = Arc::new(std::sync::Mutex::new(sqlite_conn));
    
    // Create database service
    let db_service: SharedDatabaseService = Arc::new(DatabaseService::new(sqlite_conn.clone()));
    
    // Create schema service
    let schema_service: SharedSchemaService = Arc::new(SchemaService::new(sqlite_conn, db_service.clone()));

    // Configure CORS to allow all origins
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    // Create router with API routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/dbs", get(list_databases))
        .route("/api/v1/dbs/{name}", get(get_database_metadata))
        .route("/api/v1/dbs/{name}", put(upsert_database))
        .route("/api/v1/dbs/{name}", delete(delete_database))
        .route("/api/v1/dbs/{name}/query", post(execute_query))
        .route("/api/v1/dbs/{name}/query/natural", post(execute_natural_language_query))
        .with_state((db_service, schema_service))
        .layer(cors);

    // Start server
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Server listening on {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}
