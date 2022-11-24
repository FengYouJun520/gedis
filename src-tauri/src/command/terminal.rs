use tauri::State;
use tracing::{info, instrument};

use crate::{error::Result, RedisState};

/// 在终端执行执行
#[instrument]
#[tauri::command]
pub async fn terminal(
    state: State<'_, RedisState>,
    id: String,
    db: usize,
    values: Option<Vec<String>>,
) -> Result<String> {
    info!(?values);

    let mut client = state.0.lock().await;
    let con = client.get_con_mut(&id)?;

    let Some(args) = values else {
        return Ok("".to_string());
    };

    if args.is_empty() {
        return Ok("".to_string());
    }

    let cmd_name = &args[0];
    let mut cmd = redis::cmd(&cmd_name);

    for arg in args.iter().skip(1) {
        cmd.arg(&arg);
    }

    let res: Option<String> = cmd.query_async(con).await?;

    Ok(res.unwrap_or_default())
}
