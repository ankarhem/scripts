use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(serde::Serialize, serde::Deserialize, TypedBuilder)]
#[serde(rename_all = "snake_case")]
pub struct MessageRequest {
    #[builder(default = "claude-sonnet-4-5".to_string())]
    pub model: String,
    #[builder(default = 1024)]
    pub max_tokens: u32,
    #[builder(default = Messages::new())]
    pub messages: Messages,
}

impl MessageRequest {
    pub fn add_user<S: Into<String>>(mut self, content: S) -> Self {
        let content = content.into();
        self.messages = self.messages.add_user(content);
        self
    }

    pub fn add_assistant<S: Into<String>>(mut self, content: S) -> Self {
        let content = content.into();
        self.messages = self.messages.add_assistant(content);
        self
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Assistant,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Messages(Vec<Message>);

impl Default for Messages {
    fn default() -> Self {
        Self::new()
    }
}

impl Messages {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    fn add_message(&mut self, role: Role, content: String) {
        self.0.push(Message { role, content });
    }

    pub fn add_user<S: Into<String>>(mut self, content: S) -> Self {
        let content = content.into();
        self.add_message(Role::User, content);
        self
    }

    pub fn add_assistant<S: Into<String>>(mut self, content: S) -> Self {
        let content = content.into();
        self.add_message(Role::Assistant, content);
        self
    }

    pub fn into_vec(self) -> Vec<Message> {
        self.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct Content {
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    pub content: Vec<Content>,
    pub role: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_request_builder() {
        let messages = Messages::new().add_user("Hello").add_assistant("Hello");
        let request = MessageRequest::builder()
            .model("glm-4.6".into())
            .max_tokens(256)
            .messages(messages)
            .build();

        insta::assert_json_snapshot!(request)
    }

    #[test]
    fn test_message_request_builder_defaults() {
        let request = MessageRequest::builder().build();

        insta::assert_json_snapshot!(request)
    }
}
