use csv::StringRecord;
use redis::{from_redis_value, FromRedisValue};
use serde::{Deserialize, Serialize};
use std::io;

use crate::error::SerializeError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    pub id: String,
    pub host: String,
    pub flags: String,
    pub master_id: String,
    pub ping_sent: u64,
    pub pong_sent: u64,
    pub config_epoch: u64,
    pub link_state: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub slot: Option<u64>,
}

impl NodeInfo {
    pub fn addr_or_default_port(&self, default_port: u16) -> (String, u16) {
        let res: Vec<_> = self.host.split(':').collect();
        let port: Option<u16> = res[1].split('@').next().and_then(|p| p.parse().ok());
        (res[0].to_string(), port.unwrap_or(default_port))
    }

    pub fn is_master(&self) -> bool {
        self.flags.contains("master")
    }
}

impl TryFrom<StringRecord> for NodeInfo {
    type Error = SerializeError;

    fn try_from(value: StringRecord) -> Result<Self, Self::Error> {
        Ok(NodeInfo {
            id: value[0].parse()?,
            host: value[1].parse()?,
            flags: value[2].parse()?,
            master_id: value[3].parse()?,
            ping_sent: value[4].parse()?,
            pong_sent: value[5].parse()?,
            config_epoch: value[6].parse()?,
            link_state: value[7].parse()?,
            slot: value.get(8).and_then(|r| r.parse().ok()),
        })
    }
}

#[derive(Debug)]
pub struct NodesInfo {
    nodes: Vec<NodeInfo>,
}

impl TryFrom<String> for NodesInfo {
    type Error = redis::RedisError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let records = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b' ')
            .double_quote(false)
            .flexible(true)
            .from_reader(value.as_bytes())
            .records()
            .collect::<Result<Vec<StringRecord>, csv::Error>>()
            .map_err(io::Error::from)?;
        let nodes: Vec<NodeInfo> = records.into_iter().flat_map(NodeInfo::try_from).collect();
        Ok(NodesInfo { nodes })
    }
}

impl NodesInfo {
    pub fn master_nodes(&self) -> Vec<NodeInfo> {
        self.nodes
            .iter()
            .filter(|n| n.is_master())
            .cloned()
            .collect()
    }

    pub fn nodes(&self) -> Vec<NodeInfo> {
        self.nodes.to_vec()
    }
}

impl FromRedisValue for NodesInfo {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        let s: String = from_redis_value(v)?;
        let info = s.try_into()?;
        Ok(info)
    }
}
