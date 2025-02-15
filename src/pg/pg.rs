use std::env;
use std::env::VarError;
use deadpool_postgres::{Pool, Config, Runtime, Manager, ManagerConfig, RecyclingMethod};
use log::{info};
use tokio_postgres::{Client, NoTls};
const WORKERS: usize = 16;
const ITERATIONS: usize = 1000;
pub type PgClient = deadpool::managed::Object<Manager>;

pub struct PgConnect {
    host: String ,
    user: String,
    password: String,
    dbname: String,
    port: u16
}
impl Default for PgConnect {
    fn default() -> Self {
        Self {
            host: String::new(),
            user: String::new(),
            password: String::new(),
            dbname: String::new(),
            port: 0
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
        let port: u16 = port.parse().unwrap();
        self.port = port;
        self
    }
    pub async fn _connect(&self) -> std::io::Result<Client> {
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
    //todo conitinue
    struct Config {
        #[serde(default)]
        pg: deadpool_postgres::Config,
    }
    //https://github.com/deadpool-rs/deadpool/blob/master/examples/postgres-benchmark/src/main.rs
    async fn with_deadpool(config: &Config) -> () {
        let pool = config
            .pg
            .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
            .unwrap();
        let now = Instant::now();
        let (tx, mut rx) = mpsc::channel::<usize>(16);
        for i in 0..WORKERS {
            let pool = pool.clone();
            let tx = tx.clone();
            tokio::spawn(async move {
                for _ in 0..ITERATIONS {
                    let client = pool.get().await.unwrap();
                    let stmt = client.prepare_cached("SELECT 1 + 2").await.unwrap();
                    let rows = client.query(&stmt, &[]).await.unwrap();
                    let value: i32 = rows[0].get(0);
                    assert_eq!(value, 3);
                }
                tx.send(i).await.unwrap();
            });
        }
        for _ in 0..WORKERS {
            rx.recv().await.unwrap();
        }
    }
    pub async fn create_pool(&self) -> Pool {
        // let mut cfg = Config::new();
        // cfg.host = Some(self.host.clone());
        // cfg.port = Some(self.port.parse::<u16>().unwrap());
        // cfg.password = Some(self.password.clone());
        // cfg.user = Some(self.user.clone());
        // cfg.dbname = Some(self.dbname.clone());
        // cfg.manager = Some(ManagerConfig {
        //     recycling_method: RecyclingMethod::Fast
        // });
        // let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).expect("Failed to create pool");
        // pool
        // Set config  values for tokio_postgres
        let mut pg_config = tokio_postgres::Config::new();
        // pg_config.application_name("rust_tester");
        pg_config.password(self.password.as_str());
        pg_config.user(self.user.as_str());
        pg_config.dbname(self.dbname.as_str());
        pg_config.host(self.host.as_str());
        pg_config.port(self.port);
        // Would be nice to use a postgres connection string, but doesn't work
        // pg_config.host("host1.subdomain.domain.org");
        // pg_config.?????("postgresql://a_user:a_password@10.1.1.10:5432/a_dbname");

        // Create manager config for deadpool_postgres
        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };

        // Instantiate manager and pool for deadpool_postgres
        let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
        let pool = Pool::builder(mgr).max_size(16).build().unwrap();
        pool
    }
}

pub async fn _connect_pg_from_env() -> std::io::Result<Client> {
    PgConnect::new()
        .dbname(env::var("PG_DB").unwrap())
        .password(env::var("PG_PASS").unwrap())
        .host(env::var("PG_HOST").unwrap())
        .user(env::var("PG_USER").unwrap())
        .port(env::var("PG_PORT").unwrap())
        ._connect().await
}
pub async fn create_pool_from_env() -> Result<Pool, VarError> {
    let pool = PgConnect::new()
        .dbname(env::var("PG_DB")?)
        .password(env::var("PG_PASS")?)
        .host(env::var("PG_HOST")?)
        .user(env::var("PG_USER")?)
        .port(env::var("PG_PORT")?)
        .create_pool().await;
    Ok(pool)
}
pub async fn run_migrations(client: &PgClient) {
    // todo get dynamically from folder
    let migration_files = vec!["migrations/init.sql"];
    for file in migration_files {
        let sql = std::fs::read_to_string(file).unwrap();
        client.batch_execute(&sql).await.unwrap();
        info!("Executed migration {sql}");
    }
}
