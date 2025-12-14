use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseConnection {
    pub name: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
}

impl DatabaseConnection {
    #[allow(dead_code)]
    pub fn new(name: String, url: String) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            name,
            url,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    #[allow(dead_code)]
    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now().to_rfc3339();
    }
}

impl From<&rusqlite::Row<'_>> for DatabaseConnection {
    fn from(row: &rusqlite::Row) -> Self {
        Self {
            name: row.get(0).unwrap(),
            url: row.get(1).unwrap(),
            created_at: row.get(2).unwrap(),
            updated_at: row.get(3).unwrap(),
        }
    }
}
