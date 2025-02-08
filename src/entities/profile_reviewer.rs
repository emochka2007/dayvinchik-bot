use tokio_postgres::{Client, Error};

enum ProfileReviewerStatus {
    PENDING,
    COMPLETED
}

pub struct ProfileReviewer {
    id: String,
    chat_id: i32,
    status: ProfileReviewerStatus
}
// todo implement diff struct ProfileReviewerDb
impl ProfileReviewer {
    pub async fn get_last_pending(client: Client) -> Result<ProfileReviewer, Error> {
        let query = "SELECT * from profile_reviewers \
        WHERE status=\"pending\"";
        let row = client.query_one(query, &[]).await?;
        Ok(Self {
            chat_id: row.try_get("chat_id")?,
            id: row.try_get("id")?,
            status: ProfileReviewerStatus::PENDING
        })
    }
}