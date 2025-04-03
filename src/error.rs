//! Custom error types for the Sensor library

use rppal::gpio;
use std::{error::Error, fmt, io};

/// Sensor library error types
#[derive(Debug)]
pub enum SensorError {
    /// General IO errors
    IoError(io::Error),
    /// GPIO-specific errors
    GpioError(gpio::Error),
    /// Timeout errors when communicating with sensors
    Timeout(String),
    /// Data validation errors (e.g. checksum failures)
    DataValidation(String),
    /// Initialization errors
    InitError(String),
    /// General sensor errors
    SensorError(String),
}

impl fmt::Display for SensorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SensorError::IoError(err) => write!(f, "IO error: {}", err),
            SensorError::GpioError(err) => write!(f, "GPIO error: {}", err),
            SensorError::Timeout(msg) => write!(f, "Timeout error: {}", msg),
            SensorError::DataValidation(msg) => write!(f, "Data validation error: {}", msg),
            SensorError::InitError(msg) => write!(f, "Initialization error: {}", msg),
            SensorError::SensorError(msg) => write!(f, "Sensor error: {}", msg),
        }
    }
}

impl Error for SensorError {}

impl From<io::Error> for SensorError {
    fn from(err: io::Error) -> Self {
        SensorError::IoError(err)
    }
}

impl From<gpio::Error> for SensorError {
    fn from(err: gpio::Error) -> Self {
        SensorError::GpioError(err)
    }
}

impl From<String> for SensorError {
    fn from(msg: String) -> Self {
        SensorError::SensorError(msg)
    }
}

impl From<&str> for SensorError {
    fn from(msg: &str) -> Self {
        SensorError::SensorError(msg.to_string())
    }
}
