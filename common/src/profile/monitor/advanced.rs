use serde::{Deserialize, Serialize};

use crate::hyprland::*;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Advanced {
    pub bit_depth: u8,
    pub color_mode: ColorMode,
    pub sdr_brightness: f64,
    pub sdr_saturation: f64,
    pub vrr: Vrr,
    pub transform: MonitorTransform,
}

impl std::fmt::Display for Advanced {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bit_depth_opts = if self.bit_depth == 10 {
            ",bitdepth,10"
        } else {
            ""
        };

        let color_mode_opts =
            if self.color_mode != ColorMode::Unknown && self.color_mode != ColorMode::Srgb {
                format!(",cm,{mode}", mode = self.color_mode)
            } else {
                String::new()
            };

        let hdr_opts = if self.color_mode == ColorMode::Hdr || self.color_mode == ColorMode::HdrEdid
        {
            let mut opts = String::new();

            if (self.sdr_brightness - 1.0).abs() > 0.001 {
                opts += &format!(",sdrbrightness,{:.2}", self.sdr_brightness);
            }

            if (self.sdr_saturation - 1.0).abs() > 0.001 {
                opts += &format!(",sdrsaturation,{:.2}", self.sdr_saturation);
            }

            opts
        } else {
            String::new()
        };

        let vrr_opts = match self.vrr {
            Vrr::Off => format!(",vrr,{vrr}", vrr = self.vrr),
            _ => String::new(),
        };

        let transform_opts = if self.transform != MonitorTransform::Normal {
            format!(",transform,{transform}", transform = self.transform)
        } else {
            String::new()
        };

        write!(
            f,
            "{bit_depth_opts}{color_mode_opts}{hdr_opts}{vrr_opts}{transform_opts}"
        )
    }
}
