# Implementation Plan: Database Query Tool

**Feature ID:** 1-database-query-tool  
**Status:** Draft  
**Created:** 2025-12-14  
**Last Updated:** 2025-12-14

## Technical Context

### Technology Stack

**Frontend:**
- React (latest stable)
- TypeScript (strict mode)
- Refine5 (data management framework)
- Tailwind CSS (styling)
- Ant Design (UI components)
- Monaco Editor (SQL editor)

**Backend:**
- Rust (latest stable)
- Actix-web or Axum (web framework)
- SQLx or tokio-postgres (PostgreSQL driver)
- rusqlite (SQLite driver for local storage)
- sqlparser-rs (SQL parsing and validation)
- LLM API integration (for natural language to SQL)

**Database:**
- PostgreSQL (target database for queries)
- SQLite (local storage at ~/.db_query/db_query.db)

**Other Dependencies:**
- CORS middleware (allow all origins)
- JSON serialization (serde with camelCase)
- Environment configuration management

### Architecture Overview

The system follows a client-server architecture:

1. **Frontend (React/TypeScript)**: 
   - Provides UI for database connection management
   - Displays schema metadata (tables/views)
   - SQL query editor with Monaco
   - Natural language query input
   - Results table display

2. **Backend (Rust)**:
   - RESTful API server
   - Database connection pool management
   - Schema metadata retrieval and caching
   - SQL validation and execution
   - Natural language to SQL conversion via LLM
   - Local SQLite storage for connections and metadata

3. **Data Flow**:
   - User adds DB connection → stored in SQLite
   - System connects to target DB → retrieves schema → stores in SQLite
   - User queries → validated → executed → results returned as JSON
   - Natural language → LLM generates SQL → validated → executed

### Integration Points

- **PostgreSQL databases**: Primary target for queries
- **LLM API**: For natural language to SQL conversion (OpenAI, Anthropic, or similar)
- **SQLite local storage**: Persistent storage at ~/.db_query/db_query.db

### Known Constraints

- Only SELECT statements allowed (read-only operations)
- Automatic LIMIT 1000 for queries without LIMIT clause
- CORS must allow all origins (no authentication)
- Initial focus on PostgreSQL-compatible databases
- Schema metadata cached in SQLite (may become stale if DB schema changes)
- Natural language queries depend on LLM API availability

## Constitution Check

### Principle 1: TypeScript Frontend
- [x] **PASS**: Frontend will be implemented using React with TypeScript in strict mode. All frontend code will have TypeScript type annotations.

### Principle 2: Strict Type Annotations
- [x] **PASS**: 
  - Frontend: TypeScript strict mode ensures all code has type annotations
  - Backend: Rust's type system provides compile-time type safety. All API responses and data structures will have explicit type definitions.

### Principle 3: JSON camelCase Format
- [x] **PASS**: All backend JSON responses will use camelCase property names. This will be enforced through serde serialization configuration (e.g., `#[serde(rename_all = "camelCase")]`).

### Principle 4: No Authentication Required
- [x] **PASS**: The API will not require any authentication. CORS will be configured to allow all origins. All endpoints will be publicly accessible.

## Gates

### Gate 1: Technical Feasibility
- [x] **Status:** PASS
- [x] **Notes:** 
  - All required technologies are mature and well-documented
  - Rust has excellent PostgreSQL and SQLite support
  - SQL parsing libraries (sqlparser-rs) are available
  - LLM APIs for natural language processing are accessible
  - No significant technical blockers identified

### Gate 2: Constitution Compliance
- [x] **Status:** PASS
- [x] **Notes:** 
  - All four constitutional principles are satisfied
  - TypeScript frontend with strict types
  - Rust backend with strong typing
  - camelCase JSON format will be enforced
  - No authentication required as specified

### Gate 3: Dependencies Resolved
- [x] **Status:** PASS
- [x] **Notes:** 
  - All dependencies are available and well-maintained
  - LLM API integration requires API key configuration (environment variable)
  - SQLite storage path (~/.db_query/) needs to be created if it doesn't exist
  - No blocking dependencies identified

## Phase 0: Research

### Research Tasks

Research completed. Key decisions:
- Web framework: Axum
- PostgreSQL driver: sqlx
- SQL parsing: sqlparser-rs
- LLM integration: OpenAI API (configurable)
- CORS: tower-http with allow-all origins
- SQLite schema: Simple tables for connections and metadata
- JSON serialization: serde with camelCase

**Output:** `research.md` ✅

## Phase 1: Design

### Data Model

Data model defined with:
- SQLite storage schema (databases, schema_metadata tables)
- API data structures (TypeScript interfaces)
- Validation rules and constraints
- State transitions and relationships

**Output:** `data-model.md` ✅

### API Contracts

API contracts defined with OpenAPI 3.0.3 specification:
- GET /api/v1/dbs - List all databases
- GET /api/v1/dbs/{name} - Get database metadata
- PUT /api/v1/dbs/{name} - Create/update database connection
- POST /api/v1/dbs/{name}/query - Execute SQL query
- POST /api/v1/dbs/{name}/query/natural - Execute natural language query

All endpoints use camelCase JSON format per constitution.

**Output:** `contracts/openapi.yaml` ✅

### Quick Start Guide

Quick start guide created with:
- Project structure overview
- Backend and frontend setup instructions
- Key implementation patterns and code examples
- API integration examples
- Testing guidelines
- Common issues and solutions

**Output:** `quickstart.md` ✅

## Phase 2: Implementation

### Implementation Phases

**Phase 2.1: Backend Core**
- Database connection management (SQLite storage)
- Schema metadata retrieval and caching
- SQL validation and execution
- Basic API endpoints

**Phase 2.2: Natural Language Integration**
- LLM API integration
- Natural language to SQL conversion
- Query generation endpoint

**Phase 2.3: Frontend Core**
- Database connection UI
- Schema metadata display
- SQL editor with Monaco
- Query results table

**Phase 2.4: Frontend Integration**
- Natural language query interface
- API integration
- Error handling and user feedback
- Responsive design

**Phase 2.5: Testing & Polish**
- Unit tests for backend
- Integration tests for API
- Frontend component tests
- End-to-end testing
- Documentation updates

## Notes

- All API endpoints follow RESTful conventions
- CORS configured to allow all origins (no authentication)
- SQLite storage location (~/.db_query/) created on first run
- Schema metadata caching improves performance but may become stale
- LLM API key required via environment variable
- Initial focus on PostgreSQL, extensible to other databases
- All JSON responses use camelCase per constitution
- Type safety enforced at both frontend (TypeScript) and backend (Rust) levels
