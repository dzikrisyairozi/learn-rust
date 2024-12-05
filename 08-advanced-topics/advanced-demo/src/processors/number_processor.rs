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

