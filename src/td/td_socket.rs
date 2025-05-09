use crate::pg::pg::PgClient;
use crate::td::read::parse_message;
use crate::td::td_json::receive;
use log::error;

pub async fn start_td_socket(pg_client: PgClient) {
    loop {
        let msg = tokio::task::spawn_blocking(|| {
            receive(2.0) // td_receive
        })
        .await
        .unwrap_or_else(|e| {
            error!("{:?}", e);
            panic!("Tokio task spawn blocking");
        });

        if let Some(x) = msg {
            // println!("X -> {x}");
            parse_message(&pg_client, &x)
                .await
                .unwrap_or_else(|e| error!("Parse message {:?}", e));
        }
    }
}
