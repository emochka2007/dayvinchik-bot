use serde::{Deserialize};

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct Choice {
    index: i32,
    pub message: Message,
    logprobs: Option<serde_json::Value>, // Use serde_json::Value for fields that can vary or are optional
    finish_reason: String,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    role: String,
    pub content: String,
    refusal: Option<serde_json::Value>, // assuming refusal can be null or some other structure
}


#[derive(Deserialize, Debug)]
struct TokenDetails {
    cached_tokens: Option<i32>,
    audio_tokens: Option<i32>,
    reasoning_tokens: Option<i32>,
    accepted_prediction_tokens: Option<i32>,
    rejected_prediction_tokens: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct EmbeddingResponse {
    object: String,
    pub data: Vec<EmbeddingData>,
    model: String,
    usage: EmbeddingUsage,
}
#[derive(Deserialize, Debug)]
pub struct EmbeddingData {
    object: String,
    index: i32,
    pub embedding: Vec<f32>,
}
#[derive(Deserialize, Debug)]
pub struct EmbeddingUsage {
    prompt_tokens: i32,
    total_tokens: i32,
}

#[derive(Debug, Deserialize)]
pub struct FineTuningResponse {
    pub id: String,
    pub object: String,
    pub created_at: u64,
    pub status: String,
    pub error: Option<serde_json::Value>,
    pub incomplete_details: Option<serde_json::Value>,
    pub instructions: Option<serde_json::Value>,
    pub max_output_tokens: u32,
    pub model: String,
    pub output: Vec<OutputItem>,
    pub parallel_tool_calls: bool,
    pub previous_response_id: Option<String>,
    pub reasoning: Reasoning,
    pub service_tier: String,
    pub store: bool,
    pub temperature: f32,
    pub text: TextField,
    pub tool_choice: String,
    pub tools: Vec<serde_json::Value>,
    pub top_p: f32,
    pub truncation: String,
    pub usage: Usage,
    pub user: Option<serde_json::Value>,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct OutputItem {
    pub id: String,
    #[serde(rename = "type")]
    pub output_type: String,
    pub status: String,
    pub content: Vec<ContentItem>,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct ContentItem {
    #[serde(rename = "type")]
    pub content_type: String,
    pub annotations: Vec<serde_json::Value>,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Reasoning {
    pub effort: Option<serde_json::Value>,
    pub summary: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct TextField {
    pub format: TextFormat,
}

#[derive(Debug, Deserialize)]
pub struct TextFormat {
    #[serde(rename = "type")]
    pub format_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub input_tokens_details: InputTokensDetails,
    pub output_tokens: u32,
    pub output_tokens_details: OutputTokensDetails,
    pub total_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct InputTokensDetails {
    pub cached_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct OutputTokensDetails {
    pub reasoning_tokens: u32,
}
