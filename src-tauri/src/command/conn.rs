use redis::InfoDict;
use std::collections::HashMap;
use tauri::State;
use tracing::info;

use crate::{config::RedisConfig, error::Result, RedisState};

/// 测试连接
#[tauri::command]
pub async fn test_connection(config: RedisConfig) -> Result<()> {
    let url = config.get_url();
    info!(?url);
    let client = redis::Client::open(url)?;
    let mut con = client.get_async_connection().await?;
    redis::cmd("PING").query_async(&mut con).await?;

    info!(?config, "连接成功");
    Ok(())
}

/// redis连接
#[tauri::command]
pub async fn connection(state: State<'_, RedisState>, config: RedisConfig) -> Result<()> {
    let client = redis::Client::open(config.get_url())?;
    let mut con = client
        .get_async_connection()
        .await
        .map_err(|err| anyhow::format_err!(err))?;

    redis::cmd("PING").query_async(&mut con).await?;

    let mut redis_state = state.0.lock().await;

    info!(?config, "连接成功");
    redis_state.add_connection(con, config)?;

    Ok(())
}

/// ping
#[tauri::command]
pub async fn ping(state: State<'_, RedisState>, id: String) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    let con = redis_state.get_con_mut(&id)?;

    redis::cmd("PING").query_async(con).await?;

    Ok(())
}

/// 断开连接
#[tauri::command]
pub async fn dis_connection(state: State<'_, RedisState>, id: String) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    redis_state.remove_connection(&id)?;

    let config = redis_state.get_config(&id)?;

    info!(?config, "断开所有连接成功");
    Ok(())
}

/// 断开所有连接
#[tauri::command]
pub async fn dis_connection_all(state: State<'_, RedisState>) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    redis_state.remove_connection_all()?;

    info!("断开所有连接成功");
    Ok(())
}

/// 获取redis客户端信息
#[tauri::command]
pub async fn get_info(state: State<'_, RedisState>, id: String) -> Result<HashMap<String, String>> {
    let mut clients = state.0.lock().await;
    let conn = clients.get_con_mut(&id)?;

    let info: InfoDict = redis::cmd("INFO")
        .query_async(conn)
        .await
        .map_err(|err| err.to_string())?;

    let mut info_result: HashMap<String, String> = HashMap::new();

    for entry in info.iter() {
        match redis::from_redis_value(&entry.1) {
            Ok(val) => {
                info_result.insert(entry.0.to_string(), val);
            }
            Err(_) => {}
        }
    }

    info!(?info_result, "获取redis客户端信息");
    Ok(info_result)
}
