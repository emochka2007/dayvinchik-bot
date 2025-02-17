use crate::entities::dv_bot::DvBot;
use crate::entities::profile_reviewer::ProfileReviewer;
use crate::file::{image_to_base64, move_file};
use crate::openapi::llm_api::OpenAI;
use crate::pg::pg::PgClient;
use crate::prompts::Prompt;
use log::{error, info};
use serde_json::Error;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;
use crate::td::td_response::ResponseKeys;

/**
- Entity in database
- All messages related to this actor
- Actor identifies the behaviour of your chat
**/
pub enum ActorType {
    DEFAULT,
    ANALYZER,
}
pub struct Actor {
    id: Uuid,
    actor_type: ActorType,
}

impl Actor {
    pub fn new(actor_type: ActorType) -> Self {
        Self {
            id: Uuid::new_v4(),
            actor_type,
        }
    }

    pub fn prompt(&self) -> Prompt {
        match self.actor_type {
            ActorType::ANALYZER => Prompt::analyze_alt(),
            _ => Prompt::analyze_alt(),
        }
    }

    pub async fn analyze(&self, pg_client: &PgClient) -> Result<(), Error> {
        info!("Analyzing...");
        // First we update the chat and only after update latest messages for dv bot
        DvBot::refresh(pg_client).await?;
        // todo if chats empty run or force flag mb
        // DvBot::on_init(pg_client).await?;
        loop {
            if let Ok(Some(_)) = ProfileReviewer::acquire_bot(pg_client).await {
                DvBot::read_last_message(pg_client).await?;
                DvBot::send_dislike(pg_client).await?;
            }
            sleep(Duration::from_secs(5)).await;
        }
        // Ok(())
    }
}
