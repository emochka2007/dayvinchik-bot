use std::collections::HashMap;
use crate::td::td_request::RequestKeys;
use crate::td::td_response::ResponseKeys;

#[derive(Debug)]
pub struct TdCommandMap {
    map: HashMap<RequestKeys, ResponseKeys>,
}


impl TdCommandMap {
    pub fn map(&self) -> &HashMap<RequestKeys, ResponseKeys> {
        &self.map
    }

    pub fn new() -> Self {
        let map = HashMap::from(
            [(RequestKeys::GetChats, ResponseKeys::Chats),
                (RequestKeys::GetChatHistory, ResponseKeys::Messages),
                (RequestKeys::GetChat, ResponseKeys::Chat),
                (RequestKeys::SearchPublicChat, ResponseKeys::Chat)]
        );
        Self {
            map
        }
    }
}