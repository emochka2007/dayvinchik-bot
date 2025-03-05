use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    pub choices: Vec<Choice>,
    usage: Usage,
    service_tier: String,
    system_fingerprint: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    index: i32,
    pub message: Message,
    logprobs: Option<serde_json::Value>, // Use serde_json::Value for fields that can vary or are optional
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    role: String,
    pub content: String,
    refusal: Option<serde_json::Value>, // assuming refusal can be null or some other structure
}

#[derive(Serialize, Deserialize, Debug)]
struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
    prompt_tokens_details: TokenDetails,
    completion_tokens_details: TokenDetails,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenDetails {
    cached_tokens: Option<i32>,
    audio_tokens: Option<i32>,
    reasoning_tokens: Option<i32>,
    accepted_prediction_tokens: Option<i32>,
    rejected_prediction_tokens: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddingResponse {
    object: String,
    data: Vec<EmbeddingData>,
    model: String,
    usage: EmbeddingUsage,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddingData {
    object: String,
    index: i32,
    embedding: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddingUsage {
    prompt_tokens: i32,
    total_tokens: i32,
}
