use crate::common::BotError;
use crate::entities::dv_bot::DvBot;
use crate::file::{get_image_with_retries, image_to_base64, move_file};
use crate::openapi::llm_api::OpenAI;
use crate::pg::pg::{DbQuery, DbStatusQuery, PgClient};
use crate::prompts::Prompt;
use crate::td::td_json::ClientId;
use crate::td::td_request::RequestKeys;
use async_trait::async_trait;
use log::{debug, error};
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::time::Duration;
use tokio::time::sleep;
use tokio_postgres::types::IsNull::No;
use tokio_postgres::types::{FromSql, Type};
use tokio_postgres::{Client, Error as PostgresError, Row};
use uuid::Uuid;

#[derive(Debug)]
pub enum ProfileReviewerStatus {
    WAITING,
    PENDING,
    COMPLETE,
    FAILED,
    PROCESSED,
}
impl ProfileReviewerStatus {
    pub fn to_str(&self) -> Result<&str, BotError> {
        match self {
            Self::WAITING => Ok("WAITING"),
            Self::PENDING => Ok("PENDING"),
            Self::COMPLETE => Ok("COMPLETE"),
            Self::FAILED => Ok("FAILED"),
            Self::PROCESSED => Ok("PROCESSED"),
            _ => Err(BotError::from(io::Error::new(
                ErrorKind::NotFound,
                "ProfileReviewerStatus not found",
            ))),
        }
    }
    // pub fn from_str(data: &str) -> io::Result<Self> {
    //     match data {
    //         "getChatHistory" => Ok(RequestKeys::GetChatHistory),
    //         "downloadFile" => Ok(RequestKeys::DownloadFile),
    //         "getChat" => Ok(RequestKeys::GetChat),
    //         "searchPublicChat" => Ok(RequestKeys::SearchPublicChat),
    //         "sendMessage" => Ok(RequestKeys::SendMessage),
    //         "openChat" => Ok(RequestKeys::OpenChat),
    //         _ => Ok(RequestKeys::Unknown),
    //     }
    // }
}
impl FromSql<'_> for ProfileReviewerStatus {
    fn from_sql<'a>(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let string_from_db = String::from_utf8(raw.to_vec()).expect("Invalid UTF-8");
        match string_from_db.as_str() {
            "WAITING" => Ok(ProfileReviewerStatus::WAITING),
            "PENDING" => Ok(ProfileReviewerStatus::PENDING),
            "COMPLETE" => Ok(ProfileReviewerStatus::COMPLETE),
            "FAILED" => Ok(ProfileReviewerStatus::FAILED),
            "PROCESSED" => Ok(ProfileReviewerStatus::PROCESSED),
            _ => Box::new(std::io::Error::new(
                ErrorKind::NotFound,
                "Profile Reviewer status not found",
            ))
                .into(),
        }
    }

    //todo fix type
    fn accepts(_ty: &Type) -> bool {
        true
    }
}

//todo implement multi-image storing
#[derive(Debug)]
pub struct ProfileReviewer {
    id: Uuid,
    chat_id: i64,
    score: Option<i32>,
    text: String,
    status: ProfileReviewerStatus,
    file_ids: Option<Vec<i32>>,
}

#[async_trait]
impl DbQuery for ProfileReviewer {
    async fn insert<'a>(&'a self, pg_client: &'a PgClient) -> Result<(), BotError> {
        if let Ok(Some(_)) = Self::acquire(pg_client).await {
            let query = "INSERT into profile_reviewers (chat_id, text, status,file_ids) \
        VALUES ($1,$2,$3,$4)";
            let file_ids = self.file_ids.clone().unwrap();
            pg_client
                .query(query, &[&self.chat_id, &self.text, &"WAITING", &file_ids])
                .await?;
        } else {
            debug!("Profile reviewer is still pending. Cannot insert new one");
        }
        Ok(())
    }

    async fn select_one(pg_client: &PgClient, id: Uuid) -> Result<Self, BotError>
    where
        Self: Sized,
    {
        let query = "SELECT * from profile_reviewers id = $1";
        let row = pg_client.query_one(query, &[&id]).await?;
        Ok(Self::from_sql(row)?)
    }

    fn from_sql(row: Row) -> Result<Self, BotError>
    where
        Self: Sized,
    {
        Ok(Self {
            chat_id: row.try_get("chat_id")?,
            score: Some(row.try_get("score")?),
            id: row.try_get("id")?,
            text: row.try_get("text")?,
            status: row.try_get("status")?,
            file_ids: Some(row.try_get("file_ids")?),
        })
    }
}

