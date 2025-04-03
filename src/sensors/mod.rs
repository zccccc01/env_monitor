//! Sensor implementations and traits

pub mod dht11;
pub mod fire;
pub mod traits;

// Re-export traits
pub use traits::{FireDetector, TemperatureSensor};
