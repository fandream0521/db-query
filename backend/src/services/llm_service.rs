use crate::error::AppError;
use crate::models::schema::SchemaMetadata;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct LLMService {
    api_key: String,
    api_url: String,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f64,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessageResponse,
}

#[derive(Debug, Deserialize)]
struct ChatMessageResponse {
    content: String,
}

impl LLMService {
    pub fn new(api_key: String, api_url: String) -> Self {
        Self { api_key, api_url }
    }

    /// Convert natural language query to SQL using LLM
    pub async fn natural_language_to_sql(
        &self,
        prompt: &str,
        schema: &SchemaMetadata,
    ) -> Result<String, AppError> {
        if self.api_key.is_empty() {
            return Err(AppError::InternalError(
                "LLM API key not configured. Please set LLM_API_KEY environment variable.".to_string(),
            ));
        }

        // Format schema as context
        let schema_context = Self::format_schema_context(schema);

        // Create prompt for LLM
        let system_prompt = r#"You are a SQL expert. Convert natural language queries to PostgreSQL SELECT statements.

Rules:
1. Only generate SELECT statements (read-only queries)
2. Use the provided schema information to determine table and column names
3. Return ONLY the SQL query, no explanations or markdown formatting
4. Use proper PostgreSQL syntax
5. Include appropriate WHERE clauses, JOINs, and aggregations as needed
6. Do not include LIMIT clauses (the system will add them automatically)

Schema Information:
"#;

        let user_prompt = format!("{}\n\nUser Query: {}", schema_context, prompt);

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_prompt,
            },
        ];

        let request = ChatRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages,
            temperature: 0.3,
        };

        // Make HTTP request to LLM API
        let client = reqwest::Client::new();
        let response = client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::InternalError(format!("LLM API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::InternalError(format!(
                "LLM API returned error {}: {}",
                status, error_text
            )));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to parse LLM response: {}", e)))?;

        if chat_response.choices.is_empty() {
            return Err(AppError::InternalError(
                "LLM API returned no choices".to_string(),
            ));
        }

        let sql = chat_response.choices[0]
            .message
            .content
            .trim()
            .to_string();

        // Remove markdown code blocks if present
        let sql = sql
            .trim_start_matches("```sql")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim()
            .to_string();

        if sql.is_empty() {
            return Err(AppError::InternalError(
                "LLM did not generate a valid SQL query".to_string(),
            ));
        }

        Ok(sql)
    }

    /// Format schema metadata as a readable context string for LLM
    fn format_schema_context(schema: &SchemaMetadata) -> String {
        let mut context = String::new();

        context.push_str(&format!("Database: {}\n\n", schema.db_name));

        if !schema.tables.is_empty() {
            context.push_str("Tables:\n");
            for table in &schema.tables {
                context.push_str(&format!("  - {} (", table.name));
                if let Some(ref pk) = table.primary_key {
                    context.push_str(&format!("PK: {:?}, ", pk));
                }
                context.push_str("columns: ");
                let column_names: Vec<String> = table
                    .columns
                    .iter()
                    .map(|c| format!("{} ({})", c.name, c.data_type))
                    .collect();
                context.push_str(&column_names.join(", "));
                context.push_str(")\n");
            }
        }

        if !schema.views.is_empty() {
            context.push_str("\nViews:\n");
            for view in &schema.views {
                context.push_str(&format!("  - {} (columns: ", view.name));
                let column_names: Vec<String> = view
                    .columns
                    .iter()
                    .map(|c| format!("{} ({})", c.name, c.data_type))
                    .collect();
                context.push_str(&column_names.join(", "));
                context.push_str(")\n");
            }
        }

        context
    }
}
