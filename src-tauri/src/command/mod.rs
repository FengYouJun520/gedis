pub mod conn;
pub mod key_ops;
pub mod state;
pub use conn::*;
pub use key_ops::*;
use redis::ConnectionInfo;
pub use state::*;
pub mod terminal;
pub use terminal::*;

use crate::{config::RedisConfig, error::Result, node_info::NodesInfo};

pub async fn select_db<'a>(
    config: &'a RedisConfig,
    db: u8,
    con: &'a mut RedisConnection,
    history: &History,
) -> Result<()> {
    if config.cluster {
        return Ok(());
    }

    redis::cmd("select")
        .arg(db)
        .log(history.0.clone(), config)
        .query_async(con)
        .await?;

    Ok(())
}

pub async fn get_cluster_clients(
    config: &RedisConfig,
    con: &mut RedisConnection,
) -> Result<Vec<redis::Client>> {
    let nodes: NodesInfo = redis::cmd("CLUSTER").arg("nodes").query_async(con).await?;

    let connection_infos: Vec<_> = nodes
        .master_nodes()
        .into_iter()
        .map(|node| {
            let (host, port) = node.addr_or_default_port(config.port);
            ConnectionInfo {
                addr: redis::ConnectionAddr::Tcp(host, port),
                redis: redis::RedisConnectionInfo {
                    db: 0,
                    username: config.username.clone(),
                    password: config.password.clone(),
                },
            }
        })
        .collect();

    let mut clients = vec![];
    for connection_info in connection_infos {
        let client = redis::Client::open(connection_info)?;
        clients.push(client);
    }

    Ok(clients)
}
