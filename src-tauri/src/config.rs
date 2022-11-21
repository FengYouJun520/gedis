use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub split: String,
}

impl RedisConfig {
    pub fn get_url(&self) -> String {
        match (&self.username, &self.password) {
            (Some(username), Some(password)) if !username.is_empty() && !password.is_empty() => {
                format!(
                    "redis://{}:{}@{}:{}",
                    username, password, self.host, self.port
                )
            }
            _ => format!("redis://{}:{}", self.host, self.port),
        }
    }
}
