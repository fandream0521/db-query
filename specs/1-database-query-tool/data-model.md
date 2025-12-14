# Data Model: Database Query Tool

**Feature ID:** 1-database-query-tool  
**Created:** 2025-12-14

## Overview

This document defines the data models for the database query tool, including entities stored in SQLite and data structures used in API communication.

## SQLite Storage Schema

### Table: `databases`

Stores database connection information.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | TEXT | PRIMARY KEY, NOT NULL | Unique name for the database connection |
| url | TEXT | NOT NULL | Database connection URL |
| created_at | TEXT | NOT NULL | ISO 8601 timestamp of creation |
| updated_at | TEXT | NOT NULL | ISO 8601 timestamp of last update |

**Indexes:**
- Primary key on `name`

**Validation Rules:**
- `name` must be non-empty and valid identifier
- `url` must be valid database connection URL format
- `created_at` and `updated_at` must be valid ISO 8601 timestamps

### Table: `schema_metadata`

Stores cached schema metadata for each database.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | INTEGER | PRIMARY KEY AUTOINCREMENT | Unique identifier |
| db_name | TEXT | NOT NULL, FOREIGN KEY | Reference to databases.name |
| table_name | TEXT | NOT NULL | Name of table or view |
| table_type | TEXT | NOT NULL | Either "table" or "view" |
| metadata_json | TEXT | NOT NULL | JSON string containing table/view metadata |
| updated_at | TEXT | NOT NULL | ISO 8601 timestamp of last update |

**Indexes:**
- Primary key on `id`
- Index on `db_name` for fast lookups
- Unique constraint on `(db_name, table_name, table_type)`

**Validation Rules:**
- `db_name` must reference existing database in `databases` table
- `table_type` must be either "table" or "view"
- `metadata_json` must be valid JSON
- `updated_at` must be valid ISO 8601 timestamp

**Metadata JSON Structure:**
```json
{
  "columns": [
    {
      "name": "column_name",
      "dataType": "varchar",
      "nullable": true,
      "defaultValue": null
    }
  ],
  "primaryKey": ["id"],
  "foreignKeys": [],
  "indexes": []
}
```

## API Data Models

### DatabaseConnection

Represents a database connection in API requests/responses.

```typescript
interface DatabaseConnection {
  name: string;           // Unique identifier
  url: string;            // Connection URL
  createdAt: string;      // ISO 8601 timestamp
  updatedAt: string;      // ISO 8601 timestamp
}
```

**Validation:**
- `name`: Required, non-empty string, valid identifier
- `url`: Required, valid database connection URL
- `createdAt`: Required, valid ISO 8601 timestamp
- `updatedAt`: Required, valid ISO 8601 timestamp

### CreateDatabaseRequest

Request body for creating/updating a database connection.

```typescript
interface CreateDatabaseRequest {
  url: string;            // Database connection URL
}
```

**Validation:**
- `url`: Required, valid database connection URL format

### SchemaMetadata

Represents schema metadata for a database.

```typescript
interface SchemaMetadata {
  dbName: string;         // Database connection name
  tables: TableInfo[];    // List of tables
  views: ViewInfo[];      // List of views
  updatedAt: string;      // ISO 8601 timestamp
}

interface TableInfo {
  name: string;          // Table name
  columns: ColumnInfo[]; // Column definitions
  primaryKey?: string[];  // Primary key columns
}

interface ViewInfo {
  name: string;          // View name
  columns: ColumnInfo[]; // Column definitions
}

interface ColumnInfo {
  name: string;          // Column name
  dataType: string;      // Data type (e.g., "varchar", "integer")
  nullable: boolean;     // Whether column allows NULL
  defaultValue?: string; // Default value if any
}
```

**Validation:**
- `dbName`: Required, must reference existing database
- `tables`: Array of valid TableInfo objects
- `views`: Array of valid ViewInfo objects
- `updatedAt`: Required, valid ISO 8601 timestamp

### QueryRequest

Request body for executing a SQL query.

```typescript
interface QueryRequest {
  sql: string;           // SQL SELECT statement
}
```

**Validation:**
- `sql`: Required, non-empty string
- Must be valid SQL syntax
- Must be SELECT statement only
- Will have LIMIT 1000 added automatically if missing

### NaturalLanguageQueryRequest

Request body for natural language query generation.

```typescript
interface NaturalLanguageQueryRequest {
  prompt: string;        // Natural language query description
}
```

**Validation:**
- `prompt`: Required, non-empty string

### QueryResponse

Response from query execution.

```typescript
interface QueryResponse {
  columns: string[];     // Column names
  rows: any[][];         // Result rows (array of arrays)
  rowCount: number;      // Number of rows returned
  executionTimeMs: number; // Query execution time in milliseconds
}
```

**Validation:**
- `columns`: Array of non-empty strings
- `rows`: Array of arrays, each inner array length matches columns length
- `rowCount`: Non-negative integer, equals rows.length
- `executionTimeMs`: Non-negative number

### ErrorResponse

Standard error response format.

```typescript
interface ErrorResponse {
  error: string;         // Error message
  code?: string;          // Error code (optional)
  details?: any;          // Additional error details (optional)
}
```

**Validation:**
- `error`: Required, non-empty string describing the error
- `code`: Optional, error code for programmatic handling
- `details`: Optional, additional context

## State Transitions

### Database Connection Lifecycle

1. **Created**: Connection URL provided, stored in SQLite
2. **Validated**: Connection tested successfully
3. **Schema Loaded**: Schema metadata retrieved and cached
4. **Active**: Ready for queries
5. **Stale**: Schema may be outdated (if DB schema changed)
6. **Invalid**: Connection URL no longer works

### Query Execution Flow

1. **Received**: Query request received (SQL or natural language)
2. **Validated**: SQL syntax and type validated
3. **Executed**: Query runs against database
4. **Completed**: Results returned successfully
5. **Failed**: Error occurred (validation or execution)

## Relationships

- One `DatabaseConnection` can have many `SchemaMetadata` entries (one per table/view)
- Each `SchemaMetadata` entry belongs to exactly one `DatabaseConnection`
- Queries are associated with a `DatabaseConnection` but not stored
- Query results are transient and not stored

## Notes

- All timestamps use ISO 8601 format (e.g., "2025-12-14T10:30:00Z")
- JSON property names use camelCase per constitution
- SQLite storage uses snake_case for column names (database convention)
- API uses camelCase for all JSON properties
- Schema metadata is cached and may become stale; consider refresh mechanism
- Connection URLs are stored as-is; no encryption (acceptable for local tool)

