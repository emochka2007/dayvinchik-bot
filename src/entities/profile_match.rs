use crate::pg::pg::PgClient;
use tokio_postgres::Error;

#[derive(Debug)]
pub struct ProfileMatch {
    pub(crate) url: String,
    pub(crate) full_text: String,
    //todo image
}
impl ProfileMatch {
    // todo create common trait for insert_db
    pub async fn insert_db(&self, client: &PgClient) -> Result<(), Error> {
        //todo check for fields before insert
        if self.url.is_empty() {}
        let query = "INSERT INTO matches (url, full_text)\
        VALUES ($1, $2)";
        client.query(query, &[&self.url, &self.full_text]).await?;
        Ok(())
    }
}

pub async fn _profile_match_by_link(client: &PgClient, link: &str) -> Result<ProfileMatch, Error> {
    let query = "SELECT url, full_text FROM matches WHERE url = $1";
    let row = client.query_one(query, &[&link]).await?;
    let profile_match = ProfileMatch {
        url: row.try_get("url")?,
        full_text: row.try_get("full_text")?,
    };
    Ok(profile_match)
}
