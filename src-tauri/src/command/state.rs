use std::collections::HashMap;

use anyhow::Context;
use tauri::async_runtime::Mutex;

use crate::{config::RedisConfig, redis_log::RedisLog};

#[derive(Default)]
pub struct Redis {
    configs: HashMap<String, RedisConfig>,
    clients: HashMap<String, redis::aio::Connection>,
    logs: HashMap<String, RedisLog>,
}

pub struct RedisState(pub Mutex<Redis>);

impl Redis {
    pub fn add_connection(
        &mut self,
        con: redis::aio::Connection,
        config: RedisConfig,
    ) -> anyhow::Result<()> {
        self.clients
            .insert(config.id.to_string(), con)
            .context("添加redis连接失败")?;

        self.configs
            .insert(config.id.to_string(), config)
            .context("添加配置失败");

        Ok(())
    }

    pub fn remove_connection(&mut self, id: &str) -> anyhow::Result<()> {
        self.clients.remove(id).context("断开连接失败")?;
        self.configs.remove(id).context("删除配置失败")?;
        Ok(())
    }

    pub fn remove_connection_all(&mut self) -> anyhow::Result<()> {
        self.clients.clear();
        self.configs.clear();
        Ok(())
    }

    pub fn is_connection(&self, id: &str) -> anyhow::Result<bool> {
        if self.clients.contains_key(id) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_client(&self, id: &str) -> Option<&redis::aio::Connection> {
        self.clients.get(id)
    }

    pub fn get_log(&self, id: &str) -> Option<&RedisLog> {
        self.logs.get(id)
    }

    pub fn get_config(&self, id: &str) -> Option<&RedisConfig> {
        self.configs.get(id)
    }
}
