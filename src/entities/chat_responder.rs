use crate::common::{BotError, ChatId};
use crate::entities::profile_reviewer::ProcessingStatus;
use crate::pg::pg::{DbQuery, DbStatusQuery, PgClient};
use async_trait::async_trait;
use tokio_postgres::Row;
use uuid::Uuid;

pub struct ChatResponder {
    id: Uuid,
    status: ProcessingStatus,
    chat_id: ChatId,
    from: String,
    to: Option<String>,
}

#[async_trait]
impl DbQuery for ChatResponder {
    async fn insert<'a>(&'a self, pg_client: &'a PgClient) -> Result<(), BotError> {
        let query = "INSERT into chat_responders (\
        id,
        status,\
        chat_id, \
        from, \
        VALUES ($1,$2,$3,$4)";
        pg_client
            .query(
                query,
                &[
                    &self.id,
                    //todo fix convert to str
                    &"WAITING",
                    &self.chat_id,
                    &self.from,
                ],
            )
            .await?;
        Ok(())
    }

    async fn select_by_id(pg_client: &PgClient, id: Uuid) -> Result<Self, BotError>
    where
        Self: Sized,
    {
        let query = "SELECT * from chat_responders id = $1";
        let row = pg_client.query_one(query, &[&id]).await?;
        Ok(Self::from_sql(row)?)
    }

    fn from_sql(row: Row) -> Result<Self, BotError>
    where
        Self: Sized,
    {
        Ok(Self {
            id: row.try_get("id")?,
            chat_id: row.try_get("chat_id")?,
            status: row.try_get("status")?,
            from: row.try_get("from")?,
            to: row.try_get("to")?,
        })
    }
}

#[async_trait]
impl DbStatusQuery for ChatResponder {
    type Status = ProcessingStatus;

    async fn update_status<'a>(
        &'a self,
        pg_client: &'a PgClient,
        status: Self::Status,
    ) -> Result<(), BotError> {
        let query = "UPDATE chat_responders SET status=$1 WHERE id=$2";
        pg_client
            .query(query, &[&status.to_str()?, &self.id])
            .await?;
        Ok(())
    }

    async fn get_by_status_one(
        pg_client: &PgClient,
        status: Self::Status,
    ) -> Result<Option<Self>, BotError> {
        let query = "SELECT * from chat_responders WHERE status = $1 LIMIT 1";
        let row_opt = pg_client.query_opt(query, &[&status.to_str()?]).await?;
        match row_opt {
            Some(row) => Ok(Some(Self::from_sql(row)?)),
            None => Ok(None),
        }
    }
}
impl ChatResponder {
    pub fn new(chat_id: ChatId, from: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            status: ProcessingStatus::Waiting,
            chat_id,
            from: from.to_string(),
            to: None,
        }
    }
    pub fn update_to(&self, to: &str) -> Result<(), BotError> {}
}
