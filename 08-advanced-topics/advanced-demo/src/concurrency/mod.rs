//! Module containing concurrent processing utilities

mod counter;

// This line is crucial - it re-exports ThreadSafeCounter from the counter module
pub use counter::ThreadSafeCounter;