// src/sensors/mod.rs

pub mod dht11;

use serde::Serialize;

#[derive(Serialize)]
pub struct SensorData {
    // return dht11 data
    dht11_data: dht11::Dht11Data,
}

// A generic function to read data from all sensors
pub async fn read_all_sensors() -> Vec<SensorData> {
    let dht11_data = dht11::read_sensor_data().await;

    vec![SensorData {
        dht11_data: dht11_data,
    }]
}
