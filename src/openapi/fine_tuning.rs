use std::env;
use reqwest::Client;
use serde_json::json;
use anyhow::{anyhow, Result};
use crate::openapi::openai::FineTuningResponse;

pub struct FineTuningOpenAI {
    key: String,
    model_name: String,
    client: Client,
    base_url: String,
}


impl FineTuningOpenAI {
    pub fn new() -> Result<Self> {
        let model_name = "ft:gpt-4.1-mini-2025-04-14:personal:emochka007:BTeZ9uCW:ckpt-step-20".to_string();
        let client = Client::new();
        let base_url: &str = "https://api.openai.com/v1/responses";
        let open_ai_token = env::var("OPEN_API_KEY")?;

        Ok(Self {
            key: open_ai_token,
            client,
            model_name,
            base_url: base_url.to_string(),
        })
    }

    pub async fn send(&self, input: &str) -> Result<FineTuningResponse> {
        let body = json!({
            "model": self.model_name,
            "input": input,
            "text": {
                "format": {
                    "type": "text"
                }
            },
            "reasoning": {},
            "tools": [],
            "temperature": 1,
            "max_output_tokens": 2048,
            "top_p": 1,
            "store": true
        });

        let response = self
            .client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.key))
            .json(&body)
            .send()
            .await?;
        Ok(response.json::<FineTuningResponse>().await?)
    }

    pub async fn get_assistant_response(&self, input: &str) -> Result<String> {
        let response = self.send(input).await?;
        let output = response.output.first().ok_or(anyhow!("Output not found"))?;
        let content = output.content.first().ok_or(anyhow!("Output not found"))?;
        Ok(content.text.clone())
    }
}
