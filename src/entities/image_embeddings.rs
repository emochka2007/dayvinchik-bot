use crate::common::BotError;
use crate::embeddings::ollama::OllamaVision;
use crate::file::image_to_base64;
use crate::pg::pg::{DbQuery, PgClient};
use async_trait::async_trait;
use log::info;
use pgvector::Vector;
use std::fs;
use tokio_postgres::Row;

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
        }
        Ok(())
    }
}
pub async fn get_and_store_embedding(pg_client: &PgClient) -> Result<(), BotError> {
    let popusk_base64 = image_to_base64("alt_images/popusk.jpg").unwrap();
    let ollama_vision = OllamaVision::new();
    let description = ollama_vision.describe_image(popusk_base64).await.unwrap();
    let vector = ollama_vision
        .get_image_embedding(description.as_str())
        .await
        .unwrap();
    let embedding = ImageEmbeddings {
        embedding: vector,
        description,
        image_path: "popusk".to_string(),
    };
    embedding.insert(pg_client).await?;
    Ok(())
}
