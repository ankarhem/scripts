use crate::message::{MessageRequest, MessageResponse};
use anyhow::{Context, Result};
use reqwest::header::{CONTENT_TYPE, HeaderValue};

pub struct AnthropicClient {
    client: reqwest::Client,
    base_url: String,
}

impl AnthropicClient {
    pub fn new(base_url: &str, auth_token: &str) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("x-api-key", auth_token.parse()?);
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            base_url: base_url.to_string(),
            client,
        })
    }
}

impl AnthropicClient {
    pub async fn send_message(&self, message: MessageRequest) -> Result<MessageResponse> {
        let url = format!("{}/v1/messages", &self.base_url);

        let response = self.client.post(url).json(&message).send().await?;

        response
            .json::<MessageResponse>()
            .await
            .with_context(|| "Failed to parse response JSON")
    }
}
