use std::env;
use log::{debug, info};
use tokio_postgres::{Client, Connection, NoTls, Socket};
use tokio_postgres::tls::{NoTlsStream, TlsStream};

pub type PgClient = std::io::Result<Client>;

pub struct PgConnect {
    host: String ,
    user: String,
    password: String,
    dbname: String,
    port: String
}
impl Default for PgConnect {
    fn default() -> Self {
        Self {
            host: String::new(),
            user: String::new(),
            password: String::new(),
            dbname: String::new(),
            port: String::new()
        }
    }
}
impl PgConnect {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn host(&mut self, host: String)-> &mut Self {
        self.host = host;
        self
    }
    pub fn user(&mut self, user: String)-> &mut Self {
        self.user = user;
        self
    }
    pub fn password(&mut self, password: String)-> &mut Self {
        self.password= password;
        self
    }
    pub fn dbname(&mut self, dbname: String)-> &mut Self {
        self.dbname= dbname;
        self
    }
    pub fn port(&mut self, port:String) -> &mut Self {
        self.port = port;
        self
    }
    pub async fn connect(&self) -> PgClient {
        let conn_str = format!("host={} user={} password={} dbname={} port={}",
                               self.host, self.user, self.password, self.dbname, self.port);
        let (client, connection) =tokio_postgres::connect(&conn_str, NoTls).await.unwrap();
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                panic!("Error connecting to db {:?}", e);
            } else {
                info!("Successfully connected to postgres db");
            }
        });
        Ok(client)
    }
}

pub async fn connect_pg_from_env() -> PgClient {
    PgConnect::new()
        .dbname(env::var("PG_DB").unwrap())
        .password(env::var("PG_PASS").unwrap())
        .host(env::var("PG_HOST").unwrap())
        .user(env::var("PG_USER").unwrap())
        .port(env::var("PG_PORT").unwrap())
        .connect().await
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
