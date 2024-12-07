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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_thread_safe_counter() {
        let counter = ThreadSafeCounter::new();
        let mut handles = vec![];

        // Spawn 5 threads that each increment the counter
        for _ in 0..5 {
            let counter_clone = counter.get_count_arc();
            let handle = thread::spawn(move || {
                let mut count = counter_clone.lock().unwrap();
                *count += 1;
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.get_count(), 5);
    }
}