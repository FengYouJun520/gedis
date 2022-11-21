use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RedisLog {
    pub time: DateTime<Local>,
    pub name: String,
    pub command: String,
}

impl RedisLog {
    pub fn new(time: DateTime<Local>, name: String, command: String) -> Self {
        Self {
            time,
            name,
            command,
        }
    }
}
