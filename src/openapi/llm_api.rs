use crate::file::image_to_base64;
use crate::openapi::openai::ChatCompletionResponse;
use crate::prompts::Prompt;
use log::debug;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::env;
use std::error::Error;

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
    async fn post<T: DeserializeOwned>(&self, body: Value) -> Result<T, OpenAIError> {
        let response = self
            .client
            .post(format!("{}completions", self.base_url))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.key))
            .json(&body)
            .send()
            .await?;
        Ok(response.json::<T>().await?)
    }

    pub async fn _send_user_message(&self, message: String) -> Result<String, OpenAIError> {
        let prompt = Prompt::main(&message);
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
        Ok(text)
    }

    pub async fn _send_sys_message(
        &self,
        sys_message: String,
        user_message: String,
    ) -> Result<String, OpenAIError> {
        let body = json!({
        "model": "gpt-4o",
        "store": true,
        "messages": [
            {"role": "system", "content": sys_message },
            {"role": "user", "content": user_message }
        ]
        });
        let response = self.post::<ChatCompletionResponse>(body).await?;
        let text = response.choices.get(0).unwrap().message.content.to_string();
        Ok(text)
    }
    pub async fn send_sys_image_message(
        &self,
        sys_message: String,
        user_message: String,
        image: String,
    ) -> Result<String, OpenAIError> {
        //todo create a func to convert
        let base64img = format!("data:image/png;base64,{}", image);
        let popusk_base64 = format!(
            "data:image/jpg;base64,{}",
            image_to_base64("alt_images/popusk.jpg").unwrap()
        );
        let body = json!({
            "model": "gpt-4o",
            "store": true,
            "messages": [
                {
                    "role": "system",
                    "content": sys_message
                },
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": user_message
                        },
                        {
                            "type": "image_url",
                            "image_url": {
                                "url": base64img
                            }
                        },
                        {
                            "type": "image_url",
                            "image_url": {
                                "url": popusk_base64
                            }
                        }
                    ]
                }
            ]
        });
        let response = self.post::<ChatCompletionResponse>(body).await?;
        debug!("{:?}", response);
        let text = response.choices.get(0).unwrap().message.content.to_string();
        Ok(text)
    }
}
