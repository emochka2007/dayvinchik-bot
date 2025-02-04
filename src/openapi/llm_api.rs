use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt::format;
use reqwest::Client;
use rust_tdlib::types::Chat;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use crate::openapi::openai::ChatCompletionResponse;
use crate::prompts::Prompt;

type OpenAIError = Box<dyn Error>;
pub struct OpenAI {
    key: String,
    client: Client,
    base_url: String,
}
impl OpenAI {
    pub fn new() -> Self {
        let client = Client::new();
        let base_url = "https://api.openai.com/v1/chat/".to_string();
        Self {
            key: env::var("OPEN_API_KEY").unwrap(),
            client,
            base_url,
        }
    }

    //Post Request
    async fn post<T: DeserializeOwned>(&self, body: Value) -> Result<T,OpenAIError> {
        let response = self.client.post(format!("{}completions", self.base_url))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.key))
            .json(&body)
            .send().await?;
        Ok(response.json::<T>().await?)
    }

    pub async fn profile_check(&self, profile_description: String) -> Result<String, OpenAIError> {
        let prompt = Prompt::profile_review();
        let content = prompt.user;
        let body = json!({
        "model": "gpt-4o",
        "store": true,
        "messages": [
            {"role": "user", "content": content }
        ]
        });
        let response = self.post::<ChatCompletionResponse>(body).await?;
        let text = response.choices.get(0).unwrap().message.content.to_string();
        println!("text {text}");
        Ok(text)
    }
}