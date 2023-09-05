use super::state::RedisState;
use crate::{error::Result, get_cluster_clients, model::*, select_db, CmdLog, History, LogArgs};
use anyhow::Context;
use redis::{AsyncCommands, AsyncIter};

use serde_json::json;
use std::collections::HashMap;
use tauri::State;
use tracing::{info, instrument};

/// 获取键的类型
#[tauri::command]
#[instrument(skip(state, history))]
pub async fn get_key_type(
    state: State<'_, RedisState>,
    history: State<'_, History>,
    id: String,
    db: u8,
    key: String,
) -> Result<String> {
    let mut redis_state = state.0.lock().await;
    let (con, config) = redis_state.get_con_and_config(&id)?;
    select_db(config, db, con, &history).await?;

    let typ: String = redis::cmd("TYPE")
        .arg(&key)
        .log(history.0.clone(), config)
        .query_async(con)
        .await?;
    Ok(typ)
}

/// 删除一个键
#[instrument(skip(state, history))]
#[tauri::command]
pub async fn del_key(
    state: State<'_, RedisState>,
    history: State<'_, History>,
    id: String,
    db: u8,
    key: String,
) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    let (con, config) = redis_state.get_con_and_config(&id)?;
    select_db(config, db, con, &history).await?;

    con.del(&key)
        .await
        .context(format!("删除键失败, id: {id}, key: {key}"))?;
    history.add_log(format!("del {key}"), config);

    info!("删除key: {}成功", key);
    Ok(())
}

/// 删除命名空间匹配的键
#[instrument(skip(state, history))]
#[tauri::command]
pub async fn del_match_keys(
    state: State<'_, RedisState>,
    history: State<'_, History>,
    id: String,
    db: u8,
    match_key: String,
) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    let (con, config) = redis_state.get_con_and_config(&id)?;
    select_db(config, db, con, &history).await?;

    if config.cluster {
        let clients = get_cluster_clients(config, con).await?;

        let mut keys_log = vec![];
        for client in clients {
            let mut keys = vec![];
            let mut cluster_con = client.get_async_connection().await?;

            {
                let mut iter: AsyncIter<'_, String> = cluster_con.scan_match(&match_key).await?;
                while let Some(key) = iter.next_item().await {
                    keys.push(key);
                }
                info!(?keys, "当前要删除的keys:");
            }

            if !keys.is_empty() {
                con.del(&keys).await?;
                keys_log.extend(keys);
            }
        }

        let mut logs = LogArgs!["del"];
        logs.extend(keys_log);
        history.add_log_vec(logs, config);
    } else {
        let mut keys = vec![];
        {
            let mut iter: AsyncIter<'_, String> = con.scan_match(&match_key).await?;
            while let Some(key) = iter.next_item().await {
                keys.push(key);
            }
        }

        if !keys.is_empty() {
            con.del(&keys).await?;
        }
        let mut logs = LogArgs!["del"];
        logs.extend(keys);
        history.add_log_vec(logs, config);
    }

    info!("删除多个key成功");
    Ok(())
}

/// 删除指定key类型中的部分内容
#[tauri::command]
#[instrument(skip(state, history))]
pub async fn del_key_by_value(
    state: State<'_, RedisState>,
    history: State<'_, History>,
    id: String,
    db: u8,
    key: String,
    value: Option<String>,
) -> Result<()> {
    let value = value.unwrap_or_default();
    info!(?key, ?value);

    let mut redis_state = state.0.lock().await;
    let (con, config) = redis_state.get_con_and_config(&id)?;
    select_db(config, db, con, &history).await?;

    let typ: String = redis::cmd("TYPE")
        .arg(&key)
        .log(history.0.clone(), config)
        .query_async(con)
        .await?;

    let logs = match typ.as_str() {
        "string" => {
            con.del(&key).await?;
            LogArgs!["del", &key]
        }
        "list" => {
            con.lrem(&key, 1, &value).await?;
            LogArgs!["lrem", &key, "1", &value]
        }
        "set" => {
            con.srem(&key, &value).await?;
            LogArgs!["srem", &key, &value]
        }
        "zset" => {
            con.zrem(&key, &value).await?;
            LogArgs!["zrem", &key, &value]
        }
        "hash" => {
            con.hdel(&key, &value).await?;
            LogArgs!["hdel", &key, &value]
        }
        "stream" => {
            con.xdel(&key, &[&value]).await?;
            LogArgs!["xdel", &key, &value]
        }
        _ => return Err(format!("不支持的类型: {typ}").into()),
    };

    history.add_log_vec(logs, config);

    info!(?key, "删除键成功: ");

    Ok(())
}

