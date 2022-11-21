use crate::{config::RedisConfig, error::Result};

use super::RedisState;
use anyhow::Context;
use tauri::State;

/// 测试连接
#[tauri::command]
pub async fn test_connection(config: RedisConfig) -> Result<()> {
    let client = redis::Client::open(config.get_url())?;
    client.get_async_connection().await.context("连接失败")?;
    Ok(())
}

/// redis连接
#[tauri::command]
pub async fn connection(state: State<'_, RedisState>, config: RedisConfig) -> Result<()> {
    let client = redis::Client::open(config.get_url())?;
    let con = client.get_async_connection().await.context("连接失败")?;

    let mut redis_state = state.0.lock().await;
    redis_state.add_connection(con, config)?;

    Ok(())
}

/// 断开连接
#[tauri::command]
pub async fn dis_connection(state: State<'_, RedisState>, id: String) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    redis_state.remove_connection(&id)?;
    Ok(())
}

/// 断开所有连接
#[tauri::command]
pub async fn dis_connection_all(state: State<'_, RedisState>) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    redis_state.remove_connection_all()?;
    Ok(())
}
