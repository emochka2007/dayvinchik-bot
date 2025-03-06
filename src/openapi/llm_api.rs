use crate::common::BotError;
use crate::file::image_to_base64;
use crate::openapi::openai::{ChatCompletionResponse, EmbeddingResponse};
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
    pub fn new(open_ai_type: OpenAIType) -> Result<Self, BotError> {
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
        Ok(Self {
            key: env::var("OPEN_API_KEY")?,
            client,
            base_url: base_url.to_string(),
        })
    }

    async fn post<T: DeserializeOwned>(&self, body: Value) -> Result<T, BotError> {
        // let response = self
        //     .client
        //     .post(&self.base_url)
        //     .header("Content-Type", "application/json")
        //     .header("Authorization", format!("Bearer {}", self.key))
        //     .json(&body)
        //     .send()
        //     .await?;
        // println!("{:?}", response.text().await.unwrap());
        // /**
        // Override here, we need to send not only completions but also embeddings and chat
        // */
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

    fn parse_choice(chat_completion_response: &ChatCompletionResponse) -> Result<String, BotError> {
        match chat_completion_response.choices.first() {
            Some(choice) => Ok(choice.message.content.to_string()),
            None => Err(io::Error::new(ErrorKind::InvalidData, "Choice not found").into()),
        }
    }

    pub async fn send_user_message(&self, content: String) -> Result<String, BotError> {
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
    ) -> Result<String, BotError> {
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
    ) -> Result<String, BotError> {
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
    ) -> Result<String, BotError> {
        //todo create a func to convert
        let base64img = format!("data:image/png;base64,{}", image);
        let popusk_base64 = format!(
            "data:image/jpg;base64,{}",
            image_to_base64("alt_images/popusk.jpg")?
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
    pub async fn embeddings(&self, image_description: &str) -> Result<EmbeddingResponse, BotError> {
        //todo check for self.client endpoint
        let body = json!({
        "model": "text-embedding-3-small",
        "input": image_description,
        });
        let response = self.post::<EmbeddingResponse>(body).await?;
        Ok(response)
    }
}
