use std::error::Error;
use std::io;
use tokio_postgres::types::{FromSql, Type};

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum RequestKeys {
    GetChatHistory,
    SearchPublicChat,
    DownloadFile,
    GetChat,
    GetChats,
    Unknown,
}
impl RequestKeys {
    pub fn to_str(&self) -> &str {
        match self {
            RequestKeys::GetChatHistory => "getChatHistory",
            RequestKeys::DownloadFile => "downloadFile",
            RequestKeys::GetChat => "getChat",
            RequestKeys::SearchPublicChat => "searchPublicChat",
            RequestKeys::Unknown => "unknown",
            RequestKeys::GetChats => "getChats"
        }
    }
    pub fn from_str(data: &str) -> io::Result<Self> {
        match data {
            "getChatHistory" => Ok(RequestKeys::GetChatHistory),
            "downloadFile" => Ok(RequestKeys::DownloadFile),
            "getChat" => Ok(RequestKeys::GetChat),
            "searchPubliChat" => Ok(RequestKeys::SearchPublicChat),
            _ => Ok(RequestKeys::Unknown)
        }
    }
}
impl FromSql<'_> for RequestKeys {
    fn from_sql<'a>(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let string_from_db = String::from_utf8(raw.to_vec()).expect("Invalid UTF-8");
        match string_from_db.as_str() {
            "getChatHistory" => Ok(RequestKeys::GetChatHistory),
            "downloadFile" => Ok(RequestKeys::DownloadFile),
            "getChat" => Ok(RequestKeys::GetChat),
            "searchPubliChat" => Ok(RequestKeys::SearchPublicChat),
            "getChats" => Ok(RequestKeys::GetChats),
            _ => Ok(RequestKeys::Unknown)
        }
    }

    //todo fix type
    fn accepts(ty: &Type) -> bool {
        true
    }
}
