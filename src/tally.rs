#[derive(Clone)]
#[derive (Debug)]
pub struct UserTally {
    pub name: String,
    pub records: Vec<i32>
}

impl UserTally {
    // Constructor for UserTally
    pub fn new(username: &str) -> UserTally {
        UserTally {
            name: username.to_string(),
            records: Vec::new()
        }
    }
}