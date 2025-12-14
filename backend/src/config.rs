use std::env;
use dotenv::dotenv;

pub struct Config {
    #[allow(dead_code)]
    pub database_url: String,
    pub sqlite_db_path: String,
    #[allow(dead_code)]
    pub llm_api_key: String,
    #[allow(dead_code)]
    pub llm_api_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();

        // Resolve SQLite path (handle ~ expansion)
        let sqlite_path = env::var("SQLITE_DB_PATH")
            .unwrap_or_else(|_| "~/.db_query/db_query.db".to_string());
        
        let sqlite_db_path = if sqlite_path.starts_with("~/") {
            let home = env::var("HOME")
                .or_else(|_| env::var("USERPROFILE"))
                .unwrap_or_else(|_| ".".to_string());
            sqlite_path.replace("~/", &format!("{}/", home))
        } else {
            sqlite_path
        };

        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost:5432/postgres".to_string()),
            sqlite_db_path,
            llm_api_key: env::var("LLM_API_KEY")
                .unwrap_or_else(|_| "".to_string()),
            llm_api_url: env::var("LLM_API_URL")
                .unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
        })
    }
}

