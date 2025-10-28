use anyhow::anyhow;
use dirs::home_dir;
use serde::Deserialize;
use std::{env, fs};

#[derive(Debug, Clone)]
pub struct AnthropicConfig {
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

impl AnthropicConfig {
    pub fn auth_token(&self) -> &str {
        &self.auth_token
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn builder() -> AnthropicConfigBuilder {
        AnthropicConfigBuilder::new()
    }
}

pub struct AnthropicConfigBuilder {
    auth_token: Option<String>,
    base_url: Option<String>,
    model: Option<String>,
}

impl Default for AnthropicConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AnthropicConfigBuilder {
    pub fn new() -> Self {
        Self {
            auth_token: None,
            base_url: None,
            model: None,
        }
    }

    pub fn auth_token(mut self, token: String) -> Self {
        self.auth_token = Some(token);
        self
    }

    pub fn base_url(mut self, url: String) -> Self {
        self.base_url = Some(url);
        self
    }

    pub fn model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    pub fn with_defaults(mut self) -> anyhow::Result<Self> {
        self = self.add_from_env();
        self = self.add_from_settings()?;
        Ok(self)
    }

    fn add_from_env(mut self) -> Self {
        if let Ok(token) = env::var("ANTHROPIC_AUTH_TOKEN") {
            self.auth_token = Some(token);
        }
        if let Ok(url) = env::var("ANTHROPIC_BASE_URL") {
            self.base_url = Some(url);
        }
        if let Ok(model) = env::var("ANTHROPIC_DEFAULT_SONNET_MODEL") {
            self.model = Some(model);
        }
        self
    }

    fn add_from_settings(mut self) -> anyhow::Result<Self> {
        let settings_file = home_dir()
            .ok_or_else(|| anyhow!("Cannot determine home directory"))?
            .join(".claude/settings.json");

        if settings_file.exists() {
            let content = fs::read_to_string(&settings_file)?;
            let settings: ClaudeSettings = serde_json::from_str(&content)
                .map_err(|e| anyhow!("Failed to parse settings file: {}", e))?;

            if let Some(env) = settings.env {
                if let Some(base_url) = env.base_url {
                    self.base_url = Some(base_url);
                }
                if let Some(model) = env.default_sonnet_model {
                    self.model = Some(model);
                }
                if let Some(auth_token) = env.auth_token {
                    self.auth_token = Some(auth_token);
                }
            }
        }

        Ok(self)
    }

    pub fn build(self) -> anyhow::Result<AnthropicConfig> {
        Ok(AnthropicConfig {
            auth_token: self
                .auth_token
                .ok_or_else(|| anyhow!("Auth token is required"))?,
            base_url: self
                .base_url
                .unwrap_or("https://api.anthropic.com".to_string()),
            model: self.model.unwrap_or("claude-sonnet-4-5".to_string()),
        })
    }
}
