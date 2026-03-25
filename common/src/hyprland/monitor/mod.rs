mod color_mode;
mod transform;
mod vrr;

use serde::Deserialize;
use tokio::process::Command;

use crate::error::*;
use crate::logs::*;

pub use color_mode::*;
pub use transform::*;
pub use vrr::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyprMonitor {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub make: String,
    pub model: String,
    pub serial: String,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: f64,
    pub x: i32,
    pub y: i32,
    pub scale: f64,
    pub transform: u8,
    pub vrr: bool,
    pub disabled: bool,
    pub mirror_of: String,
    pub available_modes: Vec<String>,
}

impl HyprMonitor {
    pub async fn get_all() -> Result<Vec<HyprMonitor>, Error> {
        let output = Command::new("hyprctl")
            .args(["monitors", "all", "-j"])
            .output()
            .await?;

        let monitors: Vec<HyprMonitor> = serde_json::from_slice(output.stdout.as_slice())?;
        Ok(monitors)
    }

    pub async fn get_internal() -> Result<Option<HyprMonitor>, Error> {
        let monitors = HyprMonitor::get_all().await?;

        for monitor in monitors {
            if monitor.name.starts_with("eDP") {
                return Ok(Some(monitor));
            }
        }

        Ok(None)
    }

    pub async fn get_externals() -> Result<Vec<HyprMonitor>, Error> {
        let mut names = Vec::new();
        let monitors = HyprMonitor::get_all().await?;

        for monitor in monitors {
            if !monitor.name.starts_with("eDP") {
                names.push(monitor);
            }
        }

        Ok(names)
    }

    pub async fn enable(monitor: &str, verbose: bool) -> Result<(), Error> {
        log_monitor_status(monitor, true);

        let value = format!("{monitor},preferred,auto,1");

        if verbose {
            println!("hyprctl keyword monitor {value}");
        }

        Command::new("hyprctl")
            .args(["keyword", "monitor", &value])
            .output()
            .await?;

        Ok(())
    }

    pub async fn disable(monitor: &str, verbose: bool) -> Result<(), Error> {
        log_monitor_status(monitor, false);

        let value = format!("{monitor},disable");

        if verbose {
            println!("hyprctl keyword monitor {value}");
        }

        Command::new("hyprctl")
            .args(["keyword", "monitor", &value])
            .output()
            .await?;

        Ok(())
    }

    pub async fn is_lid_open() -> Result<bool, Error> {
        let content = tokio::fs::read_to_string("/proc/acpi/button/lid/LID/state").await?;
        Ok(content.contains("open"))
    }
}
