use log::{debug, error};
use tokio_postgres::{Client, Error};
use uuid::Uuid;
use crate::pg::pg::PgClient;

#[derive(Debug)]
pub enum ProfileReviewerStatus {
    PENDING,
    COMPLETED
}

//todo implement multi-image storing
#[derive(Debug)]
pub struct ProfileReviewer {
    id: String,
    chat_id: i64,
    score: Option<i16>,
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
            score: None,
            file_ids: None
        }
    }
    pub fn _score(&self) -> &Option<i16> {
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
    pub fn main_file(&self)-> i32 {
        *self.file_ids.clone().unwrap().get(0).unwrap()
    }
    pub async fn get_last_pending(client: &Client) -> Result<ProfileReviewer, Error> {
        let query = "SELECT id::text, chat_id, text, file_ids FROM profile_reviewers WHERE status='PENDING' LIMIT 1";
        match client.query_one(query, &[]).await {
            Ok(row) => {
                debug!("{:?}", row); // Log the row data
                Ok(Self {
                    chat_id: row.try_get("chat_id")?,
                    score: row.try_get("score").unwrap_or_default(),
                    id: row.try_get("id")?,
                    text: row.try_get("text")?,
                    status: ProfileReviewerStatus::PENDING,
                    file_ids: Some(row.try_get("file_ids")?)
                })
            },
            Err(e) => {
                error!("Failed to execute query: {}", e); // Log errors
                Err(e)
            }
        }
    }


    pub async fn insert_db(&self, client: &PgClient) -> Result<(),Error>{
        let query = "INSERT into profile_reviewers (chat_id, text, status,file_ids) \
        VALUES ($1,$2,$3,$4)";
        let file_ids = self.file_ids.clone().unwrap();
        client.query(query, &[
            &self.chat_id,
            &self.text,
            &"PENDING",
            &file_ids
        ]).await?;
        Ok(())
    }

    pub async fn finalize(&self, client: &PgClient, score: i32) -> Result<(), Error> {
        let query = "UPDATE profile_reviewers SET \
        status='COMPLETED', \
        score=$1 \
        WHERE id=$2";
        //todo make it clean
        client.query(query, &[&score,
            &Uuid::parse_str(&self.id).unwrap()]).await?;
        Ok(())
    }
}