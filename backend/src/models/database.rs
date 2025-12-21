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
    #[must_use]
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

impl TryFrom<&rusqlite::Row<'_>> for DatabaseConnection {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            name: row.get(0)?,
            url: row.get(1)?,
            created_at: row.get(2)?,
            updated_at: row.get(3)?,
        })
    }
}
