# Implementation Tasks: Database Query Tool

**Feature ID:** 1-database-query-tool  
**Created:** 2025-12-14  
**Last Updated:** 2025-12-14

## Overview

This document contains all implementation tasks organized by user story, enabling independent implementation and testing of each story.

## Task Summary

- **Total Tasks:** 90
- **Tasks by User Story:**
  - Setup: 12 tasks (T001-T012)
  - Foundational: 8 tasks (T013-T020)
  - US1: 14 tasks (T021-T034)
  - US2: 12 tasks (T035-T046)
  - US3: 16 tasks (T047-T062)
  - US4: 8 tasks (T063-T071)
  - US5: 6 tasks (T072-T077)
  - Polish: 13 tasks (T078-T090)

## Dependencies

```
Setup → Foundational → US1 → US2 → US3 → US4
                                    ↓
                                  US5
```

**Dependency Graph:**
- **Setup** (Phase 1): No dependencies
- **Foundational** (Phase 2): Depends on Setup
- **US1: Database Connection Management** (Phase 3): Depends on Foundational
- **US2: Schema Metadata Retrieval** (Phase 4): Depends on US1
- **US3: Direct SQL Query Execution** (Phase 5): Depends on US1, US2
- **US4: Natural Language to SQL** (Phase 6): Depends on US1, US2, US3
- **US5: Query Result Display** (Phase 7): Depends on US3
- **Polish** (Phase 8): Depends on all user stories

## Implementation Strategy

**MVP Scope:** US1, US2, US3, US5 (Core database query functionality without natural language)

**Incremental Delivery:**
1. **MVP**: Database connection, schema display, SQL query execution, result display
2. **Enhancement**: Natural language to SQL generation (US4)
3. **Polish**: Error handling, UI improvements, documentation

## Parallel Execution Opportunities

**Within US1:**
- Backend models and frontend types can be developed in parallel
- Backend API and frontend UI components can be developed in parallel after models/types

**Within US2:**
- Schema retrieval logic and caching can be developed in parallel with frontend display components

**Within US3:**
- SQL validation logic and query execution can be developed in parallel with frontend editor

**Within US4:**
- LLM integration and frontend natural language UI can be developed in parallel

---

## Phase 1: Setup

**Goal:** Initialize project structure and development environment

### Backend Setup

- [x] T001 Create backend directory structure (backend/src/, backend/tests/)
- [x] T002 Initialize Rust project with Cargo.toml in backend/
- [x] T003 Add dependencies to backend/Cargo.toml (axum, sqlx, rusqlite, sqlparser, serde, tower-http, etc.)
- [x] T004 Create backend/.env.example with required environment variables
- [x] T005 Create backend/src/main.rs with basic Axum server setup
- [x] T006 Create backend/src/lib.rs for library exports
- [x] T007 Create backend/src/config.rs for environment configuration
- [x] T008 Create backend/src/error.rs for error handling types

### Frontend Setup

- [x] T009 Create frontend directory structure (frontend/src/, frontend/public/)
- [x] T010 Initialize React TypeScript project in frontend/
- [x] T011 Install frontend dependencies (react, typescript, refine, antd, monaco-editor, tailwindcss, axios)
- [x] T012 Configure TypeScript strict mode in frontend/tsconfig.json

---

## Phase 2: Foundational

**Goal:** Implement core infrastructure and shared components required by all user stories

### Backend Foundation

- [x] T013 [P] Create backend/src/db/sqlite.rs for SQLite connection and initialization
- [x] T014 [P] Implement SQLite schema creation in backend/src/db/sqlite.rs (databases and schema_metadata tables)
- [x] T015 [P] Create backend/src/db/mod.rs for database module organization
- [x] T016 [P] Create backend/src/models/mod.rs for data models
- [x] T017 [P] Create backend/src/api/mod.rs for API route organization
- [x] T018 [P] Configure CORS middleware in backend/src/main.rs (allow all origins)
- [x] T019 [P] Create backend/src/utils/mod.rs for utility functions
- [x] T020 [P] Create backend/src/types.rs for shared type definitions

---

## Phase 3: User Story 1 - Database Connection Management

**Goal:** Users can add, view, and manage database connections. Connection information is stored in SQLite for reuse.

**Independent Test Criteria:** 
- User can add a database connection via API
- User can list all saved connections via API
- Connection information persists in SQLite
- Invalid connection URLs are rejected with clear error messages
- Frontend displays list of connections and allows adding new ones

**Backend Tasks:**

