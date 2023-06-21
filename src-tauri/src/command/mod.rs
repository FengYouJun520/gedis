pub mod conn;
pub mod key_ops;
pub mod state;
pub use conn::*;
pub use key_ops::*;
pub use state::*;
pub mod terminal;
pub use terminal::*;

use crate::{config::RedisConfig, error::Result};

pub async fn select_db(
    config: &RedisConfig,
    db: u8,
    con: &mut RedisConnection,
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
