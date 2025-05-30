use crate::common::random_number;
use crate::pg::pg::{DbQuery, PgClient};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct SuperLike {
    id: Uuid,
    message: String,
    profile_reviewer_id: Uuid,
}

#[async_trait]
impl DbQuery for SuperLike {
    const DB_NAME: &'static str = "superlikes";
    async fn insert<'a>(&'a self, pg_client: &'a PgClient) -> Result<()> {
        let query = "INSERT into superlikes\
         (id,\
         message, \
         profile_reviewer_id) \
         VALUES (\
         $1,\
         $2,\
         $3)";
        pg_client
            .query(query, &[&self.id, &self.message, &self.profile_reviewer_id])
            .await?;
        Ok(())
    }

    fn from_sql(row: Row) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            id: row.try_get("id")?,
            message: row.try_get("message")?,
            profile_reviewer_id: row.try_get("profile_reviewer_id")?,
        })
    }
}
impl SuperLike {
    pub fn new(message: String, profile_reviewer_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            message,
            profile_reviewer_id,
        }
    }
    pub fn message(&self) -> &str {
        &self.message
    }

    // todo move them to db
    pub fn get_starter() -> String {
        let messages = [
            "няшка",
            "ты очень милая",
            "посоветуй аниме",
            "хочешь погулять?",
            "пойдем в бар",
            "го покурим",
            "я тебя в тт видел",
            "как ты?",
            "ахаххахахаахха",
            "го на концерт",
        ];
        let index = random_number(0, messages.len() as i64);
        messages.get(index as usize).unwrap().to_string()
    }

    pub fn is_superlike_notification(message_text: &str) -> bool {
        //todo mb improve str
        let string_to_contain = "Ты понравился";
        message_text.contains(string_to_contain)
    }
}
