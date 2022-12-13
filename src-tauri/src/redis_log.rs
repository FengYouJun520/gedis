use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RedisLog {
    pub time: DateTime<Local>,
    pub args: Vec<String>,
}

impl RedisLog {
    pub fn new(time: DateTime<Local>, args: Vec<String>) -> Self {
        Self { time, args }
    }
}
