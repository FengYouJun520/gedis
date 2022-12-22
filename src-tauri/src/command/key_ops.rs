use crate::{
    error::{Result, SerializeError},
    model::*,
};
use anyhow::Context;
use redis::{AsyncCommands, AsyncIter};
use serde_json::json;
use std::collections::HashMap;
use tauri::State;
use tracing::{info, instrument};

use super::state::RedisState;

/// 获取键的类型
#[instrument]
#[tauri::command]
pub async fn get_key_type(
    state: State<'_, RedisState>,
    id: String,
    db: u8,
    key: String,
) -> Result<String> {
    let mut redis_state = state.0.lock().await;
    let con = redis_state.get_con_mut(&id).await?;
    redis::cmd("SELECT").arg(db).query_async(con).await?;

    let typ: String = redis::cmd("TYPE").arg(&key).query_async(con).await?;
    Ok(typ)
}

/// 删除一个键
#[instrument(skip(state))]
#[tauri::command]
pub async fn del_key(state: State<'_, RedisState>, id: String, db: u8, key: String) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    let con = redis_state.get_con_mut(&id).await?;
    redis::cmd("SELECT").arg(db).query_async(con).await?;

    con.del(&key)
        .await
        .context(format!("删除键失败, id: {id}, key: {key}"))?;

    info!("删除key: {}成功", key);
    Ok(())
}

/// 删除命名空间匹配的键
#[instrument(skip(state))]
#[tauri::command]
pub async fn del_match_keys(
    state: State<'_, RedisState>,
    id: String,
    db: u8,
    match_key: String,
) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    let con = redis_state.get_con_mut(&id).await?;
    redis::cmd("SELECT").arg(db).query_async(con).await?;

    let mut iter: AsyncIter<String> = con.scan_match(&match_key).await?;
    let mut keys = vec![];
    while let Some(key) = iter.next_item().await {
        keys.push(key);
    }

    con.del(&keys).await.context(format!("删除多个键失败"))?;

    info!("删除多个key成功");
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

    let mut redis_state = state.0.lock().await;
    let con = redis_state.get_con_mut(&id).await?;
    redis::cmd("SELECT").arg(db).query_async(con).await?;

    let typ: String = redis::cmd("TYPE").arg(&key).query_async(con).await?;

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
    let mut redis_state = state.0.lock().await;
    let con = redis_state.get_con_mut(&id).await?;

    redis::pipe()
        .cmd("SELECT")
        .arg(db)
        .cmd("FLUSHDB")
        .query_async(con)
        .await?;

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
    let mut redis_state = state.0.lock().await;
    let con = redis_state.get_con_mut(&id).await?;

    redis::cmd("SELECT").arg(db).query_async(con).await?;

    let mut iter: AsyncIter<'_, String> = con.scan().await?;

    let mut keys = vec![];
    while let Some(val) = iter.next_item().await.to_owned() {
        keys.push(val);
    }

    info!("获取指定数据库key列表信息成功");

    Ok(keys)
}

/// 获取键的基础信息
#[tauri::command]
#[instrument(skip(state))]
pub async fn get_key_info(
    state: State<'_, RedisState>,
    id: String,
    db: u8,
    key: String,
) -> Result<KeyInfo> {
    let mut redis_state = state.0.lock().await;
    let con = redis_state.get_con_mut(&id).await?;
    redis::cmd("SELECT").arg(db).query_async(con).await?;

    let (typ, ttl): (String, i64) = redis::pipe()
        .cmd("TYPE")
        .arg(&key)
        .ttl(&key)
        .query_async(con)
        .await?;

    let label = typ[0..1].to_uppercase() + &typ[1..];

    let keyinfo = KeyInfo {
        key,
        r#type: typ,
        label,
        ttl,
    };

    info!(?keyinfo, "获取key基础信息成功");

    Ok(keyinfo)
}

