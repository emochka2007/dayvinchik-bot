use crate::common::BotError;
use crate::entities::dv_bot::DvBot;
use crate::entities::profile_match::ProfileMatch;
use crate::entities::superlike::SuperLike;
use crate::pg::pg::PgClient;
use crate::td::td_message::MessageMeta;

/// Analyze last 10 messages for any matches
pub struct MatchAnalyzer {}

impl MatchAnalyzer {
    pub async fn read_messages_from_db(pg_client: &PgClient) -> Result<(), BotError> {
        let all_messages = MessageMeta::get_all_unprocessed(pg_client).await?;
        for parsed_message in all_messages {
            if SuperLike::is_superlike_notification(parsed_message.text()) {
                DvBot::send_message(pg_client, "1").await?;
                DvBot::read_last_message(pg_client).await?;
                return Ok(());
            }
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
        }
        Ok(())
    }
}
