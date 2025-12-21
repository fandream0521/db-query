use rusqlite::{Connection, Result as SqliteResult};
use std::path::Path;

pub fn init_db(db_path: &str) -> SqliteResult<Connection> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = Path::new(db_path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| rusqlite::Error::InvalidPath(format!("Failed to create directory: {e}").into()))?;
    }

    let conn = Connection::open(db_path)?;
    
    // Create databases table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS databases (
            name TEXT PRIMARY KEY NOT NULL,
            url TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // Create schema_metadata table
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

    // Create index on db_name for faster lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_schema_metadata_db_name ON schema_metadata(db_name)",
        [],
    )?;

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_db() {
        let conn = init_db(":memory:").unwrap();
        // Verify tables exist
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'").unwrap();
        let tables: Vec<String> = stmt.query_map([], |row| row.get(0)).unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert!(tables.contains(&"databases".to_string()));
        assert!(tables.contains(&"schema_metadata".to_string()));
    }
}

