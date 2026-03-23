use serde_json::Value;
use tokio::process::Command;

use crate::error::*;
use crate::logs::*;

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

pub async fn internal_monitor_name() -> Result<Option<String>, Error> {
    let output = Command::new("hyprctl")
        .args(["monitors", "all", "-j"])
        .output()
        .await?;

    let json: Value = serde_json::from_slice(&output.stdout)?;

    match json {
        Value::Array(monitors) => {
            for monitor in monitors {
                let name = monitor["name"].as_str().unwrap_or_default();
                if name.starts_with("eDP") {
                    return Ok(Some(name.to_string()));
                }
            }
        }

        _ => return Ok(None),
    };

    Ok(None)
}

pub async fn external_monitors_names() -> Result<Vec<String>, Error> {
    let output = Command::new("hyprctl")
        .args(["monitors", "-j"])
        .output()
        .await?;

    let json: Value = serde_json::from_slice(&output.stdout)?;
    let mut names = Vec::new();

    if let Value::Array(monitors) = json {
        for monitor in monitors {
            let name = monitor["name"].as_str().unwrap_or_default();
            if !name.is_empty() && !name.starts_with("eDP") {
                names.push(name.to_string());
            }
        }
    };

    Ok(names)
}

pub async fn is_lid_open() -> Result<bool, Error> {
    let content = tokio::fs::read_to_string("/proc/acpi/button/lid/LID/state").await?;
    Ok(content.contains("open"))
}

pub async fn assign_workspace(id: usize, monitor: &str, default: bool) -> Result<(), Error> {
    let persistent = true;
    let persistent_str = if persistent { ",persistent:true" } else { "" };
    let default_str = if default { ",default:true" } else { "" };

    log_workspace_assignment(id, monitor, persistent, default);

    let value = format!("{id},monitor:{monitor}{persistent_str}{default_str}");

    Command::new("hyprctl")
        .args(["keyword", "workspace", &value])
        .output()
        .await?;

    Ok(())
}

pub async fn enable_monitor(monitor: &str) -> Result<(), Error> {
    log_monitor_status(monitor, true);

    let value = format!("{monitor},preferred,auto,1");

    Command::new("hyprctl")
        .args(["keyword", "monitor", &value])
        .output()
        .await?;

    Ok(())
}

pub async fn disable_monitor(monitor: &str) -> Result<(), Error> {
    log_monitor_status(monitor, false);

    let value = format!("{monitor},disable");

    Command::new("hyprctl")
        .args(["keyword", "monitor", &value])
        .output()
        .await?;

    Ok(())
}

pub async fn move_workspace_to_monitor(id: usize, monitor: &str) -> Result<(), Error> {
    log_workspace_move(id, monitor);

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

pub async fn jump_to_workspace(id: usize) -> Result<(), Error> {
    log_jump_to_workspace(id);

    Command::new("hyprctl")
        .args(["dispatch", "workspace", &format!("{id}")])
        .output()
        .await?;

    Ok(())
}