- [x] T021 [US1] Create backend/src/models/database.rs with DatabaseConnection struct and serde annotations (camelCase)
- [x] T022 [US1] Create backend/src/models/request.rs with CreateDatabaseRequest struct
- [x] T023 [US1] Implement database URL validation in backend/src/utils/validation.rs
- [x] T024 [US1] Create backend/src/services/database_service.rs with connection test function
- [x] T025 [US1] Implement database storage in backend/src/services/database_service.rs (SQLite operations)
- [x] T026 [US1] Create backend/src/api/databases.rs with GET /api/v1/dbs endpoint handler
- [x] T027 [US1] Create backend/src/api/databases.rs with PUT /api/v1/dbs/{name} endpoint handler
- [x] T028 [US1] Register database routes in backend/src/main.rs
- [x] T029 [US1] Create backend/src/error.rs with database-specific error types

**Frontend Tasks:**

- [x] T030 [P] [US1] Create frontend/src/types/database.ts with DatabaseConnection and CreateDatabaseRequest interfaces
- [x] T031 [P] [US1] Create frontend/src/api/database.ts with API client functions (listDatabases, upsertDatabase)
- [x] T032 [US1] Create frontend/src/components/DatabaseList.tsx to display saved connections
- [x] T033 [US1] Create frontend/src/components/AddDatabaseForm.tsx for adding new connections
- [x] T034 [US1] Integrate DatabaseList and AddDatabaseForm in frontend/src/App.tsx

---

## Phase 4: User Story 2 - Schema Metadata Retrieval and Display

**Goal:** System retrieves and displays database schema (tables and views). Schema metadata is cached in SQLite for reuse.

**Independent Test Criteria:**
- System retrieves tables and views from connected database
- Schema metadata is stored in SQLite cache
- Frontend displays schema information in organized manner
- System handles databases with no tables/views gracefully

**Backend Tasks:**

- [x] T035 [US2] Create backend/src/models/schema.rs with SchemaMetadata, TableInfo, ViewInfo, ColumnInfo structs (camelCase)
- [x] T036 [US2] Create backend/src/services/schema_service.rs for schema retrieval logic
- [x] T037 [US2] Implement PostgreSQL schema query in backend/src/services/schema_service.rs (information_schema queries)
- [x] T038 [US2] Implement schema caching in backend/src/services/schema_service.rs (store in SQLite)
- [x] T039 [US2] Create backend/src/api/databases.rs with GET /api/v1/dbs/{name} endpoint handler for metadata
- [x] T040 [US2] Add error handling for missing databases in backend/src/api/databases.rs

**Frontend Tasks:**

- [x] T041 [P] [US2] Create frontend/src/types/schema.ts with SchemaMetadata, TableInfo, ViewInfo, ColumnInfo interfaces
- [x] T042 [P] [US2] Create frontend/src/api/schema.ts with getSchemaMetadata API function
- [x] T043 [US2] Create frontend/src/components/SchemaView.tsx to display tables and views
- [x] T044 [US2] Create frontend/src/components/TableList.tsx to display table information
- [x] T045 [US2] Create frontend/src/components/ViewList.tsx to display view information
- [x] T046 [US2] Integrate SchemaView in frontend/src/App.tsx with database selection

---

## Phase 5: User Story 3 - Direct SQL Query Execution

**Goal:** Users can enter SQL queries directly and execute them. System validates SQL (SELECT only) and automatically adds LIMIT 1000 if missing.

**Independent Test Criteria:**
- User can input SQL in editor
- System validates SQL syntax and rejects non-SELECT statements
- System automatically adds LIMIT 1000 to queries without LIMIT
- Valid queries execute and return results
- Invalid SQL returns clear error messages

**Backend Tasks:**

- [ ] T047 [US3] Create backend/src/models/query.rs with QueryRequest, QueryResponse structs (camelCase)
- [ ] T048 [US3] Create backend/src/services/sql_validator.rs for SQL parsing and validation
- [ ] T049 [US3] Implement SELECT-only validation in backend/src/services/sql_validator.rs using sqlparser-rs
- [ ] T050 [US3] Implement automatic LIMIT addition in backend/src/services/sql_validator.rs
- [ ] T051 [US3] Create backend/src/services/query_executor.rs for executing queries against PostgreSQL
- [ ] T052 [US3] Implement query execution with connection pooling in backend/src/services/query_executor.rs
- [ ] T053 [US3] Create backend/src/api/queries.rs with POST /api/v1/dbs/{name}/query endpoint handler
- [ ] T054 [US3] Add error handling for invalid SQL and execution errors in backend/src/api/queries.rs
- [ ] T055 [US3] Register query routes in backend/src/main.rs

**Frontend Tasks:**

