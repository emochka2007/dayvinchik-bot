use std::collections::HashMap;
use std::error::Error;
use serde::Deserialize;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

#[derive(Debug)]
pub struct TdCommandMap {
    map: HashMap<Commands, ResponseKeys>,
}
#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum Commands {
    GetChatHistory,
    SearchPublicChat,
    DownloadFile,
    GetChat,
    GetChats,
    Unknown,
}

#[derive(Eq, Hash, PartialEq, Debug, Deserialize)]
pub enum ResponseKeys {
    Messages,
    Chat,
    UpdateFile,
    ChatIds,
}
impl ResponseKeys {
    pub fn to_str(&self) -> String {
        match self {
            ResponseKeys::Messages => String::from("messages"),
            ResponseKeys::Chat => String::from("chat"),
            ResponseKeys::UpdateFile => String::from("updateFile"),
            ResponseKeys::ChatIds => String::from("chatIds")
        }
    }
    pub fn from_str(data: &str) -> Ok(Self) {
        match data {
            "chatIds" => Ok(ResponseKeys::ChatIds),
            "messages" => Ok(ResponseKeys::Messages),
            "chat" => Ok(ResponseKeys::Chat),
            "updateFile" => Ok(ResponseKeys::UpdateFile),
            _ => panic!("From sql error")
        }
    }
}
impl FromSql<'_> for ResponseKeys {
    fn from_sql<'a>(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let string_from_db = String::from_utf8(raw.to_vec()).expect("Invalid UTF-8");
        match string_from_db.as_str() {
            "chatIds" => Ok(ResponseKeys::ChatIds),
            "messages" => Ok(ResponseKeys::Messages),
            "chat" => Ok(ResponseKeys::Chat),
            "updateFile" => Ok(ResponseKeys::UpdateFile),
            _ => panic!("From sql error")
        }

    }

    fn accepts(ty: &Type) -> bool {
        true
    }
}

impl TdCommandMap {
    pub fn map(&self) -> &HashMap<Commands, ResponseKeys> {
        &self.map
    }

    pub fn new() -> Self {
        let map = HashMap::from(
            [(Commands::GetChats, ResponseKeys::ChatIds),
                (Commands::GetChatHistory, ResponseKeys::Messages),
                (Commands::GetChat, ResponseKeys::Chat),
                (Commands::SearchPublicChat, ResponseKeys::Chat)]
        );
        Self {
            map
        }
    }
}