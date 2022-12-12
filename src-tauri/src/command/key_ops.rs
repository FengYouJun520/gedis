use crate::{error::Result, model::*};
use anyhow::Context;
use redis::{AsyncCommands, AsyncIter};
use serde_json::json;
use std::collections::HashMap;
use tauri::State;
use tracing::{info, instrument};

use super::state::RedisState;

/// 删除一个或多个键
#[instrument]
#[tauri::command]
pub async fn del_key(
    state: State<'_, RedisState>,
    id: String,
    db: usize,
    keys: Vec<String>,
) -> Result<()> {
    let mut client = state.0.lock().await;
    let con = client.get_con_mut(&id).await?;

    redis::pipe()
        .atomic()
        .cmd("SELECT")
        .arg(db)
        .ignore()
        .del(&keys)
        .query_async(con)
        .await
        .context(format!("删除多个键失败, id: {id}"))?;

    info!(?keys, "删除多个key成功");
    Ok(())
}

/// 删除指定key类型中的部分内容
#[instrument]
#[tauri::command]
pub async fn del_key_by_value(
    state: State<'_, RedisState>,
    id: String,
    db: usize,
    key: String,
    value: Option<String>,
) -> Result<()> {
    let value = value.unwrap_or_default();
    info!(?key, ?value);

    let mut client = state.0.lock().await;
    let con = client.get_con_mut(&id).await?;

    let (_, typ): ((), String) = redis::pipe()
        .atomic()
        .cmd("SELECT")
        .arg(db)
        .cmd("type")
        .arg(&key)
        .query_async(con)
        .await
        .map_err(|err| format!("获取键类型失败: {}", err))?;

    match typ.as_str() {
        "string" => con.del(&key).await,
        "list" => con.lrem(&key, 1, &value).await,
        "set" => con.srem(&key, &value).await,
        "zset" => con.zrem(&key, &value).await,
        "hash" => con.hdel(&key, &value).await,
        "stream" => con.xdel(&key, &[value]).await,
        _ => return Err(format!("不支持的类型: {}", typ).into()),
    }?;

    info!(?key, "删除键成功: ");

    Ok(())
}

/// 清空所有键
#[instrument]
#[tauri::command]
pub async fn clear_keys(state: State<'_, RedisState>, id: String, db: usize) -> Result<()> {
    let mut client = state.0.lock().await;
    let con = client.get_con_mut(&id).await?;

    redis::pipe()
        .atomic()
        .cmd("SELECT")
        .arg(db)
        .ignore()
        .cmd("FLUSHDB")
        .query_async(con)
        .await
        .context(format!("清空键失败, id: {id}"))?;

    info!("清空所有key成功");
    Ok(())
}

/// 获取指定数据库中的所有键
#[instrument]
#[tauri::command]
pub async fn get_keys_by_db(
    state: State<'_, RedisState>,
    id: String,
    db: usize,
) -> Result<Vec<String>> {
    let mut client = state.0.lock().await;
    let con = client.get_con_mut(&id).await?;

    redis::cmd("SELECT")
        .arg(db)
        .query_async(con)
        .await
        .map_err(|err| err.to_string())?;

    let mut iter: AsyncIter<'_, String> = con.scan().await.map_err(|err| err.to_string())?;

    let mut keys = vec![];
    while let Some(val) = iter.next_item().await.to_owned() {
        keys.push(val);
    }

    Ok(keys)
}

