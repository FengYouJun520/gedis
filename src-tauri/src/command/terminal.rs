use serde_json::json;
use tauri::State;
use tracing::info;

use crate::{error::Result, RedisState};

/// 在终端执行执行
#[tauri::command]
pub async fn terminal(
    state: State<'_, RedisState>,
    id: String,
    args: Option<Vec<String>>,
) -> Result<serde_json::Value> {
    info!(?args);

    let mut client = state.0.lock().await;
    let con = client.get_con_mut(&id)?;

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
        .query_async(con)
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
                        let mut result: Vec<String> = vec![];
                        for val in data {
                            result.push(redis::from_redis_value(val).unwrap_or_default());
                        }
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
