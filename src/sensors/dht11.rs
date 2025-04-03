//! DHT11 temperature and humidity sensor implementation

use async_trait::async_trait;
use rppal::gpio::{Gpio, Level, Mode};
use std::time::{Duration, Instant};
use tokio::task;

use crate::error::SensorError;
use crate::sensors::traits::TemperatureSensor;

/// DHT11 sensor data structure containing temperature and humidity readings
#[derive(Debug, Clone, Copy)]
pub struct Dht11Data {
    /// Temperature in degrees Celsius
    pub temperature: f32,
    /// Relative humidity percentage
    pub humidity: f32,
}

/// DHT11 temperature and humidity sensor implementation
pub struct Dht11Sensor {
    /// GPIO pin number connected to the DHT11 sensor
    gpio_pin: u8,
}

impl Dht11Sensor {
    /// Create a new DHT11 sensor instance
    ///
    /// # Arguments
    /// * `pin` - GPIO pin number connected to the DHT11 sensor
    ///
    /// # Example
    /// ```
    /// use env_monitor::sensors::dht11::Dht11Sensor;
    ///
    /// let sensor = Dht11Sensor::new(17);
    /// ```
    pub fn new(pin: u8) -> Self {
        Dht11Sensor { gpio_pin: pin }
    }

    // Helper function for reading sensor data
    fn read_internal(&self) -> Result<Dht11Data, SensorError> {
        let gpio = Gpio::new()?;
        let mut pin = gpio.get(self.gpio_pin)?.into_io(Mode::Output);

        // Send start signal
        pin.write(Level::Low);
        std::thread::sleep(Duration::from_millis(20)); // At least 18ms low level
        pin.write(Level::High);

        // Switch to input mode to receive data
        pin.set_mode(Mode::Input);

        // Wait for DHT11 response
        let timeout = Instant::now() + Duration::from_millis(100);
        while pin.read() == Level::High {
            if Instant::now() > timeout {
                return Err(SensorError::Timeout(
                    "Waiting for DHT11 response timed out".into(),
                ));
            }
        }

        while pin.read() == Level::Low {
            if Instant::now() > timeout {
                return Err(SensorError::Timeout(
                    "DHT11 response signal timed out".into(),
                ));
            }
        }

        while pin.read() == Level::High {
            if Instant::now() > timeout {
                return Err(SensorError::Timeout("DHT11 ready signal timed out".into()));
            }
        }

        // Read 40 bits of data (8bit humidity integer + 8bit humidity decimal + 8bit temperature integer + 8bit temperature decimal + 8bit checksum)
        let mut data = [0u8; 5];

        for byte in data.iter_mut() {
            for j in 0..8 {
                // Wait for 50us low level to pass
                while pin.read() == Level::Low {
                    if Instant::now() > timeout {
                        return Err(SensorError::Timeout(
                            "Timed out while reading data bit".into(),
                        ));
                    }
                }

                // Measure high level duration to determine data bit (0 or 1)
                let start = Instant::now();
                while pin.read() == Level::High {
                    if Instant::now() > timeout {
                        return Err(SensorError::Timeout(
                            "Timed out during high level data bit reading".into(),
                        ));
                    }
                }
                let duration = start.elapsed();

                // If high level lasts about 70 microseconds, it's a data bit "1"
                if duration > Duration::from_micros(40) {
                    *byte |= 1 << (7 - j);
                }
            }
        }

        // Verify checksum
        if data[4] != (data[0] + data[1] + data[2] + data[3]) {
            return Err(SensorError::DataValidation("Checksum error".into()));
        }

        // Process DHT11 temperature and humidity data (not using decimal parts due to low precision of DHT11)
        let humidity = data[0] as f32;
        let temperature = data[2] as f32;

        Ok(Dht11Data {
            temperature,
            humidity,
        })
    }
}

#[async_trait]
impl TemperatureSensor for Dht11Sensor {
    /// Synchronously read temperature and humidity data
    ///
    /// # Returns
    /// Temperature and humidity data or error
    ///
    /// # Example
    /// ```no_run
    /// use env_monitor::sensors::TemperatureSensor;
    /// use env_monitor::sensors::dht11::Dht11Sensor;
    ///
    /// let sensor = Dht11Sensor::new(17);
    /// match sensor.read() {
    ///     Ok(data) => println!("Temperature: {}°C, Humidity: {}%", data.temperature, data.humidity),
    ///     Err(e) => println!("Read failed: {}", e),
    /// }
    /// ```
    fn read(&self) -> Result<Dht11Data, SensorError> {
        self.read_internal()
    }

    /// Asynchronously read temperature and humidity data
    ///
    /// # Returns
    /// Temperature and humidity data or error
    ///
    /// # Example
    /// ```no_run
    /// use env_monitor::sensors::TemperatureSensor;
    /// use env_monitor::sensors::dht11::Dht11Sensor;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let sensor = Dht11Sensor::new(17);
    ///     match sensor.read_async().await {
    ///         Ok(data) => println!("Temperature: {}°C, Humidity: {}%", data.temperature, data.humidity),
    ///         Err(e) => println!("Read failed: {}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    async fn read_async(&self) -> Result<Dht11Data, SensorError> {
        let pin = self.gpio_pin;

        // Execute the read operation in a blocking task
        task::spawn_blocking(move || {
            let sensor = Dht11Sensor::new(pin);
            sensor.read()
        })
        .await
        .map_err(|e| SensorError::SensorError(format!("Task join error: {}", e)))?
    }
}