/// 获取键对应的详细信息
#[tauri::command]
#[instrument(skip(state))]
pub async fn get_key_detail(
    state: State<'_, RedisState>,
    id: String,
    db: u8,
    key: String,
) -> Result<KeyContentDetail> {
    let mut redis_state = state.0.lock().await;
    let con = redis_state.get_con_mut(&id).await?;

    let (typ, ttl): (String, i64) = redis::pipe()
        .cmd("SELECT")
        .arg(db)
        .ignore()
        .cmd("TYPE")
        .arg(&key)
        .ttl(&key)
        .query_async(con)
        .await?;

    let mut keyinfo = KeyContentDetail {
        key: key.clone(),
        r#type: typ.clone(),
        label: typ[0..1].to_uppercase() + &typ[1..],
        size: 0,
        ttl,
        value: RedisValue::String("".into()),
    };

    match typ.as_str() {
        "string" => {
            let val: String = con.get(&key).await?;
            keyinfo.size = val.len();
            keyinfo.value = RedisValue::String(val);
        }
        "list" => {
            let count: usize = con.llen(&key).await?;
            let values: Vec<String> = con.lrange(&key, 0, (count as isize) - 1).await?;
            keyinfo.size = count;
            keyinfo.value = RedisValue::List(values);
        }
        "set" => {
            let count: usize = con.scard(&key).await?;
            let mut iter: AsyncIter<'_, String> = con.sscan_match(&key, "*").await?;
            let mut values = vec![];
            while let Some(val) = iter.next_item().await {
                values.push(val);
            }
            keyinfo.size = count;
            keyinfo.value = RedisValue::Set(values);
        }
        "zset" => {
            let count: usize = con.zcard(&key).await?;
            let data: Vec<(String, f64)> =
                con.zrange_withscores(&key, 0, (count as isize) - 1).await?;

            let length = data.len();
            let values: Vec<Z> = data.into_iter().map(|d| Z::new(d.1, d.0)).collect();

            keyinfo.size = length;
            keyinfo.value = RedisValue::ZSet(values);
        }
        "hash" => {
            let count: usize = con.hlen(&key).await?;
            let mut iter: AsyncIter<'_, (String, String)> = con.hscan_match(&key, "*").await?;
            let mut values = vec![];
            while let Some((key, value)) = iter.next_item().await {
                values.push(HashResult::new(key, value));
            }

            keyinfo.size = count;
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

            keyinfo.size = count;
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
    let mut redis_state = state.0.lock().await;
    let con = redis_state.get_con_mut(&id).await?;

    redis::pipe()
        .cmd("SELECT")
        .arg(db)
        .ignore()
        .rename_nx(&key, &new_key)
        .query_async(con)
        .await?;

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
    let mut redis_state = state.0.lock().await;
    let con = redis_state.get_con_mut(&id).await?;
    redis::cmd("SELECT").arg(db).query_async(con).await?;
    let expired: isize = con.ttl(&keyinfo.key).await?;
    info!(expired);

    match keyinfo.r#type.as_str() {
        "string" => con.set(&keyinfo.key, &keyinfo.value).await,
        "list" => {
            // 键不存在
            if expired == -2 {
                con.lpush(&keyinfo.key, &keyinfo.value).await
            } else {
                con.rpush(&keyinfo.key, &keyinfo.value).await
            }
        }
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
            let field = keyinfo.field.clone().unwrap_or_default();
            match keyinfo.old_field {
                Some(ref old_field) if *old_field != field => {
                    con.hset(&keyinfo.key, &field, &keyinfo.value).await?;
                    con.hdel(&keyinfo.key, &old_field).await
                }
                _ => con.hset(&keyinfo.key, &field, &keyinfo.value).await,
            }
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

    match expired {
        // 键不存在(-2)
        -2 => con.persist(&keyinfo.key).await?,
        // 已持久化的(-1)
        -1 => return Ok(()),
        _ => con.expire(&keyinfo.key, expired as usize).await?,
    };

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
    let mut redis_state = state.0.lock().await;
    let con = redis_state.get_con_mut(&id).await?;
    redis::cmd("SELECT").arg(db).query_async(con).await?;

    if ttl < -1 {
        return Err(SerializeError::from("过期的值不能小于-1"));
    }

    if ttl == -1 {
        con.persist(&key).await?;
    } else {
        con.expire(&key, ttl as usize).await?;
    }

    info!(?key, ?ttl, "设置key的ttl成功");

    Ok(())
}
