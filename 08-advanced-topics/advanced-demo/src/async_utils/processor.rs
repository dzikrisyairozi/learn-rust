use tokio::time::{sleep, Duration};
use crate::processors::DataProcessor;

/// Asynchronously process data using the provided processor
pub async fn process_data_async<T, P: DataProcessor<T>>(processor: &P, data: T) -> T {
    println!("Processing with: {}", processor.describe());
    // Simulate async work
    sleep(Duration::from_millis(100)).await;
    processor.process(data)
}

