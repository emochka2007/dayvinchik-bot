use crate::constants::update_last_request;
use crate::pg::pg::{PgClient, PgConnect};
use crate::td::td_json::send;
use crate::td::td_request::RequestKeys;
use crate::td::td_response::ResponseKeys;
use log::{debug, error, info};
use rust_tdlib::tdjson::ClientId;
use std::time::Duration;
use tokio_postgres::{Client, Error, Row};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct TdManager {
    client_id: ClientId,
}
#[derive(Debug)]
enum TaskStatus {
    WAITING,
    PENDING,
    COMPLETE,
}
impl TaskStatus {
    pub fn to_str(&self) -> String {
        match self {
            TaskStatus::WAITING => String::from("WAITING"),
            TaskStatus::PENDING => String::from("PENDING"),
            TaskStatus::COMPLETE => String::from("COMPLETE"),
        }
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
            status: TaskStatus::WAITING,
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
    pub async fn new(
        message: String,
        request: RequestKeys,
        response: ResponseKeys,
        pg_client: &PgClient,
    ) -> Result<Self, Error> {
        let id = Uuid::new_v4();
        let task = Self {
            id,
            message,
            response,
            request,
            status: TaskStatus::WAITING,
        };
        task.insert_db(pg_client).await?;
        Ok(task)
    }
    pub async fn insert_db(&self, pg_client: &PgClient) -> Result<(), Error> {
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
    pub fn message(&self) -> &String {
        &self.message
    }
    // todo common trait
    fn parse_row(row: Row) -> Result<Self, Error> {
        Ok(Self {
            id: row.try_get("id")?,
            message: row.try_get("message")?,
            status: TaskStatus::PENDING,
            request: row.try_get("request")?,
            response: row.try_get("response")?,
        })
    }
    pub async fn first_waiting(pg_client: &PgClient) -> Result<Self, Error> {
        let query = "SELECT * from tasks WHERE status='WAITING' ORDER BY created_at LIMIT 1";
        let row = pg_client.query_one(query, &[]).await?;
        Self::parse_row(row)
    }
    pub async fn first_pending(
        pg_client: &PgClient,
        request_key: &RequestKeys,
        response_key: &ResponseKeys,
    ) -> Result<Self, Error> {
        let query = "SELECT * from tasks WHERE status='PENDING' \
        AND request=$1 AND response = $2 \
        ORDER BY created_at LIMIT 1";
        let row = pg_client
            .query_one(query, &[&request_key.to_str(), &response_key.to_str()])
            .await?;
        Self::parse_row(row)
    }

    pub async fn to_pending(&self, pg_client: &PgClient) -> Result<(), Error> {
        let query = "UPDATE tasks SET status='PENDING' \
        WHERE id = $1";
        pg_client.query(query, &[self.id()]).await?;
        Ok(())
    }
    pub async fn to_complete(&self, pg_client: &PgClient) -> Result<(), Error> {
        let query = "UPDATE tasks SET status='COMPLETE' \
        WHERE id = $1";
        pg_client.query(query, &[self.id()]).await?;
        Ok(())
    }
}
/**
Pulls from db events and executes them
*/
impl TdManager {
    pub fn init(client_id: ClientId) -> Self {
        Self { client_id }
    }

    pub async fn send_request(&self, pg_client: &PgClient) -> Result<(), Error> {
        let task = Task::first_waiting(pg_client).await?;
        // error!("task -> {:?}", task);
        update_last_request(task.request);
        task.to_pending(pg_client).await?;
        // tokio::time::sleep(Duration::from_secs(1)).await;
        send(self.client_id, &task.message);
        Ok(())
    }

    // pub async fn start(client_id: ClientId, pg_client: &PgClient) -> Result<(), Error> {
    //     loop {
    //         let td_manager = Self::init(client_id);
    //         info!("Td manager");
    //         td_manager.send_request(pg_client).await?;
    //         info!("Td send");
    //         tokio::time::sleep(Duration::from_secs(1)).await;
    //     }
    //     Ok(())
    // }

    // pub fn receive_webhook(&mut self, incoming: &str) {
    //     let message = self.current_task.message();
    //         if response == incoming {
    //             debug!("Popping from");
    //             self.current_task.remove(0);
    //         }
    //     }
}
