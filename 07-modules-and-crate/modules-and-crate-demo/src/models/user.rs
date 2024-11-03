#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub is_librarian: bool,
}

impl User {
    pub fn new(id: u32, username: &str) -> Self {
        User {
            id,
            username: username.to_string(),
            is_librarian: false,
        }
    }
}