/// 获取键对应的详细信息
#[instrument]
#[tauri::command]
pub async fn get_key_info(
    state: State<'_, RedisState>,
    id: String,
    db: i32,
    key: String,
) -> Result<KeyInfo> {
    let mut client = state.0.lock().await;
    let con = client.get_con_mut(&id).await?;

    redis::cmd("SELECT")
        .arg(db)
        .query_async(con)
        .await
        .map_err(|err| err.to_string())?;

    let (typ, ttl): (String, i64) = redis::pipe()
        .atomic()
        .cmd("type")
        .arg(&key)
        .ttl(&key)
        .query_async(con)
        .await
        .map_err(|err| err.to_string())?;

    let mut keyinfo = KeyInfo {
        key: key.clone(),
        r#type: typ.clone(),
        total: 0,
        ttl,
        value: RedisValue::String("".into()),
    };

    match typ.as_str() {
        "string" => {
            let val = con.get(&key).await.map_err(|err| err.to_string())?;
            keyinfo.value = RedisValue::String(val);
        }
        "list" => {
            let count: usize = con.llen(&key).await.map_err(|err| err.to_string())?;
            let values: Vec<String> = con
                .lrange(&key, 0, (count as isize) - 1)
                .await
                .map_err(|err| err.to_string())?;
            keyinfo.total = count;
            keyinfo.value = RedisValue::List(values);
        }
        "set" => {
            let count: usize = con.scard(&key).await.map_err(|err| err.to_string())?;
            let mut iter: AsyncIter<'_, String> = con
                .sscan_match(&key, "*")
                .await
                .map_err(|err| err.to_string())?;
            let mut values = vec![];
            while let Some(val) = iter.next_item().await {
                values.push(val);
            }
            keyinfo.total = count;
            keyinfo.value = RedisValue::Set(values);
        }
        "zset" => {
            let count: usize = con.zcard(&key).await.map_err(|err| err.to_string())?;
            let data: Vec<(String, f64)> = con
                .zrange_withscores(&key, 0, (count as isize) - 1)
                .await
                .map_err(|err| err.to_string())?;

            let length = data.len();
            let values: Vec<Z> = data.into_iter().map(|d| Z::new(d.1, d.0)).collect();

            keyinfo.total = length;
            keyinfo.value = RedisValue::ZSet(values);
        }
        "hash" => {
            let count: usize = con.hlen(&key).await.map_err(|err| err.to_string())?;
            let mut iter: AsyncIter<'_, (String, String)> = con
                .hscan_match(&key, "*")
                .await
                .map_err(|err| err.to_string())?;
            let mut values = vec![];
            while let Some((key, value)) = iter.next_item().await {
                values.push(HashResult::new(key, value));
            }

            keyinfo.total = count;
            keyinfo.value = RedisValue::Hash(values);
        }
        "stream" => {
            let count: usize = con.xlen(&key).await.map_err(|err| err.to_string())?;
            let replay: redis::streams::StreamRangeReply = con
                .xrevrange_count(&key, "+", "-", 200)
                .await
                .map_err(|err| err.to_string())?;

            let value = replay
                .ids
                .into_iter()
                .map(|id| {
                    let value: HashMap<String, String> = id
                        .map
                        .into_iter()
                        .map(|m| (m.0, redis::from_redis_value(&m.1).unwrap_or_default()))
                        .collect();
                    StreamResult::new(id.id, json!(value).to_string())
                })
                .collect();

            keyinfo.total = count;
            keyinfo.value = RedisValue::Stream(value);
        }
        _ => return Err(format!("key不存在: {}, type: {}", key, typ).into()),
    };

    info!(?keyinfo, "获取key详细信息成功");

    Ok(keyinfo)
}

/// 重命名键
#[instrument]
#[tauri::command]
pub async fn rename_key(
    state: State<'_, RedisState>,
    id: String,
    db: i32,
    key: String,
    new_key: String,
) -> Result<()> {
    let mut client = state.0.lock().await;
    let con = client.get_con_mut(&id).await?;

    redis::pipe()
        .atomic()
        .cmd("SELECT")
        .arg(db)
        .rename_nx(&key, &new_key)
        .query_async(con)
        .await
        .map_err(|err| err.to_string())?;

    info!("重命名key成功, key: {}, new_key: {}", key, new_key);

    Ok(())
}

/// 设置key
#[instrument]
#[tauri::command]
pub async fn set_key(
    state: State<'_, RedisState>,
    id: String,
    db: i32,
    keyinfo: AddKeyInfo,
) -> Result<()> {
    let mut client = state.0.lock().await;
    let con = client.get_con_mut(&id).await?;

    redis::cmd("SELECT")
        .arg(db)
        .query_async(con)
        .await
        .map_err(|err| err.to_string())?;

    match keyinfo.r#type.as_str() {
        "string" => con.set(&keyinfo.key, &keyinfo.value).await,
        "list" => con.lpush(&keyinfo.key, &keyinfo.value).await,
        "set" => con.sadd(&keyinfo.key, &keyinfo.value).await,
        "zset" => {
            con.zadd(
                &keyinfo.key,
                &keyinfo.value,
                keyinfo.score.unwrap_or_default(),
            )
            .await
        }
        "hash" => {
            con.hset_nx(
                &keyinfo.key,
                keyinfo.field.clone().unwrap_or_default(),
                &keyinfo.value,
            )
            .await
        }
        "stream" => {
            let value: HashMap<String, String> =
                serde_json::from_str(&keyinfo.value).map_err(|err| err.to_string())?;
            let value: Vec<(String, String)> = Vec::from_iter(value.into_iter());
            con.xadd(
                &keyinfo.key,
                keyinfo.id.clone().unwrap_or_else(|| "*".to_string()),
                &value,
            )
            .await
        }
        _ => return Err(format!("不支持的类型: {}", keyinfo.r#type).into()),
    }?;

    info!(?keyinfo, "设置key成功: ");
    Ok(())
}

/// 设置键的过期时间
#[instrument]
#[tauri::command]
pub async fn set_key_ttl(
    state: State<'_, RedisState>,
    id: String,
    db: usize,
    key: String,
    ttl: i64,
) -> Result<()> {
    let mut client = state.0.lock().await;
    let con = client.get_con_mut(&id).await?;

    redis::cmd("SELECT")
        .arg(db)
        .query_async(con)
        .await
        .map_err(|err| err.to_string())?;

    let mut iter: AsyncIter<'_, String> =
        con.scan_match(&key).await.map_err(|err| err.to_string())?;

    let mut keys = vec![];
    while let Some(val) = iter.next_item().await {
        keys.push(val);
    }

    for key in keys {
        if ttl == -1 {
            con.persist(key).await.map_err(|err| err.to_string())?;
        } else {
            con.expire(key, ttl as usize)
                .await
                .map_err(|err| err.to_string())?;
        }
    }

    info!(?key, ?ttl, "设置key的ttl成功");

    Ok(())
}
