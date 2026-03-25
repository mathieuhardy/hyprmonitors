use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
// TODO:remove
//#[serde(rename_all = "PascalCase")]
pub enum MonitorTransform {
    #[default]
    Normal,
    Degrees90,
    Degrees180,
    Degrees270,
    Flipped,
    FlippedDegrees90,
    FlippedDegrees180,
    FlippedDegrees270,
}

impl std::fmt::Display for MonitorTransform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "0"),
            Self::Degrees90 => write!(f, "1"),
            Self::Degrees180 => write!(f, "2"),
            Self::Degrees270 => write!(f, "3"),
            Self::Flipped => write!(f, "4"),
            Self::FlippedDegrees90 => write!(f, "5"),
            Self::FlippedDegrees180 => write!(f, "6"),
            Self::FlippedDegrees270 => write!(f, "7"),
        }
    }
}

impl From<u8> for MonitorTransform {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Normal,
            1 => Self::Degrees90,
            2 => Self::Degrees180,
            3 => Self::Degrees270,
            4 => Self::Flipped,
            5 => Self::FlippedDegrees90,
            6 => Self::FlippedDegrees180,
            7 => Self::FlippedDegrees270,
            _ => Self::Normal,
        }
    }
}

impl MonitorTransform {
    pub fn menu_entry(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Degrees90 => "90°",
            Self::Degrees180 => "180°",
            Self::Degrees270 => "270°",
            Self::Flipped => "Flip",
            Self::FlippedDegrees90 => "Flip+90°",
            Self::FlippedDegrees180 => "Flip+180°",
            Self::FlippedDegrees270 => "Flip+270°",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Normal => Self::Degrees90,
            Self::Degrees90 => Self::Degrees180,
            Self::Degrees180 => Self::Degrees270,
            Self::Degrees270 => Self::Flipped,
            Self::Flipped => Self::FlippedDegrees90,
            Self::FlippedDegrees90 => Self::FlippedDegrees180,
            Self::FlippedDegrees180 => Self::FlippedDegrees270,
            Self::FlippedDegrees270 => Self::Normal,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Self::Normal => Self::FlippedDegrees270,
            Self::Degrees90 => Self::Normal,
            Self::Degrees180 => Self::Degrees90,
            Self::Degrees270 => Self::Degrees180,
            Self::Flipped => Self::Degrees270,
            Self::FlippedDegrees90 => Self::Flipped,
            Self::FlippedDegrees180 => Self::FlippedDegrees90,
            Self::FlippedDegrees270 => Self::FlippedDegrees180,
        }
    }
}