/// 清空所有键
#[tauri::command]
#[instrument(skip(state, history))]
pub async fn clear_keys(
    state: State<'_, RedisState>,
    history: State<'_, History>,
    id: String,
    db: u8,
) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    let (con, config) = redis_state.get_con_and_config(&id)?;
    select_db(config, db, con, &history).await?;

    if config.cluster {
        let clients = get_cluster_clients(config, con).await?;
        for client in clients {
            let mut con = client.get_async_connection().await?;
            redis::cmd("FLUSHDB")
                .log(history.0.clone(), config)
                .query_async(&mut con)
                .await?;
        }
    } else {
        redis::cmd("FLUSHDB")
            .log(history.0.clone(), config)
            .query_async(con)
            .await?;
    }

    info!("清空所有key成功");
    Ok(())
}

/// 获取指定数据库中的所有键
#[tauri::command]
#[instrument(skip(state, history))]
pub async fn get_keys_by_db(
    state: State<'_, RedisState>,
    history: State<'_, History>,
    id: String,
    db: u8,
) -> Result<Vec<String>> {
    let mut redis_state = state.0.lock().await;
    let (con, config) = redis_state.get_con_and_config(&id)?;

    select_db(config, db, con, &history).await?;

    let mut keys = vec![];

    // 集群模式
    if config.cluster {
        let clients = get_cluster_clients(config, con).await?;
        for client in clients {
            let mut con = client.get_async_connection().await?;
            let mut iter: AsyncIter<'_, String> = con.scan_match("*").await?;
            while let Some(val) = iter.next_item().await {
                keys.push(val);
            }
        }
    } else {
        // 单机模式
        let mut iter: AsyncIter<'_, String> = con.scan().await?;
        let logs = LogArgs!["scan", 0, "MATCH", "*", 2000];
        history.add_log_vec(logs, config);

        while let Some(val) = iter.next_item().await {
            keys.push(val);
        }
    }

    info!(?keys, "获取指定数据库key列表信息成功");

    Ok(keys)
}

/// 获取键的基础信息
#[tauri::command]
#[instrument(skip(state, history))]
pub async fn get_key_info(
    state: State<'_, RedisState>,
    history: State<'_, History>,
    id: String,
    db: u8,
    key: String,
) -> Result<KeyInfo> {
    let mut redis_state = state.0.lock().await;
    let (con, config) = redis_state.get_con_and_config(&id)?;
    select_db(config, db, con, &history).await?;

    let (typ, ttl): (String, i64) = redis::pipe()
        .cmd("TYPE")
        .arg(&key)
        .ttl(&key)
        .log(history.0.clone(), config)
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
#[instrument(skip(state, history))]
pub async fn get_key_detail(
    state: State<'_, RedisState>,
    history: State<'_, History>,
    id: String,
    db: u8,
    key: String,
) -> Result<KeyContentDetail> {
    let mut redis_state = state.0.lock().await;
    let (con, config) = redis_state.get_con_and_config(&id)?;
    select_db(config, db, con, &history).await?;

    let (typ, ttl): (String, i64) = redis::pipe()
        .cmd("TYPE")
        .arg(&key)
        .ttl(&key)
        .log(history.0.clone(), config)
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

            history.add_log_vec(LogArgs!["get", &key], config);
        }
        "list" => {
            let count: usize = con.llen(&key).await?;
            let values: Vec<String> = con.lrange(&key, 0, (count as isize) - 1).await?;
            keyinfo.size = count;
            keyinfo.value = RedisValue::List(values);

            history.add_log_vec(LogArgs!["llen", &key], config);
            history.add_log_vec(LogArgs!["lrange", &key, 0, count - 1], config);
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

            history.add_log_vec(LogArgs!["scard", &key], config);
            history.add_log_vec(LogArgs!["sscan", &key, "MATCH", "*"], config);
        }
        "zset" => {
            let count: usize = con.zcard(&key).await?;
            let data: Vec<(String, f64)> =
                con.zrange_withscores(&key, 0, (count as isize) - 1).await?;

            let length = data.len();
            let values: Vec<Z> = data.into_iter().map(|d| Z::new(d.1, d.0)).collect();

            keyinfo.size = length;
            keyinfo.value = RedisValue::ZSet(values);

            history.add_log_vec(LogArgs!["zcard", &key], config);
            history.add_log_vec(LogArgs!["zrange", &key, 0, count - 1, "withscores"], config);
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

            history.add_log_vec(LogArgs!["hlen", &key], config);
            history.add_log_vec(LogArgs!["hscan", &key, "MATCH", "*"], config);
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

            history.add_log_vec(LogArgs!["xlen", &key], config);
            history.add_log_vec(LogArgs!["xrevrange", &key, "+", "-", "count", 200], config);
        }
        _ => return Err(format!("key不存在: {key}, type: {typ}").into()),
    };

    info!(
        key = keyinfo.key,
        r#type = keyinfo.r#type,
        "获取key详细信息成功"
    );

    Ok(keyinfo)
}

