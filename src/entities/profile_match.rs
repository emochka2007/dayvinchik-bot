use std::sync::{Arc, Mutex};

pub type ProfileMatches = Arc<Mutex<Vec<ProfileMatch>>>;

#[derive(Debug)]
pub struct ProfileMatch {
    pub(crate) url: String,
    pub(crate) full_text: String,
    //todo image
}
//todo impl