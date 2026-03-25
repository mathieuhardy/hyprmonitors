use serde::Deserialize;

use crate::error::*;
use crate::hyprland::*;

#[derive(Deserialize)]
pub struct Monitor {
    name: String,
    workspaces: Vec<usize>,
    default_workspace: Option<usize>,
}

#[derive(Default)]
pub struct CurrentConfig {
    workspace_id: Option<usize>,
    internal_monitor_name: Option<String>,
    external_monitors_names: Vec<String>,
    is_lid_open: bool,
}

impl CurrentConfig {
    pub async fn new() -> Result<Self, Error> {
        Ok(Self {
            workspace_id: active_workspace_id().await?,
            internal_monitor_name: internal_monitor_name().await?,
            external_monitors_names: external_monitors_names().await?,
            is_lid_open: is_lid_open().await?,
        })
    }
}

#[derive(Deserialize)]
pub struct Profile {
    monitors: Vec<Monitor>,

    #[serde(skip)]
    current_config: CurrentConfig,
}

impl Profile {
    pub async fn new() -> Result<Self, Error> {
        let current_config = CurrentConfig::new().await?;
        let mut monitors = Vec::new();

        match (
            current_config.internal_monitor_name.clone(),
            current_config.is_lid_open,
            current_config.external_monitors_names.first(),
        ) {
            // - Internal monitor found
            // - Lid opened
            // - No external found
            (Some(internal_monitor), true, None) => monitors.push(Monitor {
                name: internal_monitor,
                workspaces: (1..=10).collect(),
                default_workspace: Some(1),
            }),

            // - Lid closed
            // - External found
            (_, false, Some(external_monitor)) => monitors.push(Monitor {
                name: external_monitor.to_string(),
                workspaces: (1..=10).collect(),
                default_workspace: Some(1),
            }),

            // - Internal monitor found
            // - Lid opened
            // - External found
            (Some(internal_monitor), true, Some(external_monitor)) => {
                monitors.push(Monitor {
                    name: internal_monitor,
                    workspaces: vec![10],
                    default_workspace: Some(10),
                });

                monitors.push(Monitor {
                    name: external_monitor.to_string(),
                    workspaces: (1..=9).collect(),
                    default_workspace: Some(1),
                });
            }

            _ => (),
        }

        Ok(Self {
            monitors,
            current_config,
        })
    }

    pub async fn from_path(path: &std::path::Path) -> Result<Self, Error> {
        let content = tokio::fs::read_to_string(path).await?;
        let profile: Self = toml::from_str(&content)?;

        Ok(Profile {
            current_config: CurrentConfig::new().await?,
            ..profile
        })
    }

    pub async fn apply(&self) -> Result<(), Error> {
        // Disable monitors
        for monitor in self.monitors_to_disable() {
            disable_monitor(&monitor).await?;
        }

        // Assign workspaces to enabled monitors
        for monitor in &self.monitors {
            for workspace in &monitor.workspaces {
                let is_default = if let Some(default_workspace_id) = monitor.default_workspace {
                    *workspace == default_workspace_id
                } else {
                    false
                };

                assign_workspace(*workspace, &monitor.name, is_default).await?;
            }
        }

        // Enable monitors
        for monitor in self.monitors_to_enable() {
            enable_monitor(&monitor).await?;
        }

        // Move workspaces to enabled monitors
        for monitor in &self.monitors {
            for workspace in &monitor.workspaces {
                move_workspace_to_monitor(*workspace, &monitor.name).await?;
            }
        }

        // Jump back to workspace
        if let Some(workspace_id) = self.current_config.workspace_id {
            jump_to_workspace(workspace_id).await?;
        }

        Ok(())
    }

    fn monitors_to_disable(&self) -> Vec<String> {
        let mut monitors = Vec::new();

        // Special monitor: internal
        if !self.current_config.is_lid_open {
            if let Some(internal_monitor_name) = &self.current_config.internal_monitor_name {
                monitors.push(internal_monitor_name.clone());
            }
        }

        // Other monitors
        for monitor in &self.current_config.external_monitors_names {
            // Check if connected monitor is declared in profile configuration
            if !self.monitors.iter().any(|m| m.name == *monitor) {
                monitors.push(monitor.clone());
            }
        }

        monitors
    }

    fn monitors_to_enable(&self) -> Vec<String> {
        let mut monitors = Vec::new();

        // Special monitor: internal
        if self.current_config.is_lid_open {
            if let Some(internal_monitor_name) = &self.current_config.internal_monitor_name {
                monitors.push(internal_monitor_name.clone());
            }
        }

        // Other monitors
        for monitor in &self.current_config.external_monitors_names {
            // Check if connected monitor is declared in profile configuration
            if self.monitors.iter().any(|m| m.name == *monitor) {
                monitors.push(monitor.clone());
            }
        }

        monitors
    }
}
