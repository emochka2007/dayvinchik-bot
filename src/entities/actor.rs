use crate::common::BotError;
use crate::entities::dv_bot::DvBot;
use crate::entities::profile_reviewer::{ProcessingStatus, ProfileReviewer};
use crate::entities::superlike::SuperLike;
use crate::pg::pg::{DbStatusQuery, PgClient};
use crate::prompts::Prompt;
use log::{error, info};
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

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
    score_threshold: i32,
}

impl Actor {
    pub fn new(actor_type: ActorType, score_threshold: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            actor_type,
            score_threshold,
        }
    }

    pub fn prompt(&self) -> Prompt {
        match self.actor_type {
            ActorType::ANALYZER => Prompt::analyze_alt(),
            _ => Prompt::analyze_alt(),
        }
    }

    /// First we update the chat and only after update latest messages for dv bot
    pub async fn analyze(&self, pg_client: &PgClient) -> Result<(), BotError> {
        info!("Analyzing...");
        //break statement mb
        DvBot::refresh(pg_client).await?;
        DvBot::read_last_message(pg_client).await?;
        loop {
            sleep(Duration::from_secs(5)).await;
            if let Some(completed_reviewer) =
                ProfileReviewer::get_ready_to_proceed(pg_client).await?
            {
                if let Some(score) = completed_reviewer.score() {
                    if *score >= self.score_threshold {
                        DvBot::send_superlike(pg_client, completed_reviewer.id()).await?;
                    } else {
                        DvBot::send_dislike(pg_client).await?;
                    }
                    completed_reviewer
                        .update_status(pg_client, ProcessingStatus::Processed)
                        .await?;
                    DvBot::read_last_message(pg_client).await?;
                } else {
                    continue;
                }
            }
        }
        Ok(())
    }
}
