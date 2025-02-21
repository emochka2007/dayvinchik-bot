use crate::common::BotError;
use crate::entities::dv_bot::DvBot;
use crate::file::{get_image_with_retries, image_to_base64, move_file};
use crate::openapi::llm_api::OpenAI;
use crate::pg::pg::{DbQuery, DbStatusQuery, PgClient};
use crate::prompts::Prompt;
use crate::td::td_json::ClientId;
use log::{debug, error};
use std::io;
use std::time::Duration;
use tokio::time::sleep;
use tokio_postgres::types::IsNull::No;
use tokio_postgres::{Client, Error as PostgresError, Row};
use uuid::Uuid;

#[derive(Debug)]
pub enum ProfileReviewerStatus {
    WAITING,
    PENDING,
    COMPLETE,
    FAILED,
    //todo processed status
}

//todo implement multi-image storing
#[derive(Debug)]
pub struct ProfileReviewer {
    id: String,
    chat_id: i64,
    score: Option<i32>,
    text: String,
    status: ProfileReviewerStatus,
    file_ids: Option<Vec<i32>>,
}
impl DbQuery for ProfileReviewer {}
impl DbStatusQuery for ProfileReviewer {}
// todo implement diff struct ProfileReviewerDb
impl ProfileReviewer {
    pub fn new(chat_id: i64, text: &String, status: ProfileReviewerStatus) -> Self {
        Self {
            //todo make uuid
            id: Uuid::new_v4().to_string(),
            chat_id,
            text: text.to_string(),
            status,
            score: None,
            file_ids: None,
        }
    }
    pub fn score(&self) -> &Option<i32> {
        &self.score
    }
    pub fn _status(&self) -> &ProfileReviewerStatus {
        &self.status
    }
    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn _file_ids(&self) -> &Option<Vec<i32>> {
        &self.file_ids
    }
    pub fn set_file_ids(&mut self, file_ids: Option<Vec<i32>>) {
        self.file_ids = file_ids;
    }
    pub fn main_file(&self) -> i32 {
        *self.file_ids.clone().unwrap().get(0).unwrap()
    }
    pub async fn get_waiting(client: &PgClient) -> Result<ProfileReviewer, PostgresError> {
        let query = "SELECT id::text, chat_id, text, file_ids FROM profile_reviewers WHERE status='WAITING' LIMIT 1";
        match client.query_one(query, &[]).await {
            Ok(row) => Self::from_sql(row),
            Err(e) => {
                // error!("Failed to execute get_waiting query: {}", e);
                Err(e)
            }
        }
    }
    pub async fn get_completed(client: &PgClient) -> Result<ProfileReviewer, PostgresError> {
        let query = "SELECT id::text, chat_id, text, file_ids,score FROM profile_reviewers WHERE status='COMPLETE' LIMIT 1";
        match client.query_one(query, &[]).await {
            Ok(row) => Self::from_sql(row),
            Err(e) => {
                // error!("Failed to execute get_completed query: {}", e);
                Err(e)
            }
        }
    }

    // pub async fn get_by_file_id(pg_client: &PgClient) -> Result<ProfileReviewer, PostgresError> {}

    //todo convert status
    fn from_sql(row: Row) -> Result<ProfileReviewer, PostgresError> {
        Ok(Self {
            chat_id: row.try_get("chat_id")?,
            score: Some(row.try_get("score").unwrap_or_default()),
            id: row.try_get("id")?,
            text: row.try_get("text")?,
            status: ProfileReviewerStatus::PENDING,
            file_ids: Some(row.try_get("file_ids")?),
        })
    }

