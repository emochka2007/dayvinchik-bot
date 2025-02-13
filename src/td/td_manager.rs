use std::io;
use log::info;
use rust_tdlib::tdjson::ClientId;
use tokio_postgres::{Error, Row};
use uuid::Uuid;
use crate::pg::pg::PgClient;
use crate::td::td_command_map::ResponseKeys;
use crate::td::td_json::send;

#[derive(Debug)]
pub struct TdManager {
    current_task: Task,
    client_id: ClientId,
}
#[derive(Debug)]
enum TaskStatus {
    WAITING,
    COMPLETED,
}
impl TaskStatus {
    pub fn to_str(&self) -> String {
        match self {
            TaskStatus::WAITING => String::from("WAITING"),
            TaskStatus::COMPLETED => String::from("COMPLETED"),
        }
    }
}
#[derive(Debug)]
pub struct Task {
    // todo mb client_id: ClientId,
    id: Uuid,
    message: String,
    status: TaskStatus,
    response: ResponseKeys,
}
impl Task {
    pub fn new(message: String, response: ResponseKeys) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            message,
            response,
            status: TaskStatus::WAITING,
        }
    }
    pub async fn insert_db(&self, pg_client: &PgClient) -> Result<(), Error> {
        let query = "Insert into tasks \
        (id, message, status, response)
        VALUES
        ($1, $2, $3, $4)";
        pg_client.query(query, &[
            &self.id,
            &self.message,
            &self.status.to_str(),
            &self.response.to_str()]).await?;
        Ok(())
    }
    pub fn message(&self) -> &String {
        &self.message
    }
    pub async fn first_waiting(pg_client: &PgClient) -> Result<Self, Error> {
        let query = "SELECT * from tasks WHERE status='WAITING' LIMIT 1";
        match pg_client.query_one(query, &[]).await {
            Ok(row) => {
                Ok(Self {
                    id: row.try_get("id")?,
                    message: row.try_get("message")?,
                    status: TaskStatus::WAITING,
                    response: row.try_get("response")?,
                })
            }
            Err(_) => { panic!("error 70 td_maanger") }
        }
    }
}
/**
Pulls from db events and executes them
*/
impl TdManager {
    pub async fn init(client_id: ClientId, pg_client: &PgClient) -> Self {
        let task = Task::first_waiting(pg_client).await.unwrap();
        // get from sql todo
        Self {
            current_task: task,
            client_id,
        }
    }

    pub fn send_request(&self) -> io::Result<()> {
        let message = self.current_task.message();
        send(self.client_id, &message);
        Ok(())
    }

    // pub fn receive_webhook(&mut self, incoming: &str) {
    //     let message = self.current_task.message();
    //         if response == incoming {
    //             debug!("Popping from");
    //             self.current_task.remove(0);
    //         }
    //     }
}

