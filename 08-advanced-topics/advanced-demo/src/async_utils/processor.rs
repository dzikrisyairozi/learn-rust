use tokio::time::{sleep, Duration};
use crate::processors::DataProcessor;

/// Asynchronously process data using the provided processor
pub async fn process_data_async<T, P: DataProcessor<T>>(processor: &P, data: T) -> T {
    println!("Processing with: {}", processor.describe());
    // Simulate async work
    sleep(Duration::from_millis(100)).await;
    processor.process(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::processors::NumberProcessor;

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