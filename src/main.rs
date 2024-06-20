// src/main.rs

mod sensors;
mod kafka;
mod config;

use kafka::{create_producer, send_to_kafka};
use sensors::read_all_sensors;

#[async_std::main]
async fn main() {
    let config = config::load_config();

    let producer = create_producer(&config.kafka_brokers);

    loop {
        let sensor_data_list = read_all_sensors().await;
        for sensor_data in sensor_data_list {
            send_to_kafka(&producer, &config.kafka_topic, sensor_data).await;
        }
    }
}
