# Database Query Tool

A web-based tool for querying PostgreSQL databases with support for direct SQL queries and natural language to SQL conversion.

## Features

- **Database Connection Management**: Add, view, and manage database connections
- **Schema Metadata Display**: View tables, views, and column information
- **Direct SQL Query Execution**: Execute SELECT queries with automatic validation and LIMIT handling
- **Natural Language to SQL**: Generate SQL queries from natural language descriptions using LLM
- **Query Results Display**: View query results in a responsive table with pagination

## Tech Stack

### Backend
- **Rust** with **Axum** web framework
- **SQLx** for PostgreSQL connections
- **rusqlite** for local SQLite storage
- **sqlparser-rs** for SQL validation
- **reqwest** for LLM API integration

### Frontend
- **React** with **TypeScript**
- **Ant Design** for UI components
- **Monaco Editor** for SQL editing
- **Tailwind CSS** for responsive styling
- **Axios** for API calls

## Prerequisites

- Rust (latest stable version)
- Node.js 18+ and npm
- PostgreSQL database (for target databases)
- LLM API key (OpenAI or compatible service) - optional, only needed for natural language queries

## Setup

### Backend Setup

1. Navigate to the backend directory:
```bash
cd backend
```

2. Create a `.env` file in the backend directory:
```env
PORT=8080
SQLITE_DB_PATH=~/.db_query/db_query.db
LLM_API_KEY=your-openai-api-key-here
LLM_API_URL=https://api.openai.com/v1/chat/completions
```

3. Build and run the backend:
```bash
cargo build
cargo run
```

The backend will start on `http://localhost:8080` by default.

### Frontend Setup

1. Navigate to the frontend directory:
```bash
cd frontend
```

2. Install dependencies:
```bash
npm install
```

3. Create a `.env` file (optional, defaults to `http://localhost:8080`):
```env
REACT_APP_API_URL=http://localhost:8080/api/v1
```

4. Start the development server:
```bash
npm start
```

The frontend will start on `http://localhost:3000` by default.

## Usage

### Adding a Database Connection

1. Enter a unique name for your database connection
2. Enter the PostgreSQL connection URL in the format:
   ```
   postgres://username:password@host:port/database
   ```
3. Click "Add Database"

### Viewing Schema

1. Select a database from the list
2. The schema (tables and views) will be automatically loaded and displayed
3. Click "Refresh" to reload the schema

### Executing SQL Queries

1. Select a database
2. Go to the "SQL Query" tab
3. Enter your SELECT query in the editor
4. Click "Execute Query" or press `Ctrl+Enter` (or `Cmd+Enter` on Mac)
5. Results will be displayed in a table below

**Note**: Only SELECT statements are allowed. The system automatically adds `LIMIT 1000` if your query doesn't have a LIMIT clause.

### Natural Language Queries

1. Select a database
2. Go to the "Natural Language" tab
3. Enter your question in natural language, e.g., "Show me all users from the users table"
4. Click "Generate & Execute Query"
5. The system will generate SQL, validate it, and execute it

**Note**: Natural language queries require an LLM API key to be configured.

## API Endpoints

- `GET /api/v1/dbs` - List all database connections
- `GET /api/v1/dbs/{name}` - Get database metadata and schema
- `PUT /api/v1/dbs/{name}` - Create or update a database connection
- `DELETE /api/v1/dbs/{name}` - Delete a database connection
- `POST /api/v1/dbs/{name}/query` - Execute a SQL query
- `POST /api/v1/dbs/{name}/query/natural` - Execute a natural language query

All endpoints return JSON responses in camelCase format.

## Project Structure

```
db-query/
├── backend/          # Rust backend
│   ├── src/
│   │   ├── api/      # API route handlers
│   │   ├── db/       # Database connection management
│   │   ├── models/   # Data models
│   │   ├── services/  # Business logic services
│   │   └── utils/    # Utility functions
│   └── Cargo.toml
├── frontend/         # React frontend
│   ├── src/
│   │   ├── api/      # API client functions
│   │   ├── components/  # React components
│   │   ├── types/    # TypeScript type definitions
│   │   └── utils/    # Utility functions
│   └── package.json
└── specs/            # Project specifications
```

## Development

### Running Tests

**Backend:**
```bash
cd backend
cargo test
```

**Frontend:**
```bash
cd frontend
npm test
```

### Building for Production

**Backend:**
```bash
cd backend
cargo build --release
```

**Frontend:**
```bash
cd frontend
npm run build
```

## Configuration

### Environment Variables

**Backend:**
- `PORT` - Server port (default: 8080)
- `SQLITE_DB_PATH` - Path to SQLite database file (default: ~/.db_query/db_query.db)
- `LLM_API_KEY` - API key for LLM service (required for natural language queries)
- `LLM_API_URL` - LLM API endpoint URL (default: OpenAI endpoint)

**Frontend:**
- `REACT_APP_API_URL` - Backend API URL (default: http://localhost:8080/api/v1)

## Limitations

- Only PostgreSQL databases are currently supported as target databases
- Only SELECT statements are allowed (read-only queries)
- Natural language queries require an LLM API key
- Schema metadata is cached and may become stale if the database schema changes

## License

MIT

