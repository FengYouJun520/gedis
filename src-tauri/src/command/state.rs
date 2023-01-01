use crate::{config::RedisConfig, error::Result};
use anyhow::Context;
use async_trait::async_trait;
use redis::{Cmd, Pipeline};
use std::{collections::HashMap, fmt::Debug, sync::Arc};
use tauri::async_runtime::Mutex;
#[derive(Default)]
pub struct Redis {
    configs: HashMap<String, RedisConfig>,
    connections: HashMap<String, redis::aio::Connection>,
}

impl Debug for Redis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Redis")
            .field("configs", &self.configs)
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct RedisState(pub Mutex<Redis>);

impl Redis {
    pub fn add_con(&mut self, con: redis::aio::Connection, config: RedisConfig) -> Result<()> {
        self.connections.insert(config.id.to_string(), con);
        self.configs.insert(config.id.to_string(), config);

        Ok(())
    }

    pub fn remove_con(&mut self, id: &str) -> Result<()> {
        self.connections.remove(id);
        self.configs.remove(id);
        Ok(())
    }

    pub fn remove_con_all(&mut self) -> Result<()> {
        self.connections.clear();
        self.configs.clear();
        Ok(())
    }

    pub fn is_connection(&self, id: &str) -> bool {
        self.connections.contains_key(id)
    }

    pub async fn get_con_mut(&mut self, id: &str) -> Result<&mut redis::aio::Connection> {
        let con = self.connections.get_mut(id).context("客户端未连接")?;
        Ok(con)
    }

    pub async fn get_con_and_config(
        &mut self,
        id: &str,
    ) -> Result<(&mut redis::aio::Connection, &RedisConfig)> {
        let con = self.connections.get_mut(id).context("客户端未连接")?;
        let config = self.configs.get_mut(id).context("客户端未连接")?;
        Ok((con, config))
    }
}

const MAX_HISTORY: usize = 5000;

#[derive(Debug, Default)]
pub struct History(pub Arc<std::sync::Mutex<Vec<String>>>);

impl History {
    pub fn add_log(&self, value: String, config: &RedisConfig) {
        let mut histories = self.0.lock().unwrap();
        if histories.len() == MAX_HISTORY {
            histories.reverse();
            for _ in 0..=2500 {
                histories.pop();
            }
            histories.reverse();
        }
        histories.push(format!("[{}] {}", config.name, value));
    }

    pub fn add_log_vec(&self, values: Vec<String>, config: &RedisConfig) {
        let log: String = values.join(" ").to_lowercase();
        self.add_log(log, config)
    }
}

impl Clone for History {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[async_trait]
pub trait CmdLog: Send {
    fn log(&self, history: Arc<std::sync::Mutex<Vec<String>>>, config: &RedisConfig) -> &Self;
}

/// 这个不起作用？
#[async_trait]
impl CmdLog for Pipeline {
    fn log(&self, history: Arc<std::sync::Mutex<Vec<String>>>, config: &RedisConfig) -> &Self {
        let iter = self.cmd_iter();
        for cmd in iter {
            _ = cmd.log(history.clone(), config);
        }

        self
    }
}

#[async_trait]
impl CmdLog for Cmd {
    fn log(&self, history: Arc<std::sync::Mutex<Vec<String>>>, config: &RedisConfig) -> &Self {
        let mut logs: Vec<String> = vec![];
        let iter = self.args_iter();
        for arg in iter {
            match arg {
                redis::Arg::Simple(val) => {
                    logs.push(String::from_utf8_lossy(val).to_lowercase());
                }
                redis::Arg::Cursor => {}
            }
        }

        let log: String = logs.join(" ");
        history
            .lock()
            .unwrap()
            .push(format!("[{}] {}", config.name, log));

        self
    }
}
