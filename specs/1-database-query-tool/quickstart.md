# Quick Start Guide: Database Query Tool

**Feature ID:** 1-database-query-tool

## Overview

This guide helps developers get started with implementing the database query tool feature. It covers setup, architecture overview, and key implementation patterns.

## Prerequisites

- Rust (latest stable version)
- Node.js and npm/yarn (for frontend)
- PostgreSQL database (for testing)
- LLM API key (OpenAI or compatible)

## Project Structure

```
db-query/
├── backend/          # Rust backend
│   ├── src/
│   ├── Cargo.toml
│   └── .env
├── frontend/         # React/TypeScript frontend
│   ├── src/
│   ├── package.json
│   └── tsconfig.json
└── specs/            # Specifications and documentation
```

## Backend Setup

### 1. Initialize Rust Project

```bash
cd backend
cargo init
```

### 2. Add Dependencies to Cargo.toml

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls"] }
rusqlite = { version = "0.30", features = ["bundled"] }
sqlparser = "0.39"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tower-http = { version = "0.5", features = ["cors"] }
dotenv = "0.15"
anyhow = "1.0"
reqwest = { version = "0.11", features = ["json"] }
chrono = { version = "0.4", features = ["serde"] }
```

### 3. Environment Configuration

Create `.env` file:

```env
DATABASE_URL=postgres://user:password@localhost:5432/dbname
SQLITE_DB_PATH=~/.db_query/db_query.db
LLM_API_KEY=your-api-key-here
LLM_API_URL=https://api.openai.com/v1/chat/completions
PORT=8080
```

### 4. Initialize SQLite Database

Create migration script or initialize on first run:

```rust
// Initialize SQLite schema
let conn = rusqlite::Connection::open(db_path)?;
conn.execute(
    "CREATE TABLE IF NOT EXISTS databases (
        name TEXT PRIMARY KEY NOT NULL,
        url TEXT NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL
    )",
    [],
)?;
conn.execute(
    "CREATE TABLE IF NOT EXISTS schema_metadata (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        db_name TEXT NOT NULL,
        table_name TEXT NOT NULL,
        table_type TEXT NOT NULL,
        metadata_json TEXT NOT NULL,
        updated_at TEXT NOT NULL,
        FOREIGN KEY (db_name) REFERENCES databases(name),
        UNIQUE(db_name, table_name, table_type)
    )",
    [],
)?;
```

## Frontend Setup

### 1. Initialize React Project

```bash
cd frontend
npx create-react-app . --template typescript
```

### 2. Install Dependencies

```bash
npm install @refinedev/core @refinedev/simple-rest
npm install antd
npm install @monaco-editor/react
npm install tailwindcss postcss autoprefixer
npm install axios
```

### 3. Configure Tailwind

```bash
npx tailwindcss init -p
```

Update `tailwind.config.js`:

```javascript
module.exports = {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

### 4. API Client Setup

Create `src/api/client.ts`:

```typescript
import axios from 'axios';

const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:8080/api/v1';

export const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});
```

## Key Implementation Patterns

### Backend: Database Connection Management

```rust
// Store connection
async fn upsert_database(
    name: String,
    url: String,
) -> Result<DatabaseConnection> {
    // Validate URL format
    // Test connection
    // Store in SQLite
    // Return connection info
}
```

### Backend: Schema Metadata Retrieval

```rust
async fn get_schema_metadata(
    db_name: &str,
) -> Result<SchemaMetadata> {
    // Check SQLite cache first
    // If not cached or stale, connect to target DB
    // Query information_schema for tables/views
    // Store in SQLite cache
    // Return metadata
}
```

### Backend: SQL Validation

```rust
fn validate_sql(sql: &str) -> Result<()> {
    // Parse SQL with sqlparser
    // Check it's a SELECT statement
    // Validate syntax
    // Add LIMIT if missing
    // Return validated SQL
}
```

### Backend: Natural Language to SQL

```rust
async fn generate_sql_from_natural_language(
    prompt: &str,
    schema: &SchemaMetadata,
) -> Result<String> {
    // Format schema as context
    // Call LLM API with prompt
    // Extract SQL from response
    // Validate generated SQL
    // Return SQL
}
```

### Frontend: Database List Component

```typescript
const DatabaseList: React.FC = () => {
  const { data, isLoading } = useList<DatabaseConnection>({
    resource: 'dbs',
  });

  return (
    <List>
      {data?.data.map((db) => (
        <List.Item key={db.name}>
          {db.name} - {db.url}
        </List.Item>
      ))}
    </List>
  );
};
```

### Frontend: SQL Editor Component

```typescript
import Editor from '@monaco-editor/react';

const SQLEditor: React.FC = () => {
  const [sql, setSql] = useState('');

  return (
    <Editor
      height="400px"
      language="sql"
      value={sql}
      onChange={(value) => setSql(value || '')}
      theme="vs-dark"
    />
  );
};
```

## API Integration Examples

### Add Database Connection

```typescript
const addDatabase = async (name: string, url: string) => {
  const response = await apiClient.put(`/dbs/${name}`, { url });
  return response.data;
};
```

### Execute SQL Query

```typescript
const executeQuery = async (dbName: string, sql: string) => {
  const response = await apiClient.post(`/dbs/${dbName}/query`, { sql });
  return response.data;
};
```

### Natural Language Query

```typescript
const naturalLanguageQuery = async (dbName: string, prompt: string) => {
  const response = await apiClient.post(
    `/dbs/${dbName}/query/natural`,
    { prompt }
  );
  return response.data;
};
```

## Testing

### Backend Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_validate_sql() {
        assert!(validate_sql("SELECT * FROM users").is_ok());
        assert!(validate_sql("INSERT INTO users VALUES (1)").is_err());
    }
}
```

### Frontend Tests

```typescript
import { render, screen } from '@testing-library/react';
import DatabaseList from './DatabaseList';

