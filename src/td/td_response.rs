use serde::Deserialize;
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use tokio_postgres::types::{FromSql, Type};

#[derive(Eq, Hash, PartialEq, Debug, Deserialize)]
pub enum ResponseKeys {
    Messages,
    Chat,
    UpdateFile,
    Chats,
    Ok,
    UpdateChatReadInbox,
    //todo unknown
}
impl ResponseKeys {
    pub fn to_str(&self) -> String {
        match self {
            ResponseKeys::Messages => String::from("messages"),
            ResponseKeys::Chat => String::from("chat"),
            ResponseKeys::UpdateFile => String::from("updateFile"),
            ResponseKeys::Chats => String::from("chats"),
            ResponseKeys::UpdateChatReadInbox => String::from("updateChatReadInbox"),
            ResponseKeys::Ok => String::from("ok")
        }
    }
    pub fn from_str(data: &str) -> io::Result<Self> {
        match data {
            "chats" => Ok(ResponseKeys::Chats),
            "messages" => Ok(ResponseKeys::Messages),
            "chat" => Ok(ResponseKeys::Chat),
            "updateFile" => Ok(ResponseKeys::UpdateFile),
            "updateChatReadInbox" => Ok(ResponseKeys::UpdateChatReadInbox),
            "ok" => Ok(ResponseKeys::Ok),
            _ => Err(io::Error::new(
                ErrorKind::InvalidInput,
                "Unknown response key",
            )),
        }
    }
}
impl FromSql<'_> for ResponseKeys {
    fn from_sql<'a>(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let string_from_db = String::from_utf8(raw.to_vec()).expect("Invalid UTF-8");
        match string_from_db.as_str() {
            "chats" => Ok(ResponseKeys::Chats),
            "messages" => Ok(ResponseKeys::Messages),
            "chat" => Ok(ResponseKeys::Chat),
            "updateFile" => Ok(ResponseKeys::UpdateFile),
            "updateChatReadInbox" => Ok(ResponseKeys::UpdateChatReadInbox),
            "ok" => Ok(ResponseKeys::Ok),
            _ => Err(io::Error::new(ErrorKind::InvalidInput, "Unknown response key").into()),
        }
    }

    //todo fix type
    fn accepts(ty: &Type) -> bool {
        true
    }
}
