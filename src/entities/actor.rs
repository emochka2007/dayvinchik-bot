use crate::common::BotError;
use crate::entities::dv_bot::DvBot;
use crate::entities::profile_reviewer::ProfileReviewer;
use crate::pg::pg::PgClient;
use crate::prompts::Prompt;
use log::{info};
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
        DvBot::refresh(pg_client).await?;
        DvBot::read_last_message(pg_client).await?;
        loop {
            let _ = ProfileReviewer::get_ready_to_proceed(pg_client).await.is_ok();
            {
                match ProfileReviewer::get_completed(pg_client).await {
                    Ok(profile_reviewer) => {
                        if let Some(score) = profile_reviewer.score() {
                            if *score >= self.score_threshold {
                                DvBot::send_like(pg_client).await?;
                            } else {
                                DvBot::send_dislike(pg_client).await?;
                            }
                            ProfileReviewer::set_processed(
                                profile_reviewer.id().to_string(),
                                pg_client,
                            )
                                .await?;
                            DvBot::read_last_message(pg_client).await?;
                        }
                    }
                    Err(e) => {
                        // error!("Error getting completed {:?}", e);
                        DvBot::read_last_message(pg_client).await?;
                    }
                }
            }

            sleep(Duration::from_secs(10)).await;
        }
        // Ok(())
    }
}
