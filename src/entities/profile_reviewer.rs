use crate::common::BotError;
use crate::entities::dv_bot::DvBot;
use crate::file::{file_exists, get_image_with_retries, move_file};
use crate::openapi::llm_api::OpenAI;
use crate::pg::pg::{DbQuery, DbStatusQuery, PgClient};
use crate::prompts::Prompt;
use async_trait::async_trait;
use log::{debug, error};
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use tokio_postgres::types::{FromSql, Type};
use tokio_postgres::Row;
use uuid::Uuid;
use crate::main;
use crate::td::td_file::td_file_download;

#[derive(Debug)]
pub enum ProfileReviewerStatus {
    Waiting,
    Pending,
    Complete,
    Failed,
    Processed,
}
impl ProfileReviewerStatus {
    pub fn to_str(&self) -> Result<&str, BotError> {
        match self {
            Self::Waiting => Ok("WAITING"),
            Self::Pending => Ok("PENDING"),
            Self::Complete => Ok("COMPLETE"),
            Self::Failed => Ok("FAILED"),
            Self::Processed => Ok("PROCESSED"),
            // _ => Err(BotError::from(io::Error::new(
            //     ErrorKind::NotFound,
            //     "ProfileReviewerStatus not found",
            // ))),
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
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let string_from_db = String::from_utf8(raw.to_vec()).expect("Invalid UTF-8");
        match string_from_db.as_str() {
            "WAITING" => Ok(ProfileReviewerStatus::Waiting),
            "PENDING" => Ok(ProfileReviewerStatus::Pending),
            "COMPLETE" => Ok(ProfileReviewerStatus::Complete),
            "FAILED" => Ok(ProfileReviewerStatus::Failed),
            "PROCESSED" => Ok(ProfileReviewerStatus::Processed),
            _ => Err(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "Profile Reviewer status not found",
            ))),
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
    local_img_path: String,
    file_ids: Option<Vec<i32>>,
}

#[async_trait]
impl DbQuery for ProfileReviewer {
    async fn insert<'a>(&'a self, pg_client: &'a PgClient) -> Result<(), BotError> {
        let query = "INSERT into profile_reviewers (\
        chat_id, \
        text, \
        status,\
        file_ids, \
        local_img_path ) \
        VALUES ($1,$2,$3,$4,$5)";
        let file_ids = self.file_ids.clone().unwrap();
        pg_client
            .query(
                query,
                &[
                    &self.chat_id,
                    &self.text,
                    &"WAITING",
                    &file_ids,
                    &self.local_img_path,
                ],
            )
            .await?;
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
            local_img_path: row.try_get("local_img_path")?,
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
    ) -> Result<Option<Self>, BotError> {
        let query = "SELECT * from profile_reviewers WHERE status = $1 LIMIT 1";
        let row_opt = pg_client.query_opt(query, &[&status.to_str()?]).await?;
        match row_opt {
            Some(row) => Ok(Some(Self::from_sql(row)?)),
            None => Ok(None),
        }
    }
}
// todo implement diff struct ProfileReviewerDb
impl ProfileReviewer {
    pub fn new(chat_id: i64, text: &String, status: ProfileReviewerStatus, local_img_path: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            chat_id,
            text: text.to_string(),
            status,
            score: None,
            file_ids: None,
            local_img_path,
        }
    }
    pub fn score(&self) -> &Option<i32> {
        &self.score
    }
    pub fn _status(&self) -> &ProfileReviewerStatus {
        &self.status
    }
    pub fn local_img_path(&self) -> &str {
        &self.local_img_path
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
    pub fn main_file(&self) -> Option<i32> {
        if let Some(file_ids) = &self.file_ids.clone()?.first() {
            return Some(**file_ids);
        }
        None
    }

    pub async fn acquire(pg_client: &PgClient) -> Result<Option<()>, BotError> {
        let query = "SELECT id from profile_reviewers WHERE status <> $1 \
        AND status <> $2";
        // If no running reviewers then we can run new profile_reviewer
        let rows = pg_client
            .query_opt(
                query,
                &[
                    &ProfileReviewerStatus::Processed.to_str()?,
                    &ProfileReviewerStatus::Failed.to_str()?,
                ],
            )
            .await?;
        if rows.is_some() {
            return Ok(None);
        }
        Ok(Some(()))
    }
    /// Return waiting reviewer
    /// If no pending or in complete status return
    pub async fn acquire_last_waiting(client: &PgClient) -> Result<Option<Self>, BotError> {
        let query = "SELECT id from profile_reviewers WHERE status = $1 OR status = $2";
        // If no running reviewers then we can run new profile_reviewer
        let rows = client
            .query_opt(
                query,
                &[
                    &ProfileReviewerStatus::Pending.to_str()?,
                    &ProfileReviewerStatus::Complete.to_str()?,
                ],
            )
            .await?;
        if rows.is_some() {
            return Ok(None);
        }
        Self::get_by_status_one(client, ProfileReviewerStatus::Waiting).await
    }

    pub async fn finalize(&self, client: &PgClient, score: i32) -> Result<(), BotError> {
        let query = "UPDATE profile_reviewers SET \
        status=$1, \
        score=$2 \
        WHERE id=$3";
        client
            .query(
                query,
                &[&ProfileReviewerStatus::Complete.to_str()?, &score, &self.id],
            )
            .await?;
        Ok(())
    }

    /// If there's a PENDING profile_reviewer -> return None
    /// If there's no WAITING profile_reviewer -> return None
    /// Returns profile_reviewer in COMPLETE status
    pub async fn get_ready_to_proceed(client: &PgClient) -> Result<Option<Self>, BotError> {
        // let pending_reviewer =
        //     Self::get_by_status_one(client, ProfileReviewerStatus::Pending).await?;
        // if pending_reviewer.is_some() {
        //     return Ok(None);
        // }
        // let waiting_reviewer =
        //     Self::get_by_status_one(client, ProfileReviewerStatus::Waiting).await?;
        // if waiting_reviewer.is_none() {
        //     return Ok(None);
        // }
        let completed_reviewer =
            Self::get_by_status_one(client, ProfileReviewerStatus::Complete).await?;
        Ok(completed_reviewer)
    }

    pub async fn review(
        profile_reviewer: &ProfileReviewer,
        pg_client: &PgClient,
    ) -> Result<(), BotError> {
        profile_reviewer
            .update_status(pg_client, ProfileReviewerStatus::Pending)
            .await?;
        let open_ai = OpenAI::new()?;
        let prompt = Prompt::analyze_alt();
        let file_id = profile_reviewer.main_file().unwrap();
        let main_file = format!("profile_images/{file_id}.png");
        let base64_image = get_image_with_retries(&main_file, &profile_reviewer.local_img_path).await?;
        let response = open_ai
            .send_sys_image_message(prompt.system.unwrap(), prompt.user, base64_image)
            .await?;
        let score = response.parse::<i32>()?;
        profile_reviewer.finalize(pg_client, score).await?;
        let reviewed_file = format!("reviewed_images/{}.png", profile_reviewer.id());
        move_file(&main_file, &reviewed_file)?;
        Ok(())
    }

    pub async fn start(pg_client: &PgClient) -> Result<(), BotError> {
        match ProfileReviewer::acquire_last_waiting(pg_client).await? {
            Some(profile_reviewer) => {
                //Check here for file existence
                let file_id = profile_reviewer.main_file().unwrap();
                let main_file = format!("profile_images/{file_id}.png");
                if file_exists(&main_file) {
                    match Self::review(&profile_reviewer, pg_client).await {
                        Ok(_) => Ok(()),
                        Err(e) => {
                            // If profile_reviewer failed, then we send dislike and set to failed
                            profile_reviewer
                                .update_status(pg_client, ProfileReviewerStatus::Failed)
                                .await?;
                            Err(e)
                        }
                    }
                } else {
                    error!("PV start -> File doesnt exist {main_file}");
                    td_file_download(pg_client, file_id).await?;
                    Ok(())
                }
            }
            None => Ok(()),
        }
    }
}
