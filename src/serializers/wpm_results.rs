use chrono::{serde::ts_seconds, DateTime, Local};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct WpmResults {
    pub wpm: f64,
    pub accuracy: f64,
    pub duration: f64,
    pub awpm: f64,
    pub date_time: DateTime<Local>,
}

impl WpmResults {
    pub fn new(wpm: f64, accuracy: f64, duration: f64, awpm: f64) -> Self {
        Self {
            wpm,
            accuracy,
            duration,
            awpm,
            date_time: Local::now(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
