use serde::Deserialize;
use serde_json::Value;
use tokio::process::Command;

use crate::error::*;
use crate::logs::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyprWorkspace {
    pub id: i32,
    pub name: String,
    pub monitor: String,
    pub monitor_id: i32,
    pub windows: i32,
}

pub async fn workspaces() -> Result<Vec<HyprWorkspace>, Error> {
    let output = Command::new("hyprctl")
        .args(["workspaces", "-j"])
        .output()
        .await?;

    let workspaces: Vec<HyprWorkspace> = serde_json::from_slice(output.stdout.as_slice())?;
    Ok(workspaces)
}

pub async fn active_workspace_id() -> Result<Option<usize>, Error> {
    let output = Command::new("hyprctl")
        .args(["activeworkspace", "-j"])
        .output()
        .await?;

    let json: Value = serde_json::from_slice(&output.stdout)?;

    let id = match json.get("id") {
        Some(Value::Number(n)) => n.as_u64().map(|n| n as usize),
        _ => None,
    };

    Ok(id)
}

pub async fn assign_workspace(
    id: usize,
    monitor: &str,
    default: bool,
    verbose: bool,
) -> Result<(), Error> {
    let persistent = true;
    let persistent_str = if persistent { ",persistent:true" } else { "" };
    let default_str = if default { ",default:true" } else { "" };

    log_workspace_assignment(id, monitor, persistent, default);

    let value = format!("{id},monitor:{monitor}{persistent_str}{default_str}");

    if verbose {
        println!("hyprctl keyword workspace {value}");
    }

    Command::new("hyprctl")
        .args(["keyword", "workspace", &value])
        .output()
        .await?;

    Ok(())
}

pub async fn move_workspace_to_monitor(
    id: usize,
    monitor: &str,
    verbose: bool,
) -> Result<(), Error> {
    log_workspace_move(id, monitor);

    if verbose {
        println!("hyprctl dispatch moveworkspacetomonitor {id} {monitor}");
    }

    Command::new("hyprctl")
        .args([
            "dispatch",
            "moveworkspacetomonitor",
            &format!("{id}"),
            monitor,
        ])
        .output()
        .await?;

    Ok(())
}

pub async fn jump_to_workspace(id: usize, verbose: bool) -> Result<(), Error> {
    log_jump_to_workspace(id);

    if verbose {
        println!("hyprctl dispatch workspace {id}");
    }

    Command::new("hyprctl")
        .args(["dispatch", "workspace", &format!("{id}")])
        .output()
        .await?;

    Ok(())
}
