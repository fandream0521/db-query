use crate::error::AppError;
use sqlparser::ast::{Statement, Query, SetExpr};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

/// Validates SQL query and ensures it's a SELECT statement only
pub fn validate_sql(sql: &str) -> Result<String, AppError> {
    let dialect = PostgreSqlDialect {};
    
    // Parse SQL
    let statements = Parser::parse_sql(&dialect, sql)
        .map_err(|e| AppError::ValidationError(format!("Invalid SQL syntax: {}", e)))?;
    
    if statements.is_empty() {
        return Err(AppError::ValidationError("Empty SQL statement".to_string()));
    }
    
    if statements.len() > 1 {
        return Err(AppError::ValidationError(
            "Multiple statements not allowed. Please provide a single SELECT statement.".to_string(),
        ));
    }
    
    // Check if it's a SELECT statement
    match &statements[0] {
        Statement::Query(query) => {
            // Validate it's a SELECT query
            validate_select_query(query)?;
            
            // Add LIMIT if missing
            let sql_with_limit = add_limit_if_missing(sql, query)?;
            
            Ok(sql_with_limit)
        }
        _ => Err(AppError::ValidationError(
            "Only SELECT statements are allowed".to_string(),
        )),
    }
}

/// Validates that the query is a SELECT statement
fn validate_select_query(query: &Query) -> Result<(), AppError> {
    match query.body.as_ref() {
        SetExpr::Select(_) => Ok(()),
        SetExpr::Query(_) => {
            // Recursively check nested queries
            validate_select_query(query)
        }
        SetExpr::SetOperation { .. } => {
            // UNION, INTERSECT, EXCEPT are allowed if they're SELECT operations
            Ok(())
        }
        SetExpr::Values(_) => {
            // VALUES clause is allowed
            Ok(())
        }
        _ => Err(AppError::ValidationError(
            "Only SELECT statements are allowed".to_string(),
        )),
    }
}

/// Adds LIMIT 1000 if the query doesn't have a LIMIT clause
fn add_limit_if_missing(original_sql: &str, query: &Query) -> Result<String, AppError> {
    // Check if LIMIT already exists
    if query.limit.is_some() {
        return Ok(original_sql.to_string());
    }
    
    // Check if FETCH FIRST exists (SQL standard)
    if query.fetch.is_some() {
        return Ok(original_sql.to_string());
    }
    
    // Add LIMIT 1000 to the end of the query
    // Simple approach: append LIMIT if not present
    let sql_upper = original_sql.trim().to_uppercase();
    
    // Check if LIMIT or FETCH is already in the SQL string (case-insensitive)
    if sql_upper.contains("LIMIT") || sql_upper.contains("FETCH") {
        return Ok(original_sql.to_string());
    }
    
    // Append LIMIT 1000
    let mut result = original_sql.trim().to_string();
    
    // Remove trailing semicolon if present
    if result.ends_with(';') {
        result.pop();
        result = result.trim_end().to_string();
    }
    
    result.push_str(" LIMIT 1000");
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_select() {
        let sql = "SELECT * FROM users";
        assert!(validate_sql(sql).is_ok());
    }
    
    #[test]
    fn test_reject_insert() {
        let sql = "INSERT INTO users (name) VALUES ('test')";
        assert!(validate_sql(sql).is_err());
    }
    
    #[test]
    fn test_reject_update() {
        let sql = "UPDATE users SET name = 'test'";
        assert!(validate_sql(sql).is_err());
    }
    
    #[test]
    fn test_reject_delete() {
        let sql = "DELETE FROM users";
        assert!(validate_sql(sql).is_err());
    }
    
    #[test]
    fn test_add_limit() {
        let sql = "SELECT * FROM users";
        let result = validate_sql(sql).unwrap();
        assert!(result.contains("LIMIT 1000"));
    }
    
    #[test]
    fn test_preserve_existing_limit() {
        let sql = "SELECT * FROM users LIMIT 10";
        let result = validate_sql(sql).unwrap();
        assert_eq!(result, sql);
    }
}
