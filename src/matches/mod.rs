use crate::common::BotError;
use crate::constants::VINCHIK_CHAT_INT;
use crate::entities::chat_meta::ChatMeta;
use crate::entities::dv_bot::DvBot;
use crate::entities::profile_match::ProfileMatch;
use crate::entities::superlike::SuperLike;
use crate::pg::pg::PgClient;
use crate::td::td_message::{td_get_last_message, td_read_one_from_message_id, MessageMeta};
use std::time::Duration;
use tokio::time::sleep;

/// Analyze last 10 messages for any matches
pub struct MatchAnalyzer {}

impl MatchAnalyzer {
    pub async fn start(pg_client: &PgClient) -> Result<(), BotError> {
        Self::get_last_100_msgs(pg_client).await?;
        loop {
            Self::read_messages_from_db(pg_client).await?;
            sleep(Duration::from_secs(30)).await;
        }
    }
    pub async fn get_last_100_msgs(pg_client: &PgClient) -> Result<(), BotError> {
        let chat = ChatMeta::select_by_chat_id(VINCHIK_CHAT_INT, pg_client)
            .await?
            .unwrap();

        for i in 1..100 {
            td_read_one_from_message_id(pg_client, VINCHIK_CHAT_INT, *chat.last_message_id(), i)
                .await?;
        }
        Ok(())
    }
    pub async fn read_messages_from_db(pg_client: &PgClient) -> Result<(), BotError> {
        let all_messages = MessageMeta::get_all_unprocessed(pg_client).await?;
        for parsed_message in all_messages {
            // Match with url inside
            if parsed_message.is_match() {
                if let Some(url) = parsed_message.url() {
                    let profile_match = ProfileMatch {
                        url: url.to_string(),
                        full_text: parsed_message.text().to_string(),
                    };
                    profile_match.insert_db(pg_client).await?;
                }
            }
            parsed_message.process(pg_client).await?;
        }
        Ok(())
    }
}
