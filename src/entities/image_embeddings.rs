use crate::common::BotError;
use crate::embeddings::ollama::OllamaVision;
use crate::file::{image_to_base64, new_base64};
use crate::openapi::llm_api::{OpenAI, OpenAIType};
use crate::pg::pg::{DbQuery, PgClient};
use crate::prompts::Prompt;
use async_trait::async_trait;
use log::info;
use pgvector::Vector;
use std::fs;
use tokio_postgres::Row;
use viuer::ViuError::Image;

pub struct ImageEmbeddings {
    embedding: Vector,
    description: String,
    image_path: String,
}

#[async_trait]
impl DbQuery for ImageEmbeddings {
    async fn insert<'a>(&'a self, pg_client: &'a PgClient) -> Result<(), BotError> {
        pg_client
            .query(
                "INSERT INTO \
                image_embeddings (embedding, description, image_path) VALUES \
                ($1, $2, $3)",
                &[&self.embedding, &self.description, &self.image_path],
            )
            .await?;
        Ok(())
    }

    fn from_sql(row: Row) -> Result<Self, BotError>
    where
        Self: Sized,
    {
        Ok(Self {
            embedding: row.try_get("embedding")?,
            description: row.try_get("description")?,
            image_path: row.try_get("image_path")?,
        })
    }
}
impl ImageEmbeddings {
    pub fn new(embedding: Vector, description: &str, image_path: &str) -> Self {
        Self {
            embedding,
            description: description.to_string(),
            image_path: image_path.to_string(),
        }
    }
    pub async fn get_by_path(pg_client: &PgClient, path: String) -> Result<Option<Self>, BotError> {
        let query = "SELECT * from image_embeddings WHERE image_path = $1";
        let row = pg_client.query_opt(query, &[&path]).await?.unwrap();
        Ok(Some(Self::from_sql(row)?))
    }

    pub async fn get_neighbor(pg_client: &PgClient, embedding: Vector) -> Result<(), BotError> {
        let row = pg_client
            .query_one(
                "SELECT * FROM image_embeddings ORDER BY embedding <=> $1 LIMIT 1",
                &[&embedding],
            )
            .await?;
        println!("{:?}", row);
        Ok(())
    }
    pub async fn pick_and_store_reviewed_images(pg_client: &PgClient) -> Result<(), BotError> {
        let paths = fs::read_dir("./alt_images")?;
        let chat_ai = OpenAI::new(OpenAIType::Chat)?;
        let embedding_ai = OpenAI::new(OpenAIType::Embedding)?;
        let prompt = Prompt::image_description();

        for file in paths {
            let file_name = file?.path();
            let file_name_str = file_name.to_str().unwrap();
            let image_encoded = new_base64(file_name_str);
            let description = chat_ai
                .send_image_with_prompt(&prompt, image_encoded)
                .await?;
            let response = embedding_ai.embeddings(&description).await.unwrap();
            ImageEmbeddings::new(
                response.data.first().unwrap().embedding.clone().into(),
                &description,
                file_name_str,
            )
            .insert(pg_client)
            .await?;
        }
        Ok(())
    }
    // get score based on description "emo_girl"
    pub async fn get_score_of_prompt(pg_client: &PgClient, prompt: &str) -> Result<i16, BotError> {
        let embedding_ai = OpenAI::new(OpenAIType::Embedding)?;
        let response = embedding_ai.embeddings(prompt).await.unwrap();
        let query = "SELECT * FROM image_embeddings ORDER BY embedding <-> $1 LIMIT 5;";
        let vector = Vector::from(response.data.first().unwrap().embedding.clone());
        let rows = pg_client.query(query, &[&vector]).await?;
        for row in rows {
            let path: &str = row.try_get("image_path").unwrap();
            info!("{:?}", path);
        }
        Ok(1)
    }
}
pub async fn get_score_of_image(pg_client: &PgClient, path: &str) -> Result<i16, BotError> {
    let embedding = ImageEmbeddings::get_by_path(pg_client, path.to_string())
        .await?
        .unwrap();
    let query = "SELECT * FROM image_embeddings ORDER BY embedding <-> $1 LIMIT 5;";
    let rows = pg_client.query(query, &[&embedding.embedding]).await?;
    for row in rows {
        info!("{:?}", row);
    }
    Ok(1)
}
