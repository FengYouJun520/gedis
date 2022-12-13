use serde_json::json;
use tauri::State;
use tracing::info;

use crate::{error::Result, RedisState};

/// 在终端执行执行
#[tauri::command]
pub async fn terminal(
    state: State<'_, RedisState>,
    id: String,
    db: u8,
    args: Option<Vec<String>>,
) -> Result<serde_json::Value> {
    info!(?args);

    let client = state.0.lock().await;
    let mut con = client.get_async_con(&id, db).await?;

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

    let json_result = parse_result(res);

    Ok(json_result)
}

/// 解析redis命令返回的结果
fn parse_result(res: redis::Value) -> serde_json::Value {
    match res {
        redis::Value::Nil => json!(""),
        redis::Value::Int(val) => json!(val),
        redis::Value::Data(ref data) => json!(String::from_utf8_lossy(data)),
        redis::Value::Bulk(data) => {
            let mut result = vec![];
            for val in data {
                info!(?val);
                result.push(json!(parse_result(val)));
            }
            json!(result)
        }
        redis::Value::Status(status) => json!(status),
        redis::Value::Okay => json!("Ok"),
    }
}
