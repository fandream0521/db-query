# Research Findings: Database Query Tool

**Feature ID:** 1-database-query-tool  
**Created:** 2025-12-14

## Technology Decisions

### Rust Web Framework

**Decision:** Use Axum web framework

**Rationale:**
- Modern async/await support with excellent performance
- Built on tokio and tower ecosystem
- Type-safe routing and middleware
- Good ecosystem compatibility
- Simpler API compared to Actix-web for this use case

**Alternatives considered:**
- Actix-web: More mature but heavier, more complex API
- Warp: Functional style, less intuitive for some developers

### PostgreSQL Driver

**Decision:** Use `sqlx` with PostgreSQL support

**Rationale:**
- Compile-time SQL checking
- Async/await support
- Connection pooling built-in
- Type-safe query building
- Good documentation and community support

**Alternatives considered:**
- tokio-postgres: Lower level, more manual connection management
- diesel: ORM approach, overkill for this use case

### SQL Parsing and Validation

**Decision:** Use `sqlparser-rs` crate

**Rationale:**
- Comprehensive SQL parsing support
- Can parse and validate SELECT statements
- Ability to detect non-SELECT operations
- Can modify AST to add LIMIT clauses
- Well-maintained and actively developed

**Alternatives considered:**
- Custom parser: Too complex and error-prone
- Other SQL parsers: Less comprehensive or less maintained

### LLM Integration

**Decision:** Use OpenAI API (configurable to support other providers)

**Rationale:**
- Widely available and well-documented
- Good performance for SQL generation tasks
- Can be configured via environment variables
- Support for streaming responses if needed
- Alternative providers (Anthropic, local models) can be added later

**Alternatives considered:**
- Anthropic Claude: Good alternative, can be added as option
- Local LLM models: Lower quality, more setup complexity
- Specialized SQL generation models: May be considered for future optimization

### CORS Configuration

**Decision:** Use `tower-http` CORS middleware with allow-all origins

**Rationale:**
- Standard middleware for Axum
- Easy configuration for development
- Can be restricted later if needed
- No authentication required per constitution

**Alternatives considered:**
- Custom CORS handling: Unnecessary complexity
- No CORS: Would break frontend integration

### SQLite Storage Schema

**Decision:** Use rusqlite with schema:
- `databases` table: name, url, created_at, updated_at
- `schema_metadata` table: db_name, table_name, table_type (table/view), metadata_json, updated_at

**Rationale:**
- Simple key-value storage for connections
- JSON storage for flexible metadata schema
- Easy to query and update
- Minimal schema complexity

**Alternatives considered:**
- Full relational schema: Overkill for simple metadata storage
- File-based storage: Less reliable, harder to query

### JSON Serialization

**Decision:** Use `serde` with `#[serde(rename_all = "camelCase")]` for all API responses

**Rationale:**
- Standard Rust serialization library
- Compile-time guarantees for camelCase conversion
- Consistent with TypeScript frontend expectations
- Easy to configure per-struct basis

**Alternatives considered:**
- Manual field renaming: Error-prone
- Runtime conversion: Less efficient, more complex

### Error Handling

**Decision:** Use `anyhow` for error handling with custom error types

**Rationale:**
- Good for application-level error handling
- Easy error propagation
- Can convert to appropriate HTTP status codes
- Clear error messages for API responses

**Alternatives considered:**
- `thiserror`: More boilerplate, better for library code
- Custom error types: More work, similar benefits

### Environment Configuration

**Decision:** Use `dotenv` and environment variables

**Rationale:**
- Standard approach for configuration
- Easy to manage different environments
- Secure for API keys (LLM API key)
- Simple to implement

**Alternatives considered:**
- Config files: Less flexible, harder to manage secrets
- Hardcoded values: Not secure or flexible

## Best Practices

### Database Connection Management
- Use connection pooling (sqlx Pool)
- Set reasonable connection limits
- Handle connection errors gracefully
- Implement connection retry logic for transient failures

### SQL Validation
- Parse SQL before execution
- Check for SELECT-only statements
- Validate syntax thoroughly
- Add LIMIT automatically if missing
- Return clear error messages for invalid SQL

### Schema Metadata Caching
- Cache metadata in SQLite after first retrieval
- Allow manual refresh if schema changes
- Store metadata as JSON for flexibility
- Include timestamp for cache invalidation decisions

### Natural Language to SQL
- Include full schema context in LLM prompt
- Validate generated SQL before execution
- Provide user option to review generated SQL
- Handle ambiguous queries with clarification requests
- Set clear expectations in prompts (SELECT only, include LIMIT)

### API Design
- Use RESTful conventions
- Consistent error response format
- Proper HTTP status codes
- Clear endpoint naming (/api/v1/dbs/{name}/query)
- Support both direct SQL and natural language queries

## Integration Patterns

### LLM API Integration Pattern
1. Receive natural language query
2. Retrieve schema metadata from SQLite
3. Format schema as context for LLM
4. Send prompt to LLM API
5. Receive generated SQL
6. Validate SQL (same as direct input)
7. Execute if valid, return error if invalid

### Database Connection Pattern
1. User provides connection URL
2. Validate URL format
3. Test connection (connect and disconnect quickly)
4. Store connection info in SQLite
5. On first use, retrieve and cache schema metadata
6. Reuse cached metadata for subsequent queries

### Query Execution Pattern
1. Receive SQL query (direct or generated)
2. Parse SQL with sqlparser-rs
3. Validate: SELECT only, syntax correct
4. Add LIMIT 1000 if missing
5. Execute against target database
6. Convert results to JSON (camelCase)
7. Return to frontend

## Notes

- All decisions align with project constitution
- Technology choices prioritize type safety and developer experience
- LLM API choice is flexible and can be changed via configuration
- SQLite storage location (~/.db_query/) should be created on first run
- Consider adding connection health checks in future iterations

