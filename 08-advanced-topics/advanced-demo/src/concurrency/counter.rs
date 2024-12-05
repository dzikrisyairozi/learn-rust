use std::sync::{Arc, Mutex};

/// Thread-safe counter implementation using Arc and Mutex
pub struct ThreadSafeCounter {
    count: Arc<Mutex<i32>>,
}

impl ThreadSafeCounter {
    pub fn new() -> Self {
        ThreadSafeCounter {
            count: Arc::new(Mutex::new(0)),
        }
    }

    pub fn increment(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }

    pub fn get_count(&self) -> i32 {
        *self.count.lock().unwrap()
    }

    pub fn get_count_arc(&self) -> Arc<Mutex<i32>> {
        Arc::clone(&self.count)
    }
}

