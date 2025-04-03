//! Fire detection sensor implementation

use async_trait::async_trait;
use rppal::gpio::{Gpio, Level};
use std::sync::{Arc, Mutex};
use tokio::task;
use tokio::time::{Duration, sleep};

use crate::error::SensorError;
use crate::sensors::traits::FireDetector;

/// Fire sensor data structure containing detection status and timestamp
#[derive(Debug, Clone, Copy)]
pub struct FireSensorData {
    /// Whether flame is detected
    pub flame_detected: bool,
    /// Timestamp of the last detection (if detected)
    pub last_detection_timestamp: Option<u64>,
}

/// Fire sensor implementation with buzzer support
pub struct FireSensor {
    /// GPIO pin number connected to the flame sensor
    flame_pin: u8,
    /// GPIO pin number connected to the buzzer
    buzzer_pin: u8,
    /// Sensor active state
    is_active: Arc<Mutex<bool>>,
    /// Sensor logic configuration (true = high level active, false = low level active)
    high_active: bool,
}

impl FireSensor {
    /// Create a new fire sensor instance
    ///
    /// # Arguments
    /// * `flame_pin` - GPIO pin number connected to the flame sensor
    /// * `buzzer_pin` - GPIO pin number connected to the buzzer
    /// * `high_active` - Sensor logic (true if high level indicates flame detection, false if low level indicates flame detection)
    ///
    /// # Example
    /// ```
    /// use env_monitor::sensors::fire::FireSensor;
    ///
    /// // If sensor outputs high level when flame is detected
    /// let sensor = FireSensor::new(27, 17, true);
    /// // If sensor outputs low level when flame is detected
    /// let sensor = FireSensor::new(27, 17, false);
    /// ```
    pub fn new(flame_pin: u8, buzzer_pin: u8, high_active: bool) -> Self {
        FireSensor {
            flame_pin,
            buzzer_pin,
            is_active: Arc::new(Mutex::new(true)),
            high_active,
        }
    }

    // Helper function for reading sensor status
    fn read_internal(&self) -> Result<FireSensorData, SensorError> {
        let gpio = Gpio::new()?;
        let flame_sensor = gpio.get(self.flame_pin)?.into_input();

        // Determine flame detection based on configuration
        let flame_detected = if self.high_active {
            flame_sensor.read() == Level::High
        } else {
            flame_sensor.read() == Level::Low
        };

        let timestamp = if flame_detected {
            Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|e| SensorError::SensorError(format!("Time error: {}", e)))?
                    .as_secs(),
            )
        } else {
            None
        };

        Ok(FireSensorData {
            flame_detected,
            last_detection_timestamp: timestamp,
        })
    }
}

#[async_trait]
impl FireDetector for FireSensor {
    /// Synchronously read fire sensor status
    ///
    /// # Returns
    /// Fire sensor data or error
    ///
    /// # Example
    /// ```no_run
    /// use env_monitor::sensors::FireDetector;
    /// use env_monitor::sensors::fire::FireSensor;
    ///
    /// let sensor = FireSensor::new(27, 17, true);
    /// match sensor.read() {
    ///     Ok(data) => println!("Flame detected: {}", data.flame_detected),
    ///     Err(e) => println!("Read failed: {}", e),
    /// }
    /// ```
    fn read(&self) -> Result<FireSensorData, SensorError> {
        self.read_internal()
    }

    /// Asynchronously read fire sensor status
    ///
    /// # Returns
    /// Fire sensor data or error
    ///
    /// # Example
    /// ```no_run
    /// use env_monitor::sensors::FireDetector;
    /// use env_monitor::sensors::fire::FireSensor;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let sensor = FireSensor::new(27, 17, true);
    ///     match sensor.read_async().await {
    ///         Ok(data) => println!("Flame detected: {}", data.flame_detected),
    ///         Err(e) => println!("Read failed: {}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    async fn read_async(&self) -> Result<FireSensorData, SensorError> {
        let flame_pin = self.flame_pin;
        let high_active = self.high_active;

