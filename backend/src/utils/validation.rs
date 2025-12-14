// Validation utilities

pub fn validate_database_url(url: &str) -> bool {
    // Basic validation for database connection URLs
    // Supports postgres://, postgresql://, etc.
    if url.is_empty() {
        return false;
    }
    
    url.starts_with("postgres://") 
        || url.starts_with("postgresql://")
        || url.starts_with("mysql://")
        || url.starts_with("sqlite://")
}

pub fn validate_database_name(name: &str) -> bool {
    // Validate database name (alphanumeric, dash, underscore)
    !name.is_empty() 
        && name.len() <= 100
        && name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_database_url() {
        assert!(validate_database_url("postgres://user:pass@localhost:5432/db"));
        assert!(validate_database_url("postgresql://user:pass@localhost:5432/db"));
        assert!(!validate_database_url("invalid://url"));
    }
}

