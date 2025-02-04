use rust_tdlib::types::DownloadFile;

pub fn td_file_download(file_id: i32) -> DownloadFile {
    DownloadFile::builder().file_id(file_id).limit(1)
        .priority(16).build()
}