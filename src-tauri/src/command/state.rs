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
}

#[derive(Debug, Default)]
pub struct History(pub Arc<Mutex<Vec<String>>>);

impl History {
    pub async fn add_log(&self, value: String) {
        let mut histories = self.0.lock().await;
        histories.push(value);
    }

    pub async fn add_log_vec(&self, values: Vec<String>) {
        let log: String = values.join(" ").to_lowercase();
        self.add_log(log).await
    }
}

impl Clone for History {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[async_trait]
pub trait CmdLog: Send {
    async fn log(&self, state: Arc<Mutex<Vec<String>>>) -> &Self;
}

#[async_trait]
impl CmdLog for Pipeline {
    async fn log(&self, state: Arc<Mutex<Vec<String>>>) -> &Self {
        let iter = self.cmd_iter();
        for cmd in iter {
            _ = cmd.log(state.clone());
        }

        self
    }
}

#[async_trait]
impl CmdLog for Cmd {
    async fn log(&self, state: Arc<Mutex<Vec<String>>>) -> &Self {
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
        state.lock().await.push(log);

        self
    }
}
