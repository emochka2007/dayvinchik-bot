use crate::entities::task::Task;
use crate::pg::pg::PgClient;
use crate::td::td_request::RequestKeys;
use crate::td::td_response::ResponseKeys;
use anyhow::Result;
use rust_tdlib::types::DownloadFile;

const PRIORITY: i32 = 16;
const LIMIT: i32 = 1;

//todo downloads two times fix in read.ts mb
pub async fn td_file_download(pg_client: &PgClient, file_id: i32) -> Result<()> {
    let download_msg = DownloadFile::builder()
        .file_id(file_id)
        .limit(LIMIT)
        .priority(PRIORITY)
        .build();
    let message = serde_json::to_string(&download_msg)?;
    Task::new(
        message,
        RequestKeys::DownloadFile,
        ResponseKeys::UpdateFile,
        pg_client,
    )
    .await?;
    Ok(())
}
