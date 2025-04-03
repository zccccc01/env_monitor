//! Sensor trait definitions

use crate::error::SensorError;
use crate::sensors::{dht11::Dht11Data, fire::FireSensorData};
use async_trait::async_trait;

/// Temperature and humidity sensor trait
#[async_trait]
pub trait TemperatureSensor: Send + Sync {
    /// Synchronously read temperature and humidity data
    fn read(&self) -> Result<Dht11Data, SensorError>;

    /// Asynchronously read temperature and humidity data
    async fn read_async(&self) -> Result<Dht11Data, SensorError>;
}

/// Fire detection sensor trait
#[async_trait]
pub trait FireDetector: Send + Sync {
    /// Synchronously read fire detector status
    fn read(&self) -> Result<FireSensorData, SensorError>;

    /// Asynchronously read fire detector status
    async fn read_async(&self) -> Result<FireSensorData, SensorError>;

    /// Start monitoring for fire with the given check interval
    async fn start_monitoring(&self, check_interval_ms: u64) -> Result<(), SensorError>;

    /// Stop monitoring for fire
    fn stop_monitoring(&self);
}
