use async_std::task;
use serde::Serialize;
use std::time::Duration;

#[derive(Serialize)]
pub struct Dht11Data {
    pub temperature: f32,
    pub humidity: f32,
}

pub async fn read_sensor_data() -> Dht11Data {
    // Mock sensor reading, replace with actual sensor reading logic
    task::sleep(Duration::from_secs(2)).await;
    Dht11Data {
        temperature: 22.5,
        humidity: 55.0,
    }
}
