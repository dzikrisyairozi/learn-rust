use std::fmt::Display;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::time::{sleep, Duration};

// Trait definition for data processing
trait DataProcessor<T> {
    fn process(&self, data: T) -> T;
    fn describe(&self) -> String;
}

// Generic structure for handling any numeric type
#[derive(Debug)]
struct NumberProcessor<T> {
    multiplier: T,
    name: String,
}

// Implementation of DataProcessor for any type that implements necessary traits
impl<T: Display + Copy + std::ops::Mul<Output = T>> DataProcessor<T> for NumberProcessor<T> {
    fn process(&self, data: T) -> T {
        data * self.multiplier
    }

    fn describe(&self) -> String {
        format!("{} Processor (multiplier: {})", self.name, self.multiplier)
    }
}

// Async function demonstration
async fn process_data_async<T, P: DataProcessor<T>>(processor: &P, data: T) -> T {
    println!("Processing with: {}", processor.describe());
    sleep(Duration::from_millis(100)).await;
    processor.process(data)
}

// Thread-safe counter using Arc and Mutex
struct ThreadSafeCounter {
    count: Arc<Mutex<i32>>,
}

impl ThreadSafeCounter {
    fn new() -> Self {
        ThreadSafeCounter {
            count: Arc::new(Mutex::new(0)),
        }
    }

    fn increment(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }

    fn get_count(&self) -> i32 {
        *self.count.lock().unwrap()
    }
}

#[tokio::main]
async fn main() {
    // Demonstrate traits and generics
    let int_processor = NumberProcessor {
        multiplier: 2,
        name: String::from("Integer"),
    };
    let float_processor = NumberProcessor {
        multiplier: 1.5,
        name: String::from("Float"),
    };

    // Process different numeric types
    let int_result = process_data_async(&int_processor, 10).await;
    let float_result = process_data_async(&float_processor, 10.0).await;
    println!("Results: {} and {}", int_result, float_result);

    // Demonstrate iterators and closures
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter()
        .map(|x| x * 2)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("Processed sum: {}", sum);

    // Demonstrate thread-safe concurrent processing
    let counter = ThreadSafeCounter::new();
    let mut handles = vec![];

    // Spawn multiple threads
    for i in 0..5 {
        let counter_clone = Arc::clone(&counter.count);
        let handle = thread::spawn(move || {
            let mut count = counter_clone.lock().unwrap();
            *count += 1;
            println!("Thread {} incremented counter", i);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", counter.get_count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_processor() {
        let processor = NumberProcessor {
            multiplier: 2,
            name: String::from("Test"),
        };
        assert_eq!(processor.process(5), 10);
    }

    #[tokio::test]
    async fn test_async_processing() {
        let processor = NumberProcessor {
            multiplier: 2,
            name: String::from("Test"),
        };
        let result = process_data_async(&processor, 5).await;
        assert_eq!(result, 10);
    }
}