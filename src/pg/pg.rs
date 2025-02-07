use log::info;
use tokio_postgres::NoTls;

pub struct PgConnect {
    host: String ,
    user: String,
    password: String,
    dbname: String
}
impl Default for PgConnect {
    fn default() -> Self {
        Self {
            host: String::new(),
            user: String::new(),
            password: String::new(),
            dbname: String::new(),
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
    pub async fn connect(&self) -> std::io::Result<()> {
        let conn_str = format!("host={} user={} password={} dbname={}", self.host, self.user, self.password, self.dbname);
        //todo remove unwrap
        let (_client, connection) =tokio_postgres::connect(&conn_str, NoTls).await.unwrap();
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                panic!("Error connecting to db {:?}", e);
            } else {
                info!("Successfully connected to postgres db");
            }
        });
        Ok(())
    }
}