    pub async fn run(client: &PgClient) -> Result<ProfileReviewer, PostgresError> {
        let query = "SELECT id::text, chat_id, text, file_ids FROM profile_reviewers WHERE status='WAITING' LIMIT 1";
        match client.query_one(query, &[]).await {
            Ok(row) => {
                let id = row.try_get("id")?;
                match Self::set_pending(id, client).await {
                    Ok(_) => Self::from_sql(row),
                    Err(e) => {
                        // error!("Failed to execute set_pending query: {}", e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                // error!("Failed to execute start query: {}", e);
                Err(e)
            }
        }
    }
    pub async fn set_pending(id: String, client: &PgClient) -> Result<(), PostgresError> {
        let query = "UPDATE profile_reviewers SET status='PENDING' WHERE id=$1";
        let id = &Uuid::parse_str(&id).unwrap();
        client.query(query, &[id]).await?;
        Ok(())
    }
    pub async fn set_processed(id: String, client: &PgClient) -> Result<(), PostgresError> {
        let query = "UPDATE profile_reviewers SET status='PROCESSED' WHERE id=$1";
        let id = &Uuid::parse_str(&id).unwrap();
        client.query(query, &[id]).await?;
        Ok(())
    }

    pub async fn acquire(client: &PgClient) -> Result<Option<()>, PostgresError> {
        let query =
            "SELECT id from profile_reviewers WHERE status = 'PENDING' OR status='COMPLETE'";
        let rows_len = client.query(query, &[]).await?.len();
        // If no running reviewers then we can run new profile_reviewer
        match client.query(query, &[]).await {
            Ok(rows) => {
                if rows.len() == 0 {
                    match Self::get_waiting(client).await {
                        Ok(_) => Ok(Some(())),
                        Err(_e) => Ok(None),
                    }?;
                    return Ok(Some(()));
                } else {
                    Ok(None)
                }
            }
            Err(e) => {
                //todo match for row count error only here
                Ok(Some(()))
            }
        }
    }

    pub async fn get_ready_to_proceed(client: &PgClient) -> Result<Option<()>, PostgresError> {
        let query = "SELECT id from profile_reviewers WHERE status = 'PENDING' OR status='WAITING'";
        match client.query(query, &[]).await {
            Ok(rows) => {
                if rows.len() == 0 {
                    Ok(Some(()))
                } else {
                    Ok(None)
                }
            }
            Err(e) => {
                //todo match for row count error only here
                Ok(Some(()))
            }
        }
    }

    pub async fn insert_db(&self, client: &PgClient) -> Result<(), PostgresError> {
        if let Ok(Some(_)) = Self::acquire(client).await {
            let query = "INSERT into profile_reviewers (chat_id, text, status,file_ids) \
        VALUES ($1,$2,$3,$4)";
            let file_ids = self.file_ids.clone().unwrap();
            client
                .query(query, &[&self.chat_id, &self.text, &"WAITING", &file_ids])
                .await?;
        } else {
            debug!("Profile reviewer is still pending. Cannot insert new one");
        }
        Ok(())
    }

    pub async fn finalize(&self, client: &PgClient, score: i32) -> Result<(), PostgresError> {
        let query = "UPDATE profile_reviewers SET \
        status='COMPLETE', \
        score=$1 \
        WHERE id=$2";
        let id = &Uuid::parse_str(&self.id).unwrap();
        client.query(query, &[&score, id]).await?;
        Ok(())
    }
    pub async fn to_failed(&self, pg_client: &PgClient) -> Result<(), PostgresError> {
        let query = "UPDATE profile_reviewers SET \
        status='FAILED' \
        WHERE id=$1";
        let id = &Uuid::parse_str(&self.id).unwrap();
        pg_client.query(query, &[id]).await?;
        Ok(())
    }
    pub async fn start(pg_client: &PgClient) -> Result<(), BotError> {
        match ProfileReviewer::acquire(pg_client).await? {
            Some(_) => {
                let last_pending = ProfileReviewer::run(pg_client).await?;
                let open_ai = OpenAI::new();
                let prompt = Prompt::analyze_alt();
                let path_to_img = format!("profile_images/{}.png", last_pending.main_file());
                match get_image_with_retries(&path_to_img).await {
                    Ok(base64_image) => {
                        let response = open_ai
                            .send_sys_image_message(
                                prompt.system.unwrap(),
                                prompt.user,
                                base64_image,
                            )
                            .await
                            .unwrap();
                        match response.parse::<i32>() {
                            Ok(score) => {
                                last_pending
                                    .finalize(pg_client, score)
                                    .await
                                    .expect("TODO: panic message");
                                let reviewed_file =
                                    format!("reviewed_images/{}.png", last_pending.id());
                                move_file(&path_to_img, &reviewed_file)
                                    .expect("TODO: panic message");
                            }
                            Err(e) => {
                                last_pending.to_failed(&pg_client).await?;
                                error!("Response parsing error {:?}", e);
                                return Err(e);
                            }
                        }
                    }
                    Err(e) => {
                        last_pending.to_failed(&pg_client).await?;
                        return Err(e);
                    }
                }
            }
            None => {}
        }
        Ok(())
    }
}
