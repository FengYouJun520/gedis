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
    db: u8,
    keys: Vec<String>,
) -> Result<()> {
    let client = state.0.lock().await;
    let mut con = client.get_async_con(&id, db).await?;

    con.del(&keys)
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
    db: u8,
    key: String,
    value: Option<String>,
) -> Result<()> {
    let value = value.unwrap_or_default();
    info!(?key, ?value);

    let client = state.0.lock().await;
    let mut con = client.get_async_con(&id, db).await?;

    let typ: String = redis::cmd("TYPE").arg(&key).query_async(&mut con).await?;

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
pub async fn clear_keys(state: State<'_, RedisState>, id: String, db: u8) -> Result<()> {
    let client = state.0.lock().await;
    let mut con = client.get_async_con(&id, db).await?;

    redis::cmd("FLUSHDB").query_async(&mut con).await?;

    info!("清空所有key成功");
    Ok(())
}

/// 获取指定数据库中的所有键
#[tauri::command]
#[instrument(skip(state))]
pub async fn get_keys_by_db(
    state: State<'_, RedisState>,
    id: String,
    db: u8,
) -> Result<Vec<String>> {
    let client = state.0.lock().await;
    let mut con = client.get_async_con(&id, db).await?;

    let mut iter: AsyncIter<'_, String> = con.scan().await?;

    let mut keys = vec![];
    while let Some(val) = iter.next_item().await.to_owned() {
        keys.push(val);
    }

    info!("获取指定数据库key列表信息成功");

    Ok(keys)
}

/// 获取键对应的详细信息
#[tauri::command]
#[instrument(skip(state))]
pub async fn get_key_info(
    state: State<'_, RedisState>,
    id: String,
    db: u8,
    key: String,
) -> Result<KeyInfo> {
    let client = state.0.lock().await;
    let mut con = client.get_async_con(&id, db).await?;

    let (typ, ttl): (String, i64) = redis::pipe()
        .atomic()
        .cmd("TYPE")
        .arg(&key)
        .ttl(&key)
        .query_async(&mut con)
        .await?;

    let mut keyinfo = KeyInfo {
        key: key.clone(),
        r#type: typ.clone(),
        label: typ[0..1].to_uppercase() + &typ[1..],
        total: 0,
        ttl,
        value: RedisValue::String("".into()),
    };

    match typ.as_str() {
        "string" => {
            let val = con.get(&key).await?;
            keyinfo.value = RedisValue::String(val);
        }
        "list" => {
            let count: usize = con.llen(&key).await?;
            let values: Vec<String> = con.lrange(&key, 0, (count as isize) - 1).await?;
            keyinfo.total = count;
            keyinfo.value = RedisValue::List(values);
        }
        "set" => {
            let count: usize = con.scard(&key).await?;
            let mut iter: AsyncIter<'_, String> = con.sscan_match(&key, "*").await?;
            let mut values = vec![];
            while let Some(val) = iter.next_item().await {
                values.push(val);
            }
            keyinfo.total = count;
            keyinfo.value = RedisValue::Set(values);
        }
        "zset" => {
            let count: usize = con.zcard(&key).await?;
            let data: Vec<(String, f64)> =
                con.zrange_withscores(&key, 0, (count as isize) - 1).await?;

            let length = data.len();
            let values: Vec<Z> = data.into_iter().map(|d| Z::new(d.1, d.0)).collect();

            keyinfo.total = length;
            keyinfo.value = RedisValue::ZSet(values);
        }
        "hash" => {
            let count: usize = con.hlen(&key).await?;
            let mut iter: AsyncIter<'_, (String, String)> = con.hscan_match(&key, "*").await?;
            let mut values = vec![];
            while let Some((key, value)) = iter.next_item().await {
                values.push(HashResult::new(key, value));
            }

            keyinfo.total = count;
            keyinfo.value = RedisValue::Hash(values);
        }
        "stream" => {
            let count: usize = con.xlen(&key).await?;
            let replay: redis::streams::StreamRangeReply =
                con.xrevrange_count(&key, "+", "-", 200).await?;

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
#[tauri::command]
#[instrument(skip(state))]
pub async fn rename_key(
    state: State<'_, RedisState>,
    id: String,
    db: u8,
    key: String,
    new_key: String,
) -> Result<()> {
    let client = state.0.lock().await;
    let mut con = client.get_async_con(&id, db).await?;

    con.rename_nx(&key, &new_key).await?;

    info!("重命名key成功, key: {}, new_key: {}", key, new_key);

    Ok(())
}

/// 设置key
#[tauri::command]
#[instrument(skip(state))]
pub async fn set_key(
    state: State<'_, RedisState>,
    id: String,
    db: u8,
    keyinfo: AddKeyInfo,
) -> Result<()> {
    let client = state.0.lock().await;
    let mut con = client.get_async_con(&id, db).await?;

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
            let value: HashMap<String, String> = serde_json::from_str(&keyinfo.value)?;
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
#[tauri::command]
#[instrument(skip(state))]
pub async fn set_key_ttl(
    state: State<'_, RedisState>,
    id: String,
    db: u8,
    key: String,
    ttl: i64,
) -> Result<()> {
    let client = state.0.lock().await;
    let mut con = client.get_async_con(&id, db).await?;

    let mut iter: AsyncIter<'_, String> = con.scan_match(&key).await?;

    let mut keys = vec![];
    while let Some(val) = iter.next_item().await {
        keys.push(val);
    }

    for key in keys {
        if ttl == -1 {
            con.persist(key).await?;
        } else {
            con.expire(key, ttl as usize).await?;
        }
    }

    info!(?key, ?ttl, "设置key的ttl成功");

    Ok(())
}
