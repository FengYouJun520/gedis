use crate::{config::RedisConfig, error::Result, redis_log::RedisLog};
use anyhow::Context;
use std::{collections::HashMap, fmt::Debug};
use tauri::async_runtime::Mutex;

#[derive(Default)]
pub struct Redis {
    configs: HashMap<String, RedisConfig>,
    clients: HashMap<String, redis::Client>,
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
    pub fn add_client(&mut self, client: redis::Client, config: RedisConfig) -> Result<()> {
        self.clients.insert(config.id.to_string(), client);
        self.configs.insert(config.id.to_string(), config);

        Ok(())
    }

    pub fn remove_client(&mut self, id: &str) -> Result<()> {
        self.clients.remove(id);
        self.configs.remove(id);
        Ok(())
    }

    pub fn remove_client_all(&mut self) -> Result<()> {
        self.clients.clear();
        self.configs.clear();
        Ok(())
    }

    pub fn is_connection(&self, id: &str) -> bool {
        self.clients.contains_key(id)
    }

    pub async fn get_async_con(&self, id: &str, db: u8) -> Result<redis::aio::Connection> {
        let client = self.clients.get(id).context("客户端未连接")?;
        let mut con = client.get_async_connection().await?;
        redis::cmd("SELECT").arg(db).query_async(&mut con).await?;
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
