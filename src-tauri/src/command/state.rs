use crate::{config::RedisConfig, error::Result};
use anyhow::Context;
use redis::{aio::ConnectionLike, Cmd, Pipeline};
use std::{collections::HashMap, fmt::Debug, sync::Arc};
use tauri::async_runtime::Mutex;

/// redis实例
pub struct RedisInstance {
    id: String,
    config: RedisConfig,
    con: RedisConnection,
}

impl Debug for RedisInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedisInstance")
            .field("id", &self.id)
            .field("config", &self.config)
            .finish()
    }
}

impl RedisInstance {
    pub fn new(config: RedisConfig, con: RedisConnection) -> Self {
        Self {
            id: config.id.clone(),
            config,
            con,
        }
    }
}

#[derive(Debug, Default)]
pub struct Redis {
    redis_instances: HashMap<String, RedisInstance>,
}

impl Redis {
    pub fn add_instance(&mut self, con: RedisConnection, config: RedisConfig) -> Result<()> {
        let id = config.id.clone();
        let instance = RedisInstance::new(config, con);
        self.redis_instances.insert(id, instance);

        Ok(())
    }

    pub fn remove_con(&mut self, id: &str) -> Result<()> {
        self.redis_instances.remove(id);
        Ok(())
    }

    pub fn remove_con_all(&mut self) -> Result<()> {
        self.redis_instances.clear();
        Ok(())
    }

    pub fn is_connection(&self, id: &str) -> bool {
        self.redis_instances.contains_key(id)
    }

    pub fn get_con_mut(&mut self, id: &str) -> Result<&mut RedisConnection> {
        let instance = self.redis_instances.get_mut(id).context("客户端未连接")?;
        Ok(&mut instance.con)
    }

    pub fn get_con_and_config(&mut self, id: &str) -> Result<(&mut RedisConnection, &RedisConfig)> {
        let instance = self.redis_instances.get_mut(id).context("客户端未连接")?;
        Ok((&mut instance.con, &instance.config))
    }
}

pub enum RedisConnection {
    Connection(redis::aio::Connection),
    ClusterConnection(redis::cluster_async::ClusterConnection),
}

impl RedisConnection {
    pub async fn new(config: &RedisConfig) -> Result<RedisConnection> {
        if config.cluster {
            let client = redis::cluster::ClusterClient::new(vec![config.clone()])?;
            let con = client.get_async_connection().await?;
            Ok(RedisConnection::ClusterConnection(con))
        } else {
            let client = redis::Client::open(config.clone())?;
            let con = client.get_async_connection().await?;
            Ok(RedisConnection::Connection(con))
        }
    }
}

impl ConnectionLike for RedisConnection {
    fn req_packed_command<'a>(&'a mut self, cmd: &'a Cmd) -> redis::RedisFuture<'a, redis::Value> {
        match self {
            RedisConnection::Connection(con) => con.req_packed_command(cmd),
            RedisConnection::ClusterConnection(con) => con.req_packed_command(cmd),
        }
    }

    fn req_packed_commands<'a>(
        &'a mut self,
        cmd: &'a redis::Pipeline,
        offset: usize,
        count: usize,
    ) -> redis::RedisFuture<'a, Vec<redis::Value>> {
        match self {
            RedisConnection::Connection(con) => con.req_packed_commands(cmd, offset, count),
            RedisConnection::ClusterConnection(con) => con.req_packed_commands(cmd, offset, count),
        }
    }

    fn get_db(&self) -> i64 {
        match self {
            RedisConnection::Connection(con) => con.get_db(),
            RedisConnection::ClusterConnection(con) => con.get_db(),
        }
    }
}

#[derive(Debug, Default)]
pub struct RedisState(pub Arc<Mutex<Redis>>);

const MAX_HISTORY: usize = 5000;

#[derive(Debug, Default, Clone)]
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

pub trait CmdLog {
    fn log(&self, history: Arc<std::sync::Mutex<Vec<String>>>, config: &RedisConfig) -> &Self;
}

impl CmdLog for Pipeline {
    fn log(&self, history: Arc<std::sync::Mutex<Vec<String>>>, config: &RedisConfig) -> &Self {
        let iter = self.cmd_iter();
        for cmd in iter {
            _ = cmd.log(history.clone(), config);
        }

        self
    }
}

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