        // Execute the read operation in a blocking task
        task::spawn_blocking(move || {
            let gpio = Gpio::new()?;
            let flame_sensor = gpio.get(flame_pin)?.into_input();

            // Determine flame detection based on configuration
            let flame_detected = if high_active {
                flame_sensor.read() == Level::High
            } else {
                flame_sensor.read() == Level::Low
            };

            let timestamp = if flame_detected {
                Some(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map_err(|e| SensorError::SensorError(format!("Time error: {}", e)))?
                        .as_secs(),
                )
            } else {
                None
            };

            Ok::<FireSensorData, SensorError>(FireSensorData {
                flame_detected,
                last_detection_timestamp: timestamp,
            })
        })
        .await
        .map_err(|e| SensorError::SensorError(format!("Task join error: {}", e)))?
    }

    /// Start monitoring for fire with the given check interval
    ///
    /// # Arguments
    /// * `check_interval_ms` - Interval in milliseconds between checks
    ///
    /// # Returns
    /// Ok if monitoring started successfully, Error otherwise
    ///
    /// # Example
    /// ```no_run
    /// use env_monitor::sensors::FireDetector;
    /// use env_monitor::sensors::fire::FireSensor;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let sensor = FireSensor::new(27, 17, true);
    ///     sensor.start_monitoring(100).await?;
    ///     
    ///     // Do other things while monitoring runs in background
    ///     
    ///     // Stop monitoring
    ///     sensor.stop_monitoring();
    ///     Ok(())
    /// }
    /// ```
    async fn start_monitoring(&self, check_interval_ms: u64) -> Result<(), SensorError> {
        println!("Starting fire monitoring (async version)");
        println!(
            "Sensor configuration: {} level active",
            if self.high_active { "high" } else { "low" }
        );

        // Initialize GPIO
        let gpio = Gpio::new()?;
        let flame_pin_clone = self.flame_pin;
        let buzzer_pin_clone = self.buzzer_pin;
        let is_active_clone = self.is_active.clone();
        let high_active = self.high_active;

        // Run monitoring in a separate task
        tokio::spawn(async move {
            // Initialize GPIO pins
            let flame_sensor = match gpio.get(flame_pin_clone) {
                Ok(pin) => pin.into_input(),
                Err(e) => {
                    eprintln!("Failed to initialize flame sensor: {}", e);
                    return;
                }
            };

            let mut buzzer = match gpio.get(buzzer_pin_clone) {
                Ok(pin) => pin.into_output(),
                Err(e) => {
                    eprintln!("Failed to initialize buzzer: {}", e);
                    return;
                }
            };

            // Initial state: turn off buzzer
            buzzer.set_high();

            // Monitoring loop
            loop {
                // Check if monitoring should continue
                {
                    let is_active = is_active_clone.lock().unwrap();
                    if !*is_active {
                        buzzer.set_high(); // Ensure buzzer is off
                        break;
                    }
                }

                // Detect flame based on configuration
                let flame_detected = if high_active {
                    flame_sensor.read() == Level::High
                } else {
                    flame_sensor.read() == Level::Low
                };

                // Flame detection
                if flame_detected {
                    println!("WARNING: Flame detected!");

                    // Sound the alarm
                    const ALARM_FREQ: u32 = 1000; // 1kHz
                    const ALARM_DURATION: u64 = 200; // Duration of each tone (ms)

                    let half_period = 1_000_000 / ALARM_FREQ / 2;
                    let cycles = ALARM_DURATION * 1000 / (half_period as u64 * 2);

                    for _ in 0..cycles {
                        buzzer.set_low();
                        std::thread::sleep(std::time::Duration::from_micros(half_period as u64));
                        buzzer.set_high();
                        std::thread::sleep(std::time::Duration::from_micros(half_period as u64));
                    }
                } else {
                    // No flame - ensure buzzer is off
                    buzzer.set_high();
                }

                // Wait for next check
                sleep(Duration::from_millis(check_interval_ms)).await;
            }
        });

        Ok(())
    }

    /// Stop monitoring for fire
    ///
    /// # Example
    /// ```no_run
    /// use env_monitor::sensors::FireDetector;
    /// use env_monitor::sensors::fire::FireSensor;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let sensor = FireSensor::new(27, 17, true);
    ///     sensor.start_monitoring(100).await?;
    ///     
    ///     // Do other things...
    ///     
    ///     // Stop monitoring
    ///     sensor.stop_monitoring();
    ///     Ok(())
    /// }
    /// ```
    fn stop_monitoring(&self) {
        let mut is_active = self.is_active.lock().unwrap();
        *is_active = false;
    }
}