- [ ] T056 [P] [US3] Create frontend/src/types/query.ts with QueryRequest, QueryResponse interfaces
- [ ] T057 [P] [US3] Create frontend/src/api/query.ts with executeQuery API function
- [ ] T058 [US3] Create frontend/src/components/SQLEditor.tsx using Monaco Editor
- [ ] T059 [US3] Create frontend/src/components/QueryPanel.tsx with editor and execute button
- [ ] T060 [US3] Create frontend/src/components/QueryResults.tsx for displaying results (basic structure)
- [ ] T061 [US3] Integrate QueryPanel in frontend/src/App.tsx with database and schema context
- [ ] T062 [US3] Add error display for SQL validation errors in frontend/src/components/QueryPanel.tsx

---

## Phase 6: User Story 4 - Natural Language to SQL Query Generation

**Goal:** Users can generate SQL queries from natural language descriptions. System uses schema metadata as context for LLM.

**Independent Test Criteria:**
- User can input natural language query
- System generates valid SQL from natural language
- Generated SQL follows same validation rules as direct SQL
- System handles ambiguous queries with appropriate feedback

**Backend Tasks:**

- [ ] T063 [US4] Create backend/src/models/natural_language.rs with NaturalLanguageQueryRequest struct
- [ ] T064 [US4] Create backend/src/services/llm_service.rs for LLM API integration
- [ ] T065 [US4] Implement natural language to SQL conversion in backend/src/services/llm_service.rs (format schema as context, call LLM API)
- [ ] T066 [US4] Create backend/src/api/queries.rs with POST /api/v1/dbs/{name}/query/natural endpoint handler
- [ ] T067 [US4] Add error handling for LLM API failures in backend/src/api/queries.rs

**Frontend Tasks:**

- [ ] T068 [P] [US4] Create frontend/src/types/natural_language.ts with NaturalLanguageQueryRequest interface
- [ ] T069 [P] [US4] Create frontend/src/api/natural_language.ts with executeNaturalLanguageQuery API function
- [ ] T070 [US4] Create frontend/src/components/NaturalLanguageQuery.tsx with input field and generate button
- [ ] T071 [US4] Integrate NaturalLanguageQuery in frontend/src/components/QueryPanel.tsx

---

## Phase 7: User Story 5 - Query Result Display

**Goal:** Query results are displayed in a clear, readable tabular format with proper handling of large result sets.

**Independent Test Criteria:**
- Results display in tabular format with column headers
- Large result sets are handled appropriately (pagination or scrolling)
- Empty result sets are clearly indicated
- Result display is responsive and usable

**Frontend Tasks:**

- [ ] T072 [US5] Enhance frontend/src/components/QueryResults.tsx with Ant Design Table component
- [ ] T073 [US5] Implement column header display in frontend/src/components/QueryResults.tsx
- [ ] T074 [US5] Implement pagination for large result sets in frontend/src/components/QueryResults.tsx
- [ ] T075 [US5] Add empty state display in frontend/src/components/QueryResults.tsx
- [ ] T076 [US5] Add responsive styling to frontend/src/components/QueryResults.tsx using Tailwind CSS
- [ ] T077 [US5] Display execution time in frontend/src/components/QueryResults.tsx

---

## Final Phase: Polish & Cross-Cutting Concerns

**Goal:** Improve error handling, user experience, and code quality

### Error Handling

- [ ] T078 [P] Create consistent error response format in backend/src/error.rs (ErrorResponse struct)
- [ ] T079 [P] Implement error handling middleware in backend/src/api/middleware.rs
- [ ] T080 [P] Create frontend/src/utils/error.ts for error handling utilities
- [ ] T081 [P] Add error toast notifications in frontend using Ant Design message component

### UI/UX Improvements

- [ ] T082 [P] Add loading states for all async operations in frontend components
- [ ] T083 [P] Improve form validation and user feedback in frontend forms
- [ ] T084 [P] Add keyboard shortcuts for common actions (e.g., Ctrl+Enter to execute query)

### Documentation

- [ ] T085 [P] Create README.md with setup and usage instructions
- [ ] T086 [P] Add code comments and documentation for complex functions
- [ ] T087 [P] Update API documentation in contracts/openapi.yaml if needed

### Testing

- [ ] T088 [P] Write unit tests for backend services (database_service, schema_service, sql_validator)
- [ ] T089 [P] Write integration tests for API endpoints
- [ ] T090 [P] Write frontend component tests for key components

---

## Notes

- All backend JSON responses must use camelCase per constitution
- All TypeScript interfaces must match backend structs exactly
- SQLite storage path (~/.db_query/) should be created on first run
- LLM API key must be configured via environment variable
- All tasks should be completed in dependency order
- Parallel tasks [P] can be worked on simultaneously by different developers
- Story labels [US1], [US2], etc. indicate which user story the task belongs to

