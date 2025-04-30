use crate::file::image_to_base64;
use crate::openapi::openai::{ChatCompletionResponse, EmbeddingResponse};
use anyhow::Result;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::io::ErrorKind;
use std::{env, io};

pub struct OpenAI {
    key: String,
    client: Client,
    base_url: String,
}
pub enum OpenAIType {
    Chat,
    Embedding,
}
impl OpenAI {
    pub fn new(open_ai_type: OpenAIType) -> Result<Self> {
        let client = Client::new();
        let base_url: &str;
        match open_ai_type {
            OpenAIType::Chat => {
                base_url = "https://api.openai.com/v1/chat/completions";
            }
            OpenAIType::Embedding => {
                base_url = "https://api.openai.com/v1/embeddings";
            }
        }
        let open_ai_token = env::var("OPEN_API_KEY")?;

        Ok(Self {
            key: open_ai_token,
            client,
            base_url: base_url.to_string(),
        })
    }

    async fn post<T: DeserializeOwned>(&self, body: Value) -> Result<T> {
        let response = self
            .client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.key))
            .json(&body)
            .send()
            .await?;
        Ok(response.json::<T>().await?)
    }

    fn parse_choice(chat_completion_response: &ChatCompletionResponse) -> Result<String> {
        match chat_completion_response.choices.first() {
            Some(choice) => Ok(choice.message.content.to_string()),
            None => Err(io::Error::new(ErrorKind::InvalidData, "Choice not found").into()),
        }
    }

    pub async fn send_user_message(&self, content: String) -> Result<String> {
        let body = json!({
        "model": "gpt-4o",
        "store": true,
        "messages": [
            {"role": "user", "content": content }
        ]
        });
        let response = self.post::<ChatCompletionResponse>(body).await?;
        Self::parse_choice(&response)
    }
    pub async fn send_image_with_prompt(
        &self,
        user_message: &str,
        image: String,
    ) -> Result<String> {
        let base64img = format!("data:image/png;base64,{}", image);
        let body = json!({
            "model": "gpt-4o",
            "store": true,
            "messages": [
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
                        }
                    ]
                }
            ]
        });
        let response = self.post::<ChatCompletionResponse>(body).await?;
        Self::parse_choice(&response)
    }

    pub async fn _send_sys_message(
        &self,
        sys_message: String,
        user_message: String,
    ) -> Result<String> {
        let body = json!({
        "model": "gpt-4o",
        "store": true,
        "messages": [
            {"role": "system", "content": sys_message },
            {"role": "user", "content": user_message }
        ]
        });
        let response = self.post::<ChatCompletionResponse>(body).await?;
        Self::parse_choice(&response)
    }
    pub async fn send_image_with_ref_image(
        &self,
        sys_message: String,
        user_message: String,
        image: String,
    ) -> Result<String> {
        //todo create a func to convert
        let base64img = format!("data:image/png;base64,{}", image);
        let popusk_base64 = format!(
            "data:image/jpg;base64,{}",
            //todo make generic
            image_to_base64("alt_images/alt.jpg")?
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
        Self::parse_choice(&response)
    }
    pub async fn embeddings(&self, image_description: &str) -> Result<EmbeddingResponse> {
        //todo check for self.client endpoint
        let body = json!({
        "model": "text-embedding-3-small",
        "input": image_description,
        });
        let response = self.post::<EmbeddingResponse>(body).await?;
        Ok(response)
    }
}