#[async_trait]
impl DbStatusQuery for ProfileReviewer {
    type Status = ProfileReviewerStatus;

    async fn update_status<'a>(
        &'a self,
        pg_client: &'a PgClient,
        status: Self::Status,
    ) -> Result<(), BotError> {
        let query = "UPDATE profile_reviewers SET status=$1 WHERE id=$2";
        pg_client
            .query(query, &[&status.to_str()?, &self.id])
            .await?;
        Ok(())
    }

    async fn get_by_status_one(
        pg_client: &PgClient,
        status: Self::Status,
    ) -> Result<Self, BotError> {
        let query = "SELECT * from profile_reviewers WHERE status = $1 LIMIT 1";
        let row = pg_client.query_one(query, &[&status.to_str()?]).await?;
        Ok(Self::from_sql(row)?)
    }
}
// todo implement diff struct ProfileReviewerDb
impl ProfileReviewer {
    pub fn new(chat_id: i64, text: &String, status: ProfileReviewerStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
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
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn _file_ids(&self) -> &Option<Vec<i32>> {
        &self.file_ids
    }
    pub fn set_file_ids(&mut self, file_ids: Vec<i32>) {
        self.file_ids = Some(file_ids);
    }
    pub fn main_file(&self) -> i32 {
        *self.file_ids.clone().unwrap().get(0).unwrap()
    }
    pub async fn get_waiting(client: &PgClient) -> Result<ProfileReviewer, BotError> {
        let query = "SELECT * FROM profile_reviewers WHERE status='WAITING' LIMIT 1";
        let row = client.query_one(query, &[]).await?;
        Ok(Self::from_sql(row)?)
    }
    pub async fn get_completed(client: &PgClient) -> Result<ProfileReviewer, BotError> {
        let query = "SELECT * FROM profile_reviewers WHERE status='COMPLETE' LIMIT 1";
        let row = client.query_one(query, &[]).await?;
        Ok(Self::from_sql(row)?)
    }

    /// Get Waiting -> Set to pending
    /// Returns Self
    pub async fn run(client: &PgClient) -> Result<Self, BotError> {
        let awaiting_reviewer =
            Self::get_by_status_one(client, ProfileReviewerStatus::WAITING).await?;
        awaiting_reviewer
            .update_status(client, ProfileReviewerStatus::PENDING)
            .await?;
        Ok(awaiting_reviewer)
    }

    /// Return waiting reviewer
    pub async fn acquire(client: &PgClient) -> Result<Self, BotError> {
        let query = "SELECT id from profile_reviewers WHERE status = $1 OR status = $2";
        // If no running reviewers then we can run new profile_reviewer
        let rows = client
            .query(
                query,
                &[
                    &ProfileReviewerStatus::PENDING.to_str()?,
                    &ProfileReviewerStatus::COMPLETE.to_str()?,
                ],
            )
            .await?;
        if rows.len() == 0 {
            let awaiting_profile = Self::get_waiting(client).await?;
            return Ok(awaiting_profile);
        }
        Err(io::Error::new(
            ErrorKind::AlreadyExists,
            "Cannot acquire the profile_reviewer",
        )
            .into())
    }

    pub async fn get_ready_to_proceed(client: &PgClient) -> Result<(), BotError> {
        let query = "SELECT id from profile_reviewers WHERE status = 'PENDING' OR status='WAITING'";
        let rows = client.query(query, &[]).await?;
        if rows.len() == 0 {
            Ok(())
        } else {
            Err(io::Error::new(ErrorKind::ResourceBusy, "PROFILE REVIEWER is still pending or no waiting tasks found").into())
        }
    }

    pub async fn start(pg_client: &PgClient) -> Result<(), BotError> {
        let profile_reviewer = ProfileReviewer::acquire(pg_client).await?;
        profile_reviewer
            .update_status(pg_client, ProfileReviewerStatus::PENDING)
            .await?;
        let open_ai = OpenAI::new();
        let prompt = Prompt::analyze_alt();
        let path_to_img = format!("profile_images/{}.png", profile_reviewer.main_file());
        let base64_image = get_image_with_retries(&path_to_img).await?;
        let response = open_ai
            .send_sys_image_message(prompt.system.unwrap(), prompt.user, base64_image)
            .await?;
        let score = response.parse::<i32>()?;
        profile_reviewer.finalize(pg_client, score).await?;
        let reviewed_file = format!("reviewed_images/{}.png", profile_reviewer.id());
        move_file(&path_to_img, &reviewed_file)?;
        Ok(())
    }
}
