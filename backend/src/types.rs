// Shared type definitions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timestamp {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[allow(dead_code)]
impl Timestamp {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self) {
        self.updated_at = Utc::now();
    }
}

#[allow(dead_code)]
impl Default for Timestamp {
    fn default() -> Self {
        Self::new()
    }
}

