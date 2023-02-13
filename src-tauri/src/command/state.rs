use crate::{config::RedisConfig, error::Result};
use anyhow::Context;
use redis::{cluster::ClusterClient, Cmd, ConnectionLike, Pipeline};
use std::{collections::HashMap, fmt::Debug, sync::Arc, time::Duration};
use tauri::async_runtime::Mutex;
#[derive(Default)]
pub struct Redis {
    configs: HashMap<String, RedisConfig>,
    connections: HashMap<String, RedisConnection>,
}

pub enum RedisConnection {
    Connection(redis::Connection),
    ClusterConnection(redis::cluster::ClusterConnection),
}

impl RedisConnection {
    pub fn new(config: &RedisConfig) -> Result<RedisConnection> {
        if config.cluster {
            let client = ClusterClient::builder(vec![config.clone()])
                .read_from_replicas()
                .build()?;
            let con = client.get_connection()?;
            Ok(RedisConnection::ClusterConnection(con))
        } else {
            let client = redis::Client::open(config.clone())?;
            let con = client.get_connection_with_timeout(Duration::from_secs(5))?;
            Ok(RedisConnection::Connection(con))
        }
    }
}

impl ConnectionLike for RedisConnection {
    fn req_packed_command(&mut self, cmd: &[u8]) -> redis::RedisResult<redis::Value> {
        match self {
            RedisConnection::Connection(con) => con.req_packed_command(cmd),
            RedisConnection::ClusterConnection(con) => con.req_packed_command(cmd),
        }
    }

    fn req_packed_commands(
        &mut self,
        cmd: &[u8],
        offset: usize,
        count: usize,
    ) -> redis::RedisResult<Vec<redis::Value>> {
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

    fn check_connection(&mut self) -> bool {
        match self {
            RedisConnection::Connection(con) => con.check_connection(),
            RedisConnection::ClusterConnection(con) => con.check_connection(),
        }
    }

    fn is_open(&self) -> bool {
        match self {
            RedisConnection::Connection(con) => con.is_open(),
            RedisConnection::ClusterConnection(con) => con.is_open(),
        }
    }
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
    pub fn add_con(&mut self, con: RedisConnection, config: RedisConfig) -> Result<()> {
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

    pub fn get_con_mut(&mut self, id: &str) -> Result<&mut RedisConnection> {
        let con = self.connections.get_mut(id).context("客户端未连接")?;
        Ok(con)
    }

    pub fn get_con_and_config(&mut self, id: &str) -> Result<(&mut RedisConnection, &RedisConfig)> {
        let con = self.connections.get_mut(id).context("客户端未连接")?;
        let config = self.configs.get_mut(id).context("客户端未连接")?;
        Ok((con, config))
    }
}

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
