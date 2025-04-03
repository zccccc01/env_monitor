use env_monitor::sensors::dht11::Dht11Sensor;
use env_monitor::sensors::fire::FireSensor;
use env_monitor::sensors::{FireDetector, TemperatureSensor};
use std::error::Error;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("环境监控系统启动");
    println!("- DHT11温湿度传感器: GPIO17");
    println!("- 火焰传感器: GPIO27, 蜂鸣器: GPIO22");

    let dht_sensor = Dht11Sensor::new(17);
    let fire_sensor = FireSensor::new(27, 22, true);

    fire_sensor.start_monitoring(100).await?;

    loop {
        match dht_sensor.read_async().await {
            Ok(data) => {
                println!(
                    "温度: {:.1}°C, 湿度: {:.1}%",
                    data.temperature, data.humidity
                );

                if data.temperature > 40.0 {
                    println!("警告: 温度过高! ({:.1}°C)", data.temperature);
                }
            }
            Err(e) => {
                println!("DHT11读取失败: {}", e);
            }
        }

        time::sleep(Duration::from_secs(5)).await;
    }
}
