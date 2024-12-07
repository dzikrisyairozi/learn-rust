use std::fmt::Display;
use super::DataProcessor;

/// Generic structure for handling numeric type processing
#[derive(Debug)]
pub struct NumberProcessor<T> {
    pub multiplier: T,
    pub name: String,
}

impl<T: Display + Copy + std::ops::Mul<Output = T>> DataProcessor<T> for NumberProcessor<T> {
    fn process(&self, data: T) -> T {
        data * self.multiplier
    }

    fn describe(&self) -> String {
        format!("{} Processor (multiplier: {})", self.name, self.multiplier)
    }
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
        assert_eq!(processor.describe(), "Test Processor (multiplier: 2)");
    }
}