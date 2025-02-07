#[derive(Debug)]
pub struct MyProfile {
    name: String,
    age: i16
}

impl MyProfile {
    pub fn new() -> Self {
        Self {
            name: "Nikita".to_string(),
            age: 23
        }
    }
}