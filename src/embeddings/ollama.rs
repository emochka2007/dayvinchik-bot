use crate::common::BotError;
use crate::prompts::Prompt;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest};
use ollama_rs::generation::images::Image;
use ollama_rs::Ollama;
use pgvector::Vector;
use std::fmt::format;

pub struct OllamaVision {
    ollama: Ollama,
    model: String,
}
impl OllamaVision {
    pub fn new() -> Self {
        let model = "llava".to_string();
        // todo config
        let ollama = Ollama::new("http://localhost".to_string(), 11434);

        Self { ollama, model }
    }
    pub async fn get_image_embedding(&self, query: &str) -> Result<Vector, BotError> {
        let embed_input = EmbeddingsInput::from(query);
        let res = self
            .ollama
            .generate_embeddings(GenerateEmbeddingsRequest::new(
                self.model.to_string(),
                embed_input,
            ))
            .await?;
        let float_embeddings = res.embeddings.first().unwrap();
        Ok(Vector::from(float_embeddings.clone()))
    }

    pub async fn describe_image(&self, bs64image: String) -> Result<String, BotError> {
        let prompt = Prompt::image_description();
        let image = Image::from_base64(bs64image);
        let request = GenerationRequest::new(self.model.to_string(), prompt).add_image(image);
        let res = self.ollama.generate(request).await?;
        Ok(res.response)
    }
}
