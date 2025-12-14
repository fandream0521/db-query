# Agent Context: cursor-agent

**Last Updated:** 2025-12-14

## Technology Stack

<!-- AUTO-GENERATED START: Technology Stack -->
**Frontend:**
- React (latest stable)
- TypeScript (strict mode)
- Refine5 (data management framework)
- Tailwind CSS (styling)
- Ant Design (UI components)
- Monaco Editor (SQL editor)

**Backend:**
- Rust (latest stable)
- Axum (web framework)
- SQLx (PostgreSQL driver with compile-time checking)
- rusqlite (SQLite driver for local storage)
- sqlparser-rs (SQL parsing and validation)
- serde (JSON serialization with camelCase)
- tower-http (CORS middleware)
- anyhow (error handling)
- reqwest (HTTP client for LLM API)

**Databases:**
- PostgreSQL (target database for queries)
- SQLite (local storage at ~/.db_query/db_query.db)

**External Services:**
- LLM API (OpenAI or compatible) for natural language to SQL conversion
<!-- AUTO-GENERATED END: Technology Stack -->

## Project-Specific Patterns

<!-- AUTO-GENERATED START: Patterns -->
**JSON Format:**
- All API responses use camelCase property names (enforced via serde `#[serde(rename_all = "camelCase")]`)

**SQL Validation:**
- Only SELECT statements allowed
- Automatic LIMIT 1000 addition if missing
- Syntax validation via sqlparser-rs before execution

**Error Handling:**
- Use anyhow for error propagation
- Convert to appropriate HTTP status codes
- Return ErrorResponse with clear messages

**Database Storage:**
- Connection info stored in SQLite at ~/.db_query/db_query.db
- Schema metadata cached in SQLite for performance
- Timestamps use ISO 8601 format

**CORS:**
- Allow all origins (no authentication required per constitution)
- Configure via tower-http CORS middleware

**Type Safety:**
- Frontend: TypeScript strict mode
- Backend: Rust's type system with explicit types for all API structures
<!-- AUTO-GENERATED END: Patterns -->

## Manual Additions

<!-- Manual additions below this line will be preserved -->

