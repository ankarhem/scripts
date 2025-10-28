use crate::config::AnthropicConfig;
use crate::message::{MessageRequest, MessageResponse};
use anyhow::{Context, Result};
use reqwest::header::{CONTENT_TYPE, HeaderValue};

pub struct AnthropicClient {
    client: reqwest::Client,
    config: AnthropicConfig,
}

impl AnthropicClient {
    pub fn new(config: AnthropicConfig) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("x-api-key", config.auth_token().parse()?);
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client, config })
    }
}

impl AnthropicClient {
    pub async fn send_message(&self, mut message: MessageRequest) -> Result<MessageResponse> {
        let url = format!("{}/v1/messages", self.config.base_url());

        if message.model.is_none() {
            message.model = Some(self.config.model().to_string());
        }

        let response = self.client.post(url).json(&message).send().await?;

        response
            .json::<MessageResponse>()
            .await
            .with_context(|| "Failed to parse response JSON")
    }
}
