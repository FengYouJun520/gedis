use redis::InfoDict;
use std::collections::HashMap;
use tauri::State;
use tracing::{info, instrument};

use crate::{config::RedisConfig, error::Result, CmdLog, History, RedisConnection, RedisState};

/// 测试连接
#[tauri::command]
#[instrument(skip_all, fields(name=config.name, host=config.host, port=config.port))]
pub async fn test_connection(history: State<'_, History>, config: RedisConfig) -> Result<()> {
    let mut redis_conn = RedisConnection::new(&config).await?;
    redis::cmd("PING")
        .log(history.0.clone(), &config)
        .query_async(&mut redis_conn)
        .await?;

    info!(?config, "测试连接成功");
    Ok(())
}

/// redis连接
#[tauri::command]
#[instrument(skip_all, fields(id=config.id, host=config.host, port=config.port))]
pub async fn connection(
    state: State<'_, RedisState>,
    history: State<'_, History>,
    config: RedisConfig,
) -> Result<()> {
    let mut redis_conn = RedisConnection::new(&config).await?;
    redis::cmd("PING")
        .log(history.0.clone(), &config)
        .query_async(&mut redis_conn)
        .await?;

    let mut redis_state = state.0.lock().await;

    info!(?config, "连接成功");
    redis_state.add_con(redis_conn, config)?;

    Ok(())
}

/// 判断是否已连接
#[tauri::command]
#[instrument(skip(state))]
pub async fn is_connection(state: State<'_, RedisState>, id: String) -> Result<bool> {
    let redis_state = state.0.lock().await;
    let is_connection = redis_state.is_connection(&id);

    info!(?is_connection, "是否已连接");
    Ok(is_connection)
}

/// ping
#[tauri::command]
#[instrument(skip(state, history))]
pub async fn ping(
    state: State<'_, RedisState>,
    history: State<'_, History>,
    id: String,
) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    let (con, config) = redis_state.get_con_and_config(&id)?;

    redis::cmd("PING")
        .log(history.0.clone(), config)
        .query_async(con)
        .await?;

    info!("ping");
    Ok(())
}

/// change db
#[tauri::command]
#[instrument(skip(state, history))]
pub async fn change_db(
    state: State<'_, RedisState>,
    history: State<'_, History>,
    id: String,
    db: u16,
) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    let (con, config) = redis_state.get_con_and_config(&id)?;

    redis::cmd("SELECT")
        .arg(db)
        .log(history.0.clone(), config)
        .query_async(con)
        .await?;

    info!("change db: {}", db);
    Ok(())
}

/// 断开连接
#[tauri::command]
#[instrument(skip(state))]
pub async fn dis_connection(state: State<'_, RedisState>, id: String) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    redis_state.remove_con(&id)?;

    info!(?id, "断开连接成功");
    Ok(())
}

/// 断开所有连接
#[tauri::command]
#[instrument(skip(state))]
pub async fn dis_connection_all(state: State<'_, RedisState>) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    redis_state.remove_con_all()?;

    info!("断开所有连接成功");
    Ok(())
}

/// 获取redis客户端信息
#[tauri::command]
#[instrument(skip(state, history))]
pub async fn get_info(
    state: State<'_, RedisState>,
    history: State<'_, History>,
    id: String,
) -> Result<HashMap<String, String>> {
    let mut redis_state = state.0.lock().await;
    let (con, config) = redis_state.get_con_and_config(&id)?;

    let info: InfoDict = redis::cmd("INFO")
        .log(history.0.clone(), config)
        .query_async(con)
        .await?;

    let mut info_result = HashMap::new();

    for entry in info.iter() {
        if let Ok(val) = redis::from_redis_value(entry.1) {
            info_result.insert(entry.0.to_string(), val);
        }
    }

    info!("获取redis客户端信息: {}", id);
    Ok(info_result)
}

/// 获取日志
#[tauri::command]
#[instrument(skip(history))]
pub async fn get_logs(history: State<'_, History>) -> Result<Vec<String>> {
    let logs = history.0.lock().unwrap();
    Ok(logs.to_vec())
}

/// 清空日志
#[tauri::command]
#[instrument(skip(history))]
pub async fn clear_logs(history: State<'_, History>) -> Result<()> {
    let mut logs = history.0.lock().unwrap();
    logs.clear();

    Ok(())
}
