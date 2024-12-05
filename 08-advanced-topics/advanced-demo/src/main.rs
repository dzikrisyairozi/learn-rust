use std::thread;
use advanced_demo::{
    NumberProcessor,
    ThreadSafeCounter,
    process_data_async,
};

#[tokio::main]
async fn main() {
    // SECTION 1: Traits and Generics Demonstration
    let int_processor = NumberProcessor {
        multiplier: 2,
        name: String::from("Integer"),
    };
    let float_processor = NumberProcessor {
        multiplier: 1.5,
        name: String::from("Float"),
    };

    // SECTION 2: Async Processing Demonstration
    let int_result = process_data_async(&int_processor, 10).await;
    let float_result = process_data_async(&float_processor, 10.0).await;
    println!("Results: {} and {}", int_result, float_result);

    // SECTION 3: Functional Programming Demonstration
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter()
        .map(|x| x * 2)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("Processed sum: {}", sum);

    // SECTION 4: Concurrent Processing Demonstration
    let counter = ThreadSafeCounter::new();
    let mut handles = vec![];

    // Spawn multiple threads to demonstrate thread safety
    for i in 0..5 {
        let counter_clone = counter.get_count_arc();
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