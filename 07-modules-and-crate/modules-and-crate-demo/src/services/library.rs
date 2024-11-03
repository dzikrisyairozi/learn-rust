use crate::models::{book::Book, user::User};
use std::collections::HashMap;

pub struct Library {
    books: HashMap<u32, Book>,
    users: HashMap<u32, User>,
}

impl Library {
    pub fn new() -> Self {
        Library {
            books: HashMap::new(),
            users: HashMap::new(),
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.insert(book.id, book);
    }

    pub fn get_book(&self, id: u32) -> Option<&Book> {
        self.books.get(&id)
    }

    pub fn register_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }
}