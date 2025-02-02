use serde::{Deserialize, Serialize};
use crate::errors::GeneralError;
use crate::file::read_json_file;

#[derive(Deserialize, Serialize, Debug)]
pub struct SuperLike {
    pub nyash: String,
    cute: String,
    photo_action_like: String,
}

impl SuperLike {
    pub fn get_from_file() -> Result<SuperLike, GeneralError> {
        let path = "superlikes.json";
        let json_content = read_json_file(path).unwrap();
        let superlikes = serde_json::from_str::<SuperLike>(&json_content)?;
        println!("{:?}", json_content);
        Ok(superlikes)
    }
}