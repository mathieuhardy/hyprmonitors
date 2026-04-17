mod monitor;

pub use monitor::*;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::error::*;
use crate::hyprland::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Profile {
    pub monitors: Vec<Monitor>,

    #[serde(skip)]
    pub filepath: Option<PathBuf>,

    #[serde(skip)]
    current_workspace_id: Option<usize>,
}

impl Profile {
    pub async fn new() -> Result<Self, Error> {
        Ok(Self {
            monitors: vec![],
            filepath: None,
            current_workspace_id: active_workspace_id().await?,
        })
    }

    pub async fn from_path(path: &std::path::Path) -> Result<Self, Error> {
        let content = tokio::fs::read_to_string(path).await?;
        let profile: Self = serde_yaml::from_str(&content)?;

        Ok(Self {
            filepath: Some(path.to_path_buf()),
            current_workspace_id: active_workspace_id().await?,
            ..profile
        })
    }

    pub async fn apply(&self, logs: bool, verbose: bool) -> Result<(), Error> {
        // Disable monitors
        for monitor in &self.monitors {
            if monitor.active {
                continue;
            }

            HyprMonitor::disable(&monitor.name, logs, verbose).await?;
        }

        // Assign workspaces to enabled monitors
        for monitor in &self.monitors {
            if !monitor.active {
                continue;
            }

            for workspace in &monitor.workspaces {
                let is_default = if let Some(default_workspace_id) = monitor.default_workspace {
                    *workspace == default_workspace_id
                } else {
                    false
                };

                assign_workspace(*workspace, &monitor.name, is_default, logs, verbose).await?;
            }
        }

        // Enable monitors
        for monitor in &self.monitors {
            if !monitor.active {
                continue;
            }

            HyprMonitor::enable(&monitor.name, &monitor.to_string(), logs, verbose).await?;
        }

        // Move workspaces to enabled monitors
        for monitor in &self.monitors {
            if !monitor.active {
                continue;
            }

            for workspace in &monitor.workspaces {
                move_workspace_to_monitor(*workspace, &monitor.name, logs, verbose).await?;
            }
        }

        // Jump back to workspace
        if let Some(workspace_id) = self.current_workspace_id {
            jump_to_workspace(workspace_id, logs, verbose).await?;
        }

        Ok(())
    }

    pub async fn save(&self) -> Result<Option<&PathBuf>, Error> {
        if let Some(path) = &self.filepath {
            let content = serde_yaml::to_string(self)?;
            tokio::fs::write(path, content).await?;
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }

    pub async fn save_to(&mut self, path: &std::path::Path) -> Result<(), Error> {
        let content = serde_yaml::to_string(self)?;

        tokio::fs::write(path, content).await?;

        self.filepath = Some(path.to_path_buf());

        Ok(())
    }
}

pub async fn load_profiles() -> Result<Vec<Profile>, Error> {
    let mut profiles = Vec::new();

    let home_dir = std::env::home_dir().ok_or(Error::NoHomeDir)?;

    for path in glob::glob(&format!(
        "{}/.config/hyprmonitors/profiles/*.yml",
        home_dir.display()
    ))?
    .flatten()
    {
        if let Ok(profile) = Profile::from_path(&path).await {
            profiles.push(profile);
        }
    }

    Ok(profiles)
}
