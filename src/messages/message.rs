use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessage {
    #[serde(rename = "@type")]
    pub(crate) t: String,
    pub(crate) chat_id: String,
    pub(crate) input_message_content: InputMessageContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputMessageContent {
    #[serde(rename = "@type")]
    t: String,
    text: FormattedText,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FormattedText {
    #[serde(rename = "@type")]
    td_type: String,
    text: String,
}

//todo implement serialize
impl SendMessage {
    pub fn dislike(chat_id: &str) -> Self {
        let message = SendMessage {
            t: "sendMessage".to_string(),
            chat_id: chat_id.to_string(),
            input_message_content: InputMessageContent {
                t: "inputMessageText".to_string(),
                text: FormattedText {
                    td_type: "formattedText".to_string(),
                    text: "👎".to_string(),
                },
            },
        };
        message
    }
    pub fn like(chat_id: &str) -> Self {
        let message = SendMessage {
            t: "sendMessage".to_string(),
            chat_id: chat_id.to_string(),
            input_message_content: InputMessageContent {
                t: "inputMessageText".to_string(),
                text: FormattedText {
                    td_type: "formattedText".to_string(),
                    text: "👍".to_string(),
                },
            },
        };
        message
    }
    
    pub fn super_like(chat_id: &str) -> Self {
        SendMessage {
            t: "sendMessage".to_string(),
            chat_id: chat_id.to_string(),
            input_message_content: InputMessageContent {
                t: "inputMessageText".to_string(),
                text: FormattedText {
                    td_type: "formattedText".to_string(),
                    text: "💌 / 📹".to_string(),
                },
            },
        }
    }

    pub fn text_message(text: &str, chat_id: &str) -> Self {
        let message = SendMessage {
            t: "sendMessage".to_string(),
            chat_id: chat_id.to_string(),
            input_message_content: InputMessageContent {
                t: "inputMessageText".to_string(),
                text: FormattedText {
                    td_type: "formattedText".to_string(),
                    text: text.to_string(),
                },
            },
        };
        message
    }
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomGetMe {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i64>,
    #[serde(rename(serialize = "@type"))]
    td_type: String,
}
impl CustomGetMe {
    pub fn builder() -> Self {
        CustomGetMe {
            extra: Some(Uuid::new_v4().to_string()),
            client_id: Some(123),
            td_type: "getMe".to_string(),
        }
    }
}
