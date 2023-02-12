use redis::{ConnectionInfo, IntoConnectionInfo};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub split: String,
    pub cluster: bool,
}

impl IntoConnectionInfo for RedisConfig {
    fn into_connection_info(self) -> redis::RedisResult<ConnectionInfo> {
        Ok(ConnectionInfo {
            addr: redis::ConnectionAddr::Tcp(self.host, self.port),
            redis: redis::RedisConnectionInfo {
                db: 0,
                username: self.username,
                password: self.password,
            },
        })
    }
}
