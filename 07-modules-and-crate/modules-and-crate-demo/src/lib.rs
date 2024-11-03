// Main library crate file that exposes our public API
pub mod models;
pub mod services;

// Re-export commonly used items for convenience
pub use models::{book::Book, user::User};
pub use services::{library::Library, auth::Auth};

// Library crate configuration and initialization
pub fn init() -> Library {
    println!("Initializing library system...");
    Library::new()
}