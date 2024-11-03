// Import the library crate's functionality
use library_system::{init, Book, User};

fn main() {
    // Initialize the library system
    let mut library = init();

    // Create and add a book
    let book = Book::new(
        1,
        "The Rust Programming Language",
        "Dzikri Syairozi and Lebron James",
    );
    library.add_book(book);

    // Register a user
    let user = User::new(1, "dzikrisyairozi");
    library.register_user(user);

    // Demonstrate accessing a book
    if let Some(book) = library.get_book(1) {
        println!("Found book: {:?}", book);
    }
}