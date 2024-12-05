//! Module containing trait definitions and implementations for data processing

mod number_processor;

pub use number_processor::NumberProcessor;

/// Generic trait for data processing operations
pub trait DataProcessor<T> {
    /// Process the input data and return transformed data
    fn process(&self, data: T) -> T;
    
    /// Provide a description of the processor
    fn describe(&self) -> String;
}