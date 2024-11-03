use crate::models::user::User;

pub struct Auth;

impl Auth {
    pub fn check_librarian(user: &User) -> bool {
        user.is_librarian
    }
}