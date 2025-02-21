use crate::common::BotError;
use deadpool_postgres::{Config, Manager, ManagerConfig, Pool, RecyclingMethod, Runtime};
use log::info;
use std::env;
use async_trait::async_trait;
use dotenvy::Error;
use tokio_postgres::{Client, NoTls};
use uuid::Uuid;

const WORKERS: usize = 16;
const ITERATIONS: usize = 1000;
pub type PgClient = deadpool::managed::Object<Manager>;

pub struct PgConnect {
    host: String,
    user: String,
    password: String,
    dbname: String,
    port: u16,
}
impl PgConnect {
    pub fn host(&mut self, host: String) -> &mut Self {
        self.host = host;
        self
    }
    pub fn user(&mut self, user: String) -> &mut Self {
        self.user = user;
        self
    }
    pub fn password(&mut self, password: String) -> &mut Self {
        self.password = password;
        self
    }
    pub fn dbname(&mut self, dbname: String) -> &mut Self {
        self.dbname = dbname;
        self
    }

    pub fn port(&mut self, port: String) -> Result<&mut Self, BotError> {
        let port: u16 = port.parse()?;
        self.port = port;
        Ok(self)
    }

    pub async fn connect(&self) -> Result<Client, BotError> {
        let conn_str = format!(
            "host={} user={} password={} dbname={} port={}",
            self.host, self.user, self.password, self.dbname, self.port
        );
        match tokio_postgres::connect(&conn_str, NoTls).await {
            Ok((client, connection)) => {
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        panic!("Error on connection to db {:?}", e);
                    } else {
                        info!("Successfully connected to postgres conn_str {}", conn_str);
                    }
                });
                Ok(client)
            }
            Err(e) => {
                panic!("Error connecting to db {:?}", e);
            }
        }
    }

    pub fn create_pool(&self) -> Pool {
        let mut cfg = Config::new();
        cfg.host = Some(self.host.clone());
        cfg.port = Some(self.port);
        cfg.password = Some(self.password.clone());
        cfg.user = Some(self.user.clone());
        cfg.dbname = Some(self.dbname.clone());
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });
        let pool = cfg
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .expect("Failed to create pool");
        pool
    }

    pub fn from_env() -> Result<Self, BotError> {
        let mut pg = Self {
            dbname: env::var("PG_DB")?,
            password: env::var("PG_PASS")?,
            host: env::var("PG_HOST")?,
            user: env::var("PG_USER")?,
            port: 0,
        };
        let port = env::var("PG_PORT")?;
        pg.port(port)?;
        Ok(pg)
    }

    pub async fn connect_pg_from_env() -> Result<Client, BotError> {
        let pg = Self::from_env()?;
        Ok(pg.connect().await?)
    }
    pub fn create_pool_from_env() -> Result<Pool, BotError> {
        let pool = Self::from_env()?;
        Ok(pool.create_pool())
    }

    pub async fn run_migrations(client: &Client) -> Result<(), BotError> {
        // todo get dynamically from folder
        let migration_files = vec!["migrations/init.sql"];
        for file in migration_files {
            let sql = std::fs::read_to_string(file)?;
            client.batch_execute(&sql).await?;
            info!("Executed migration {sql}");
        }
        Ok(())
    }
}

#[async_trait]
pub trait DbQuery {
    async fn insert(&self, pg_client: &PgClient) -> Result<(), BotError>;
    async fn select_one(pg_client: PgClient, id: Uuid) -> Result<Self, BotError>
    where
        Self: Sized;
}
#[async_trait]
pub trait DbStatusQuery {
    type Status;
    async fn update_status(&self, pg_client: &PgClient, status: Self::Status) -> Result<(), BotError>;
    async fn get_by_status_one(&self, pg_client: &PgClient, status: Self::Status) -> Result<(), BotError>;
}