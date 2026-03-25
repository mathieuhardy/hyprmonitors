use std::fmt::Display;

use common::profile::Resolution;

use crate::snap_mode::*;

pub enum Status {
    AdvancedSettingsApplied,
    Cancelled(String),
    ConfigurationApplied,
    ConfigurationReverted,
    ConfigurationSaved(String),
    Error(String),
    Grid(i32),
    LoadingMonitors,
    Mirror(String, Option<String>),
    Resolution(Resolution),
    MonitorsLoaded(usize),
    MonitorState(String, bool),
    ProfileSaved(String),
    Scale(f64),
    Snap(SnapMode),
    WorkspaceAssignment(String, Option<Vec<usize>>),
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::AdvancedSettingsApplied => write!(f, "Advanced settings applied"),
            Status::Cancelled(msg) => write!(f, "{msg} cancelled"),
            Status::ConfigurationApplied => write!(f, "Configuration applied"),
            Status::ConfigurationReverted => write!(f, "Reverted to previous configuration"),
            Status::ConfigurationSaved(destination) => {
                write!(f, "Configuration saved to {destination}")
            }
            Status::Error(value) => write!(f, "{value}"),
            Status::Grid(value) => write!(f, "Grid: {value}px"),
            Status::LoadingMonitors => write!(f, "Loading monitors..."),
            Status::Mirror(source, dest) => {
                if let Some(dest) = dest {
                    write!(f, "Mirror: {source} -> {dest}")
                } else {
                    write!(f, "Mirroring disabled for {source}")
                }
            }
            Status::Resolution(value) => write!(f, "Resolution: {value}"),
            Status::MonitorsLoaded(count) => {
                write!(f, "Loaded {count} monitor(s)")
            }
            Status::MonitorState(name, state) => {
                write!(f, "{name} {}", if *state { "enabled" } else { "disabled" })
            }
            Status::ProfileSaved(name) => write!(f, "Profile {name} saved"),
            Status::Scale(value) => write!(f, "Scale: {value:.2}"),
            Status::Snap(value) => write!(f, "Snap: {value}"),
            Status::WorkspaceAssignment(name, workspaces) => {
                if let Some(workspaces) = workspaces {
                    let workspaces: String = workspaces
                        .iter()
                        .map(|w| w.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    write!(f, "Assigned workspaces [{workspaces}] to {name}")
                } else {
                    write!(f, "Cleared workspace assignments for {name}")
                }
            }
        }
    }
}
