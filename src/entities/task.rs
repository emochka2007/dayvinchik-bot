use crate::constants::update_last_request;
use crate::pg::pg::{DbQuery, DbStatusQuery, PgClient};
use crate::td::td_json::send;
use crate::td::td_request::RequestKeys;
use crate::td::td_response::ResponseKeys;
use anyhow::{Error, Result};
use async_trait::async_trait;
use deadpool_postgres::GenericClient;
use log::debug;
use rust_tdlib::tdjson::ClientId;
use std::io;
use std::io::ErrorKind;
use tokio_postgres::types::{FromSql, Type};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct TdManager {
    client_id: ClientId,
}
#[derive(Debug)]
pub enum TaskStatus {
    Waiting,
    Pending,
    Complete,
}
impl TaskStatus {
    pub fn to_str(&self) -> String {
        match self {
            TaskStatus::Waiting => String::from("WAITING"),
            TaskStatus::Pending => String::from("PENDING"),
            TaskStatus::Complete => String::from("COMPLETE"),
        }
    }

    pub fn from_str(str: &str) -> Result<Self> {
        let status = match str {
            "WAITING" => TaskStatus::Waiting,
            "PENDING" => TaskStatus::Pending,
            "COMPLETE" => TaskStatus::Complete,
            _ => {
                return Err(Error::from(io::Error::new(
                    ErrorKind::InvalidData,
                    "Task status str not found",
                )))
            }
        };
        Ok(status)
    }
}
impl FromSql<'_> for TaskStatus {
    fn from_sql(_ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let string_from_db = String::from_utf8(raw.to_vec()).expect("Invalid UTF-8");
        let status = match string_from_db.as_str() {
            "WAITING" => TaskStatus::Waiting,
            "PENDING" => TaskStatus::Pending,
            "COMPLETE" => TaskStatus::Complete,
            _ => {
                return Err(Box::new(io::Error::new(
                    ErrorKind::NotFound,
                    "Task status not found",
                )))
            }
        };
        Ok(status)
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }
}
#[derive(Debug)]
pub struct Task {
    // todo mb client_id: ClientId,
    id: Uuid,
    message: String,
    status: TaskStatus,
    request: RequestKeys,
    response: ResponseKeys,
}
impl Default for Task {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            message: String::new(),
            status: TaskStatus::Waiting,
            response: ResponseKeys::Chat,
            request: RequestKeys::Unknown,
        }
    }
}
impl Task {
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn request(&self) -> &RequestKeys {
        &self.request
    }

    pub fn response(&self) -> &ResponseKeys {
        &self.response
    }
    pub fn message(&self) -> &String {
        &self.message
    }

    pub async fn new(
        message: String,
        request: RequestKeys,
        response: ResponseKeys,
        pg_client: &PgClient,
    ) -> Result<Self> {
        let id = Uuid::new_v4();
        let task = Self {
            id,
            message,
            response,
            request,
            status: TaskStatus::Waiting,
        };
        task.insert(pg_client).await?;
        Ok(task)
    }
    pub async fn match_by_req_res(
        pg_client: &PgClient,
        last_td_lib_call: &RequestKeys,
        response_keys: &ResponseKeys,
    ) -> Result<Option<Self>> {
        let query = "SELECT * from tasks WHERE request = $1 AND response = $2 ORDER BY created_at ASC LIMIT 1";
        let task_opt = pg_client
            .query_opt(
                query,
                &[&last_td_lib_call.to_str(), &response_keys.to_str()],
            )
            .await?;
        match task_opt {
            Some(row) => Ok(Some(Self::from_sql(row)?)),
            None => Ok(None),
        }
    }
    /// Find `/start` task
    pub async fn find_start(pg_client: &PgClient) -> Result<Option<()>> {
        let query = "SELECT id FROM tasks WHERE status = 'WAITING' \n
        AND jsonb_extract_path_text(message::jsonb, 'input_message_content', 'text', 'text') = '/start';";
        let task_opt = pg_client.query_opt(query, &[]).await?;
        match task_opt {
            Some(_) => Ok(Some(())),
            None => Ok(None),
        }
    }
}
/**
Pulls from db events and executes them
*/
impl TdManager {
    pub fn init(client_id: ClientId) -> Self {
        Self { client_id }
    }

    pub async fn send_request(&self, pg_client: &PgClient) -> Result<()> {
        match Task::get_by_status_one(pg_client, TaskStatus::Waiting).await? {
            Some(task) => {
                update_last_request(task.request)?;
                task.update_status(pg_client, TaskStatus::Pending).await?;
                send(self.client_id, &task.message);
                Ok(())
            }
            None => {
                debug!("No task found");
                Ok(())
            }
        }
    }
}

#[async_trait]
impl DbQuery for Task {
    const DB_NAME: &'static str = "tasks";
    async fn insert<'a>(&'a self, pg_client: &'a PgClient) -> Result<()> {
        let query = "Insert into tasks \
        (id, message, status, response,request)
        VALUES
        ($1, $2, $3, $4, $5)";
        pg_client
            .query(
                query,
                &[
                    &self.id,
                    &self.message,
                    &self.status.to_str(),
                    &self.response.to_str(),
                    &self.request.to_str(),
                ],
            )
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
            status: row.try_get("status")?,
            request: row.try_get("request")?,
            response: row.try_get("response")?,
        })
    }
    async fn clean_up(pg_client: &PgClient) -> Result<()> {
        let query = "UPDATE tasks SET status = $1";
        pg_client
            .query(query, &[&TaskStatus::Complete.to_str()])
            .await?;
        Ok(())
    }
}

#[async_trait]
impl DbStatusQuery for Task {
    type Status = TaskStatus;

    async fn update_status<'a>(
        &'a self,
        pg_client: &'a PgClient,
        status: Self::Status,
    ) -> Result<()> {
        let query = "UPDATE tasks SET status = $1 \
        WHERE id = $2";
        pg_client
            .query(query, &[&status.to_str(), self.id()])
            .await?;
        Ok(())
    }

    async fn get_by_status_one(pg_client: &PgClient, status: Self::Status) -> Result<Option<Self>>
    where
        Self: Sized,
    {
        let query = "SELECT * from tasks WHERE status= $1 ORDER BY created_at ASC LIMIT 1";
        let row_opt = pg_client.query_opt(query, &[&status.to_str()]).await?;
        match row_opt {
            Some(row) => Ok(Some(Self::from_sql(row)?)),
            None => Ok(None),
        }
    }
}
