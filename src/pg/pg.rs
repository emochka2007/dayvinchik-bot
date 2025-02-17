use deadpool_postgres::{Config, Manager, ManagerConfig, Pool, RecyclingMethod, Runtime};
use log::info;
use std::env;
use std::env::VarError;
use tokio_postgres::{Client, NoTls};
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
impl Default for PgConnect {
    fn default() -> Self {
        Self {
            host: String::new(),
            user: String::new(),
            password: String::new(),
            dbname: String::new(),
            port: 0,
        }
    }
}
impl PgConnect {
    pub fn new() -> Self {
        Self::default()
    }

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
    pub fn port(&mut self, port: String) -> &mut Self {
        let port: u16 = port.parse().unwrap();
        self.port = port;
        self
    }
    pub async fn connect(&self) -> std::io::Result<Client> {
        let conn_str = format!(
            "host={} user={} password={} dbname={} port={}",
            self.host, self.user, self.password, self.dbname, self.port
        );
        let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await.unwrap();
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                panic!("Error connecting to db {:?}", e);
            } else {
                // info!("Successfully connected to postgres db");
            }
        });
        Ok(client)
    }

    pub async fn create_pool(&self) -> Pool {
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

    pub async fn connect_pg_from_env() -> std::io::Result<Client> {
        PgConnect::new()
            .dbname(env::var("PG_DB").unwrap())
            .password(env::var("PG_PASS").unwrap())
            .host(env::var("PG_HOST").unwrap())
            .user(env::var("PG_USER").unwrap())
            .port(env::var("PG_PORT").unwrap())
            .connect()
            .await
    }
    pub async fn create_pool_from_env() -> Result<Pool, VarError> {
        let pool = PgConnect::new()
            .dbname(env::var("PG_DB")?)
            .password(env::var("PG_PASS")?)
            .host(env::var("PG_HOST")?)
            .user(env::var("PG_USER")?)
            .port(env::var("PG_PORT")?)
            .create_pool()
            .await;
        Ok(pool)
    }
    pub async fn run_migrations(client: &Client) {
        // todo get dynamically from folder
        let migration_files = vec!["migrations/init.sql"];
        for file in migration_files {
            let sql = std::fs::read_to_string(file).unwrap();
            client.batch_execute(&sql).await.unwrap();
            info!("Executed migration {sql}");
        }
    }
}
