//! # env_monitor
//!
//! A Rust library for interacting with temperature, humidity, and fire sensors
//! on Raspberry Pi and similar devices.
//!
//! ## Features
//!
//! - DHT11 temperature and humidity sensor interface
//! - Fire detection sensor with buzzer control
//! - Async support with Tokio
//! - Trait-based design for extensibility
//!
//! ## Example
//!
//! ```rust,no_run
//! use std::error::Error;
//! use env_monitor::sensors::{TemperatureSensor, FireDetector};
//! use env_monitor::sensors::dht11::Dht11Sensor;
//! use env_monitor::sensors::fire::FireSensor;
//! use tokio::time::{self, Duration};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
//!     // Initialize sensors
//!     let temp_sensor = Dht11Sensor::new(17);
//!     let fire_sensor = FireSensor::new(27, 22, true);
//!     
//!     // Start fire monitoring
//!     fire_sensor.start_monitoring(100).await?;
//!     
//!     // Read temperature and humidity
//!     match temp_sensor.read_async().await {
//!         Ok(data) => println!("Temperature: {}°C, Humidity: {}%", data.temperature, data.humidity),
//!         Err(e) => println!("Sensor read error: {}", e),
//!     }
//!     
//!     Ok(())
//! }
//! ```

// Re-export modules
pub mod error;
pub mod sensors;

// Re-export main types for convenience
pub use sensors::dht11::Dht11Data;
pub use sensors::fire::FireSensorData;
