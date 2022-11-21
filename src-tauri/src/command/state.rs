use std::{collections::HashMap, fmt::Debug};

use anyhow::Context;
use tauri::async_runtime::Mutex;

use crate::{config::RedisConfig, error::Result, redis_log::RedisLog};

#[derive(Default)]
pub struct Redis {
    configs: HashMap<String, RedisConfig>,
    connections: HashMap<String, redis::aio::Connection>,
    logs: HashMap<String, RedisLog>,
}

impl Debug for Redis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Redis")
            .field("configs", &self.configs)
            .field("logs", &self.logs)
            .finish()
    }
}

#[derive(Debug)]
pub struct RedisState(pub Mutex<Redis>);

impl Redis {
    pub fn add_connection(
        &mut self,
        con: redis::aio::Connection,
        config: RedisConfig,
    ) -> anyhow::Result<()> {
        self.connections
            .insert(config.id.to_string(), con)
            .context("添加redis连接失败")?;

        self.configs
            .insert(config.id.to_string(), config)
            .context("添加配置失败")?;

        Ok(())
    }

    pub fn remove_connection(&mut self, id: &str) -> anyhow::Result<()> {
        self.connections.remove(id).context("断开连接失败")?;
        self.configs.remove(id).context("删除配置失败")?;
        Ok(())
    }

    pub fn remove_connection_all(&mut self) -> anyhow::Result<()> {
        self.connections.clear();
        self.configs.clear();
        Ok(())
    }

    pub fn is_connection(&self, id: &str) -> anyhow::Result<bool> {
        if self.connections.contains_key(id) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_con(&self, id: &str) -> Result<&redis::aio::Connection> {
        let res = self.connections.get(id).context("该连接不存在")?;
        Ok(res)
    }

    pub fn get_con_mut(&mut self, id: &str) -> Result<&mut redis::aio::Connection> {
        let res = self.connections.get_mut(id).context("该连接不存在")?;
        Ok(res)
    }

    pub fn get_log(&self, id: &str) -> Result<&RedisLog> {
        let log = self.logs.get(id).context("获取日志信息失败")?;
        Ok(log)
    }

    pub fn get_config(&self, id: &str) -> Result<&RedisConfig> {
        let config = self.configs.get(id).context("获取配置失败")?;
        Ok(config)
    }
}
