use tokio_postgres::{Client, Error};
use uuid::Uuid;

pub enum ProfileReviewerStatus {
    PENDING,
    COMPLETED
}

pub struct ProfileReviewer {
    id: String,
    chat_id: i64,
    text: String,
    status: ProfileReviewerStatus,
    file_ids: Option<Vec<i32>>
}
// todo implement diff struct ProfileReviewerDb
impl ProfileReviewer {
    pub fn new(chat_id: i64, text: &String, status: ProfileReviewerStatus) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            chat_id,
            text: text.to_string(),
            status,
            file_ids: None
        }
    }
    pub async fn get_last_pending(client: &Client) -> Result<ProfileReviewer, Error> {
        let query = "SELECT * from profile_reviewers \
        WHERE status=\"pending\"";
        let row = client.query_one(query, &[]).await?;
        Ok(Self {
            chat_id: row.try_get("chat_id")?,
            id: row.try_get("id")?,
            text: row.try_get("text")?,
            status: ProfileReviewerStatus::PENDING,
            file_ids: Some(row.try_get("file_ids")?)
        })
    }

    pub async fn insert_db(&self, client: &Client) -> Result<(),Error>{
        let query = "INSERT into profile_reviewers (id, chat_id, text, status,file_ids) \
        VALUES ($1,$2,$3,$4,$5)";
        let row = client.query_one(query, &[
            &self.id,
            &self.chat_id,
            &self.text,
            &"PENDING",
            &self.file_ids
        ]).await?;
        Ok(())
    }
    pub fn file_ids(&self) -> &Option<Vec<i32>> {
        &self.file_ids
    }
}