use crate::common::BotError;
use crate::embeddings::ollama::OllamaVision;
use crate::file::{image_to_base64, new_base64};
use crate::pg::pg::{DbQuery, PgClient};
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
        let paths = fs::read_dir("./reviewed_images")?;
        for file in paths {
            let file_name = file?.path();
            let image_encoded = new_base64(file_name.to_str().unwrap());
            let ollama_vision = OllamaVision::new();
            let description = ollama_vision.describe_image(image_encoded).await?;
            let vector = ollama_vision
                .get_image_embedding(description.as_str())
                .await?;
            let embedding = ImageEmbeddings {
                embedding: vector,
                description,
                image_path: file_name.to_str().unwrap().to_string(),
            };
            embedding.insert(pg_client).await?;
        }
        Ok(())
    }
    // get score based on description "emo_girl"
    pub async fn get_score_of_prompt(pg_client: &PgClient, prompt: &str) -> Result<i16, BotError> {
        let ollama_vision = OllamaVision::new();
        let vector = ollama_vision.get_image_embedding(prompt).await?;
        let query = "SELECT * FROM image_embeddings ORDER BY embedding <-> $1 LIMIT 5;";
        let rows = pg_client.query(query, &[&vector]).await?;
        for row in rows {
            let path: &str = row.try_get("image_path").unwrap();
            info!("{:?}", path);
        }
        Ok(1)
    }
}
pub async fn get_and_store_embedding(pg_client: &PgClient) -> Result<(), BotError> {
    ImageEmbeddings::pick_and_store_reviewed_images(pg_client).await?;
    Ok(())
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
