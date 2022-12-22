use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum RedisValue {
    String(String),
    List(Vec<String>),
    Set(Vec<String>),
    ZSet(Vec<Z>),
    Hash(Vec<HashResult>),
    Stream(Vec<StreamResult>),
}

#[derive(Debug, Serialize)]
pub struct Z {
    pub score: f64,
    pub member: String,
}

impl Z {
    pub fn new(score: f64, member: String) -> Self {
        Self { score, member }
    }
}

#[derive(Debug, Serialize)]
pub struct HashResult {
    pub key: String,
    pub value: String,
}

impl HashResult {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, Serialize)]
pub struct StreamResult {
    pub id: String,
    pub value: String,
}

impl StreamResult {
    pub fn new(id: String, value: String) -> Self {
        Self { id, value }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyContentDetail {
    pub key: String,
    pub r#type: String,
    pub label: String,
    pub ttl: i64,
    pub size: usize,
    pub value: RedisValue,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyInfo {
    pub key: String,
    pub r#type: String,
    pub label: String,
    pub ttl: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddKeyInfo {
    pub r#type: String,
    pub key: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