/// 重命名键
#[tauri::command]
#[instrument(skip(state, history))]
pub async fn rename_key(
    state: State<'_, RedisState>,
    history: State<'_, History>,
    id: String,
    db: u8,
    key: String,
    new_key: String,
) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    let (con, config) = redis_state.get_con_and_config(&id)?;
    select_db(config, db, con, &history).await?;
    redis::pipe()
        .rename_nx(&key, &new_key)
        .log(history.0.clone(), config)
        .query_async(con)
        .await?;

    info!("重命名key成功, key: {}, new_key: {}", key, new_key);

    Ok(())
}

/// 设置key
#[tauri::command]
#[instrument(skip(state, history))]
pub async fn set_key(
    state: State<'_, RedisState>,
    history: State<'_, History>,
    id: String,
    db: u8,
    keyinfo: AddKeyInfo,
) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    let (con, config) = redis_state.get_con_and_config(&id)?;
    select_db(config, db, con, &history).await?;

    let expired: isize = con.ttl(&keyinfo.key).await?;
    history.add_log_vec(LogArgs!["ttl", &keyinfo.key], config);

    match keyinfo.r#type.as_str() {
        "string" => {
            con.set(&keyinfo.key, &keyinfo.value).await?;
            history.add_log_vec(LogArgs!["set", &keyinfo.key, &keyinfo.value], config);
        }
        "list" => {
            // 键不存在
            if expired == -2 {
                con.lpush(&keyinfo.key, &keyinfo.value).await?;
                history.add_log_vec(
                    vec![
                        "lpush".to_string(),
                        keyinfo.key.to_string(),
                        keyinfo.value.to_string(),
                    ],
                    config,
                );
            } else {
                con.rpush(&keyinfo.key, &keyinfo.value).await?;
                history.add_log_vec(LogArgs!["rpush", &keyinfo.key, &keyinfo.value], config)
            }
        }
        "set" => {
            con.sadd(&keyinfo.key, &keyinfo.value).await?;
            history.add_log_vec(LogArgs!["sadd", &keyinfo.key, &keyinfo.value], config);
        }
        "zset" => {
            con.zadd(
                &keyinfo.key,
                &keyinfo.value,
                keyinfo.score.unwrap_or_default(),
            )
            .await?;
            history.add_log_vec(
                LogArgs![
                    "zadd",
                    &keyinfo.key,
                    &keyinfo.value,
                    keyinfo.score.unwrap_or_default()
                ],
                config,
            );
        }
        "hash" => {
            let field = keyinfo.field.clone().unwrap_or_default();
            match keyinfo.old_field {
                Some(ref old_field) if *old_field != field => {
                    con.hset(&keyinfo.key, &field, &keyinfo.value).await?;
                    con.hdel(&keyinfo.key, old_field).await?;

                    history.add_log_vec(
                        LogArgs!["hset", &keyinfo.key, &field, &keyinfo.value],
                        config,
                    );
                    history.add_log_vec(LogArgs!["hdel", &keyinfo.key, &old_field], config);
                }
                _ => {
                    con.hset(&keyinfo.key, &field, &keyinfo.value).await?;
                    history.add_log_vec(
                        LogArgs!["hset", &keyinfo.key, &field, &keyinfo.value],
                        config,
                    );
                }
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
            .await?;

            let mut logs = LogArgs![
                "xadd",
                &keyinfo.key,
                keyinfo.id.clone().unwrap_or_else(|| "*".to_string()),
            ];
            let values: Vec<String> = value.iter().map(|v| v.0.to_string() + "," + &v.1).collect();
            logs.extend(values);
            history.add_log_vec(logs, config);
        }
        _ => return Err(format!("不支持的类型: {}", keyinfo.r#type).into()),
    };

    match expired {
        // 键不存在(-2)
        -2 => {
            con.persist(&keyinfo.key).await?;
            history.add_log_vec(LogArgs!["persist", &keyinfo.key], config);
        }
        // 已持久化的(-1)
        -1 => return Ok(()),
        _ => {
            con.expire(&keyinfo.key, expired as usize).await?;
            history.add_log_vec(LogArgs!["persist", &keyinfo.key, expired], config);
        }
    };

    info!(?keyinfo, "设置key成功: ");
    Ok(())
}

/// 设置键的过期时间
#[tauri::command]
#[instrument(skip(state, history))]
pub async fn set_key_ttl(
    state: State<'_, RedisState>,
    history: State<'_, History>,
    id: String,
    db: u8,
    key: String,
    ttl: i64,
) -> Result<()> {
    let mut redis_state = state.0.lock().await;
    let (con, config) = redis_state.get_con_and_config(&id)?;
    select_db(config, db, con, &history).await?;

    if ttl < -1 {
        return Err("过期的值不能小于-1".into());
    }

    if ttl == -1 {
        con.persist(&key).await?;
        history.add_log_vec(LogArgs!["persist", &key], config);
    } else {
        con.expire(&key, ttl as usize).await?;
        history.add_log_vec(LogArgs!["expire", &key, ttl], config);
    }

    info!(?key, ?ttl, "设置key的ttl成功");

    Ok(())
}
