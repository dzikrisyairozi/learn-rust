//! Advanced Rust features demonstration library
//! Showcases traits, generics, async programming, and concurrency

pub mod processors;
pub mod async_utils;
pub mod concurrency;

// Re-export commonly used items
pub use processors::{DataProcessor, NumberProcessor};
pub use concurrency::ThreadSafeCounter;
pub use async_utils::process_data_async;