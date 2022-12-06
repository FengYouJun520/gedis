use redis::FromRedisValue;
use serde_json::json;
use tauri::State;
use tracing::info;

use crate::{error::Result, RedisState};

/// 在终端执行执行
#[tauri::command]
pub async fn terminal(
    state: State<'_, RedisState>,
    id: String,
    db: i32,
    args: Option<Vec<String>>,
) -> Result<serde_json::Value> {
    info!(?args);

    let client = state.0.lock().await;
    let mut con = client.get_con(&id).await?;
    redis::cmd("SELECT").arg(db).query_async(&mut con).await?;

    let Some(args) = args else {
        return Ok(json!(""));
    };

    if args.is_empty() {
        return Ok(json!(""));
    }

    let cmd_name = &args[0];
    let args = if args.len() == 1 {
        vec![]
    } else {
        args[1..].to_vec()
    };

    let res: redis::Value = redis::cmd(cmd_name.as_ref())
        .arg(args)
        .query_async(&mut con)
        .await?;
    match res {
        redis::Value::Nil => Ok(json!("")),
        redis::Value::Int(val) => Ok(json!(val)),
        redis::Value::Data(data) => Ok(json!(String::from_utf8(data).unwrap_or_default())),
        redis::Value::Bulk(ref data) => {
            let mut result: Vec<serde_json::Value> = vec![];
            for val in data {
                let val = match val {
                    redis::Value::Nil => json!(""),
                    redis::Value::Int(val) => json!(val),
                    redis::Value::Data(data) => {
                        json!(String::from_utf8(data.to_owned()).unwrap_or_default())
                    }
                    redis::Value::Bulk(ref data) => {
                        let result: Vec<String> = FromRedisValue::from_redis_values(&data)?;
                        json!(result)
                    }
                    redis::Value::Status(status) => json!(status),
                    redis::Value::Okay => json!("OK"),
                };
                result.push(val);
            }

            Ok(json!(result))
        }
        redis::Value::Status(status) => Ok(json!(status)),
        redis::Value::Okay => Ok(json!("OK")),
    }
}
