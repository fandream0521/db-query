# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Database Query Tool - A full-stack web application for querying PostgreSQL databases with natural language to SQL conversion. The system allows users to:
- Manage multiple PostgreSQL database connections (stored in local SQLite)
- View database schemas (tables, views, columns)
- Execute validated SQL queries (SELECT-only, read-only)
- Generate SQL from natural language using LLM integration

## Commands

### Backend (Rust)

```bash
# Build backend
cd backend && cargo build

# Run backend
cd backend && cargo run

# Run tests
cd backend && cargo test

# Build for production
cd backend && cargo build --release
```

### Frontend (React + TypeScript)

```bash
# Install dependencies
cd frontend && npm install

# Start development server
cd frontend && npm start

# Run tests
cd frontend && npm test

# Build for production
cd frontend && npm run build
```

### Environment Setup

Backend requires `.env` in `backend/` directory:
```
PORT=8080
SQLITE_DB_PATH=~/.db_query/db_query.db
LLM_API_KEY=your-api-key-here
LLM_API_URL=https://api.openai.com/v1/chat/completions
```

### Testing API Endpoints

Use the REST client file with curl or VSCode REST Client:
```bash
# Install VSCode REST Client extension and open fixtures/test.rest
# Or use curl:
curl http://localhost:8080/health
curl http://localhost:8080/api/v1/dbs
```

## Architecture

### Backend Architecture (Rust + Axum)

**Three-Layer Architecture:**

1. **API Layer** (`src/api/`): Axum route handlers
   - `databases.rs`: Database connection CRUD operations
   - `queries.rs`: SQL and natural language query execution
   - All handlers use shared state pattern (Arc-wrapped services)

2. **Service Layer** (`src/services/`): Business logic
   - `database_service.rs`: Manages database connections in SQLite
   - `schema_service.rs`: Fetches and caches PostgreSQL schema metadata
   - `query_executor.rs`: Executes SQL queries via SQLx, handles type conversion to JSON
   - `sql_validator.rs`: Validates SQL using sqlparser-rs, enforces SELECT-only, adds LIMIT 1000
   - `llm_service.rs`: Natural language to SQL conversion via OpenAI API

3. **Data Layer** (`src/db/`): Database initialization
   - `sqlite.rs`: Initializes SQLite database for storing connection strings and metadata

**Key Design Patterns:**
- **Shared State**: Services wrapped in `Arc<T>` and passed via Axum state
- **Error Handling**: Custom `AppError` enum with `IntoResponse` implementation for consistent error responses
- **Type Safety**: All models use strongly-typed structs with serde for JSON serialization (camelCase format enforced via serde rename)
- **Connection Pooling**: SQLx PgPool for PostgreSQL connections (created on-demand per query)

**Data Flow:**
1. User adds database via API → stored in SQLite with connection URL
2. User queries database → SQL validator ensures SELECT-only → SQLx executes → results serialized to JSON
3. Natural language query → LLM Service generates SQL using schema context → validator checks → executor runs

### Frontend Architecture (React + TypeScript)

**Component Structure:**
- `DatabaseList.tsx`: Lists all database connections
- `AddDatabaseForm.tsx`: Form for adding new databases
- `SchemaView.tsx`: Container for displaying database schema
  - `TableList.tsx`: Displays tables with columns
  - `ViewList.tsx`: Displays views with columns
- `QueryPanel.tsx`: Container for query execution
  - `SQLEditor.tsx`: Monaco editor for SQL input
  - `NaturalLanguageQuery.tsx`: Natural language input
- `QueryResults.tsx`: Displays query results in table format

**State Management**: Uses React state within components (no global state library)

**API Client**: Axios-based API client in `src/api/` with TypeScript types matching backend models

### Cross-Cutting Concerns

**Security:**
- Only SELECT statements allowed (enforced by sqlparser-rs)
- SQL injection prevention via parameterized queries (SQLx)
- No authentication (per requirements - any user can access)

**Performance:**
- Automatic LIMIT 1000 on queries without LIMIT clause
- Schema metadata cached in SQLite (avoid repeated metadata queries)
- Connection pooling for PostgreSQL connections

**Error Handling:**
- Backend: Custom `AppError` enum with structured JSON responses
- Frontend: Error states displayed in UI components
- All errors include descriptive messages for debugging

## Important Code Conventions

### Backend Rust Conventions

1. **Type Annotations**: All function signatures must have explicit return types
2. **Error Propagation**: Use `?` operator and custom `AppError` conversions
3. **Naming**:
   - Use snake_case for functions and variables
   - Use PascalCase for types and structs
   - Prefix shared state types with `Shared` (e.g., `SharedDatabaseService`)
4. **Serialization**: All API models use serde with `#[serde(rename_all = "camelCase")]` for JSON
5. **Builder Pattern**: Services use `new()` constructors, no builder pattern currently
6. **Function Size**: Keep functions focused and under 150 lines
7. **Logging**: Use `tracing` crate with appropriate levels (debug, info, error)

### Frontend TypeScript Conventions

1. **Type Safety**: All API responses and component props must have TypeScript interfaces
2. **Naming**: Use camelCase for variables and functions, PascalCase for components and types
3. **Component Structure**: Functional components with hooks (no class components)
4. **Styling**: Tailwind CSS for layout, Ant Design components for UI elements
5. **API Types**: Types should mirror backend models (camelCase format)

### Project-Specific Requirements

**From specs/instructions.md:**
- Backend and frontend both require strict type annotations
- All backend JSON responses MUST use camelCase format (enforced via serde)
- No authentication required (open access)
- SQL validation: Only SELECT statements, auto-add LIMIT 1000 if missing
- Database metadata stored in SQLite at `~/.db_query/db_query.db`
- Backend API supports CORS for all origins

## Common Development Workflows

### Adding a New API Endpoint

1. Add model in `backend/src/models/` with proper serde annotations
2. Add service method in appropriate service (`src/services/`)
3. Add route handler in `src/api/databases.rs` or `src/api/queries.rs`
4. Register route in `src/main.rs` router
5. Add corresponding TypeScript types in `frontend/src/types/`
6. Add API client function in `frontend/src/api/`
7. Update frontend components to use new endpoint

### Modifying Schema or Database Logic

Schema metadata is cached in SQLite. Key files:
- `backend/src/services/schema_service.rs`: Schema fetching and caching logic
- `backend/src/db/sqlite.rs`: SQLite schema definition
- Schema is fetched on-demand and cached with `last_updated` timestamp

### SQL Validation Changes

All SQL validation happens in `backend/src/services/sql_validator.rs`:
- Uses `sqlparser` crate with PostgreSQL dialect
- Validates only SELECT statements allowed
- Adds LIMIT 1000 if missing
- Returns descriptive validation errors

### LLM Integration

Natural language to SQL conversion in `backend/src/services/llm_service.rs`:
- Requires `LLM_API_KEY` environment variable
- Uses schema context (tables, columns, types) in prompt
- Supports OpenAI API format (can be swapped with compatible APIs)
- Generated SQL is validated before execution

## Testing Strategy

**Backend Tests:**
- Unit tests in `sql_validator.rs` for validation logic
- Integration tests deleted (previously in `backend/tests/`)
- Test coverage focuses on core business logic

**Frontend Tests:**
- Jest + React Testing Library
- Component tests in `components/__tests__/`
- Tests for AddDatabaseForm and QueryResults exist

**Manual Testing:**
- Use `fixtures/test.rest` for comprehensive API testing
- Includes 32 test scenarios covering all endpoints
- Tests error handling, validation, and integration workflows
