use advanced_demo::{NumberProcessor, DataProcessor, process_data_async, ThreadSafeCounter};
use std::thread;

#[test]
fn test_complete_workflow() {
    // Test number processing
    let processor = NumberProcessor {
        multiplier: 2,
        name: String::from("Test"),
    };
    assert_eq!(processor.process(5), 10);

    // Test thread-safe counter
    let counter = ThreadSafeCounter::new();
    let mut handles = vec![];

    for _ in 0..3 {
        let counter_clone = counter.get_count_arc();
        let handle = thread::spawn(move || {
            let mut count = counter_clone.lock().unwrap();
            *count += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(counter.get_count(), 3);
}

#[tokio::test]
async fn test_async_workflow() {
    let processor = NumberProcessor {
        multiplier: 3,
        name: String::from("Test"),
    };
    let result = process_data_async(&processor, 5).await;
    assert_eq!(result, 15);
}