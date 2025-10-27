use anyhow::anyhow;
use dirs::home_dir;
use serde::Deserialize;
use std::{env, fs};

#[derive(Debug)]
pub struct Config {
    auth_token: String,
    base_url: String,
    model: String,
}
#[derive(Debug, Deserialize, Default)]
struct EnvSettings {
    #[serde(rename = "ANTHROPIC_AUTH_TOKEN")]
    auth_token: Option<String>,
    #[serde(rename = "ANTHROPIC_BASE_URL")]
    base_url: Option<String>,
    #[serde(rename = "ANTHROPIC_DEFAULT_SONNET_MODEL")]
    default_sonnet_model: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ClaudeSettings {
    env: Option<EnvSettings>,
}

impl Config {
    pub fn auth_token(&self) -> &str {
        &self.auth_token
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn model(&self) -> &str {
        &self.model
    }
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        // Try to load from settings file
        let settings_file = home_dir()
            .ok_or_else(|| anyhow!("Cannot determine home directory"))?
            .join(".claude/settings.json");

        let (file_auth_token, file_base_url, file_model) = if settings_file.exists() {
            let content = fs::read_to_string(&settings_file)?;
            let settings: ClaudeSettings = serde_json::from_str(&content)
                .map_err(|e| anyhow!("Failed to parse settings file: {}", e))?;

            let env_settings = settings.env.unwrap_or_default();
            (
                env_settings.auth_token,
                env_settings.base_url,
                env_settings.default_sonnet_model,
            )
        } else {
            (None, None, None)
        };

        // Priority: env vars > file > defaults
        let auth_token = env::var("ANTHROPIC_AUTH_TOKEN")
            .ok()
            .or(file_auth_token)
            .ok_or_else(|| anyhow!("ANTHROPIC_AUTH_TOKEN environment variable is not set"))?;

        let base_url = env::var("ANTHROPIC_BASE_URL")
            .ok()
            .or(file_base_url)
            .unwrap_or_else(|| "https://api.anthropic.com".to_string());

        let model = env::var("ANTHROPIC_DEFAULT_SONNET_MODEL")
            .ok()
            .or(file_model)
            .unwrap_or_else(|| "claude-sonnet-4-5".to_string());

        Ok(Config {
            auth_token,
            base_url,
            model,
        })
    }
}
