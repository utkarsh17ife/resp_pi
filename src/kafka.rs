// src/kafka.rs

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use serde_json::json;
use crate::sensors::SensorData;

pub async fn send_to_kafka(producer: &FutureProducer, topic: &str, data: SensorData) {
    let payload = json!(data).to_string();
    let record = FutureRecord::to(topic)
        .payload(&payload)
        .key("sensor_data");

    producer.send(record, Timeout::Never).await.unwrap();
}

pub fn create_producer(brokers: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create()
        .expect("Producer creation error")
}
