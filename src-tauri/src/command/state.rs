use crate::{config::RedisConfig, error::Result, redis_log::RedisLog};
use anyhow::Context;
use std::{collections::HashMap, fmt::Debug};
use tauri::async_runtime::Mutex;

#[derive(Default)]
pub struct Redis {
    configs: HashMap<String, RedisConfig>,
    connections: HashMap<String, redis::aio::Connection>,
    histories: HashMap<String, RedisLog>,
}

impl Debug for Redis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Redis")
            .field("configs", &self.configs)
            .field("histories", &self.histories)
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

    pub fn get_log(&self, id: &str) -> Result<&RedisLog> {
        let log = self.histories.get(id).context("获取日志信息失败")?;
        Ok(log)
    }

    pub fn get_config(&self, id: &str) -> Result<&RedisConfig> {
        let config = self.configs.get(id).context("获取配置失败")?;
        Ok(config)
    }
}
