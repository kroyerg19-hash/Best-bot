use anyhow::{Context, Result};

#[derive(Clone, Debug)]
pub struct Config {
    pub creator_number: String,
    pub spiderx_api_key: Option<String>,
    pub gemini_api_key: Option<String>,
    pub openai_api_key: Option<String>,
    pub telegram_bot_token: Option<String>,
    pub prefix: String,
    pub data_dir: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            creator_number: std::env::var("GIPSY_CREATOR_NUMBER")
                .context("GIPSY_CREATOR_NUMBER não configurado")?,
            spiderx_api_key: std::env::var("SPIDERX_API_KEY").ok(),
            gemini_api_key: std::env::var("GEMINI_API_KEY").ok(),
            openai_api_key: std::env::var("OPENAI_API_KEY").ok(),
            telegram_bot_token: std::env::var("TELEGRAM_BOT_TOKEN").ok(),
            prefix: std::env::var("GIPSY_PREFIX").unwrap_or_else(|_| "!".into()),
            data_dir: std::env::var("GIPSY_DATA_DIR").unwrap_or_else(|_| "./data".into()),
        })
    }
}
