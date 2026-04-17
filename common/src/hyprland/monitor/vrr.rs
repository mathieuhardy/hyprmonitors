use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
pub enum Vrr {
    #[default]
    Off,
    On,
    FullscreenOnly,
}

impl std::fmt::Display for Vrr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Off => write!(f, "0"),
            Self::On => write!(f, "1"),
            Self::FullscreenOnly => write!(f, "2"),
        }
    }
}

impl Vrr {
    pub fn menu_entry(&self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::On => "On",
            Self::FullscreenOnly => "Fullscreen",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::FullscreenOnly,
            Self::FullscreenOnly => Self::Off,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Self::Off => Self::FullscreenOnly,
            Self::On => Self::Off,
            Self::FullscreenOnly => Self::On,
        }
    }
}