test('renders database list', () => {
  render(<DatabaseList />);
  // Test implementation
});
```

## Development Workflow

1. **Start Backend:**
   ```bash
   cd backend
   cargo run
   ```

2. **Start Frontend:**
   ```bash
   cd frontend
   npm start
   ```

3. **Test API:**
   ```bash
   curl -X PUT http://localhost:8080/api/v1/dbs/test-db \
     -H "Content-Type: application/json" \
     -d '{"url": "postgres://user:pass@localhost:5432/dbname"}'
   ```

## Common Issues

### SQLite Path Resolution

On Windows, `~/.db_query/` needs to be resolved to user home directory:

```rust
let home = std::env::var("HOME")
    .or_else(|_| std::env::var("USERPROFILE"))?;
let db_path = format!("{}/.db_query/db_query.db", home);
```

### CORS Configuration

Ensure CORS allows all origins:

```rust
let cors = tower_http::cors::CorsLayer::new()
    .allow_origin(tower_http::cors::Any)
    .allow_methods([Method::GET, Method::POST, Method::PUT])
    .allow_headers([header::CONTENT_TYPE]);
```

### LLM API Integration

Handle API errors gracefully:

```rust
async fn call_llm_api(prompt: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let response = client
        .post(&env::var("LLM_API_URL")?)
        .header("Authorization", format!("Bearer {}", env::var("LLM_API_KEY")?))
        .json(&json!({
            "model": "gpt-4",
            "messages": [{"role": "user", "content": prompt}]
        }))
        .send()
        .await?;
    
    // Parse response and extract SQL
}
```

## Next Steps

1. Implement database connection management
2. Implement schema metadata retrieval
3. Implement SQL validation and execution
4. Implement natural language to SQL conversion
5. Build frontend UI components
6. Integrate frontend with backend API
7. Add error handling and user feedback
8. Write tests for all components

## Resources

- [Axum Documentation](https://docs.rs/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Refine Documentation](https://refine.dev/docs/)
- [Monaco Editor](https://microsoft.github.io/monaco-editor/)
- [OpenAPI Specification](https://swagger.io/specification/)

