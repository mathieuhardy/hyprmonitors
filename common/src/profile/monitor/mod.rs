mod advanced;
mod position;
mod resolution;

pub use advanced::*;
pub use position::*;
pub use resolution::*;

use serde::{Deserialize, Serialize};

use crate::profile::*;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Monitor {
    pub name: String,
    pub uniq: String,
    pub active: bool,

    pub position: Position,

    pub scale: f64,
    pub resolution: Resolution,
    #[serde(skip)]
    pub resolutions: Vec<Resolution>,

    pub is_mirrored: bool,
    pub mirror_source: String,
    #[serde(skip)]
    pub mirror_targets: Vec<String>,

    pub advanced: Advanced,

    pub workspaces: Vec<usize>,
    pub default_workspace: Option<usize>,
}

impl From<HyprMonitor> for Monitor {
    fn from(m: HyprMonitor) -> Self {
        let resolutions = m
            .available_modes
            .iter()
            .filter_map(|s| parse_mode(s))
            .collect();

        Self {
            name: m.name.clone(),
            uniq: format!("{} {} {}", m.make, m.model, m.serial),
            active: !m.disabled,
            position: Position { x: m.x, y: m.y },
            scale: m.scale,
            resolution: Resolution {
                width: m.width,
                height: m.height,
                hz: m.refresh_rate,
            },
            resolutions,
            is_mirrored: !m.mirror_of.is_empty() && m.mirror_of != "none",
            mirror_source: if !m.mirror_of.is_empty() && m.mirror_of != "none" {
                m.mirror_of.clone()
            } else {
                String::new()
            },
            mirror_targets: vec![],
            advanced: Advanced {
                bit_depth: 8,
                color_mode: ColorMode::Unknown,
                sdr_brightness: 1.0,
                sdr_saturation: 1.0,
                vrr: if m.vrr { Vrr::On } else { Vrr::Off },
                transform: m.transform.into(),
            },
            workspaces: vec![],
            default_workspace: None,
        }
    }
}

impl std::fmt::Display for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.active {
            return write!(f, "{name},disable", name = self.name);
        }

        let position_opts = self.position.to_string();
        let scale_opts = format!(",{:.2}", self.scale);
        let resolution_opts = self.resolution.to_string();
        let mirror_opts = format!(",mirror,{src}", src = self.mirror_source);

        if self.is_mirrored && !self.mirror_source.is_empty() {
            return write!(
                f,
                "{name}{resolution_opts}{position_opts}{scale_opts}{mirror_opts}",
                name = self.name,
            );
        }

        let advanced_opts = self.advanced.to_string();

        let output = format!(
            "{name}{resolution_opts}{position_opts}{scale_opts}{advanced_opts}",
            name = self.name,
        );

        write!(f, "{output}")
    }
}

impl Monitor {
    pub fn effective_width(&self) -> i32 {
        match self.advanced.transform {
            MonitorTransform::Degrees90
            | MonitorTransform::Degrees270
            | MonitorTransform::FlippedDegrees90
            | MonitorTransform::FlippedDegrees270 => {
                (self.resolution.height as f64 / self.scale) as i32
            }

            _ => (self.resolution.width as f64 / self.scale) as i32,
        }
    }

    pub fn effective_height(&self) -> i32 {
        match self.advanced.transform {
            MonitorTransform::Degrees90
            | MonitorTransform::Degrees270
            | MonitorTransform::FlippedDegrees90
            | MonitorTransform::FlippedDegrees270 => {
                (self.resolution.width as f64 / self.scale) as i32
            }

            _ => (self.resolution.height as f64 / self.scale) as i32,
        }
    }
}

fn parse_mode(s: &str) -> Option<Resolution> {
    let parts: Vec<&str> = s.splitn(2, '@').collect();
    if parts.len() != 2 {
        return None;
    }

    let res: Vec<&str> = parts[0].splitn(2, 'x').collect();
    if res.len() != 2 {
        return None;
    }

    let width: u32 = res[0].parse().ok()?;
    let height: u32 = res[1].parse().ok()?;

    let hz_str = parts[1].trim_end_matches("Hz");
    let hz: f64 = hz_str.parse().ok()?;

    Some(Resolution { width, height, hz })
}
