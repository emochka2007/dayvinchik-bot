/**
- Dayvinchik interaction
- Actions:
*/
#[derive(Debug)]
pub struct Profile {
    name: String,
    age: i16,
}

impl Profile {
    pub fn _new() -> Self {
        Self {
            name: "Nikita".to_string(),
            age: 23,
        }
    }
}
