use std::io::Read;
use std::path::Path;
use chrono::{serde::ts_seconds, DateTime, Local};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use serde_json::Error;
use crate::loaders::loader::read_lines;

#[derive(Serialize, Deserialize, Debug)]
pub struct WpmResult {
    pub wpm: f64,
    pub accuracy: f64,
    pub duration: f64,
    pub awpm: f64,
    pub date_time: DateTime<Local>,
}

impl WpmResult {
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

    pub fn from_json(json: &str) -> Result<WpmResult, Error> {
        serde_json::from_str(json)
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Vec<WpmResult>, Error> {
        let f = read_lines(path).unwrap();
        let mut vec: Vec<WpmResult> = Vec::new();
        let num = f.iter().map(|x| x.to_string()).collect::<Vec<String>>();

        for mut i in num {
            if !i.is_empty() {
                vec.push(serde_json::from_str(&i.as_str()).unwrap());
            }
        }

        Ok(vec)
    }
}
