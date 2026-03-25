use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
// TODO: remove
//#[serde(rename_all = "PascalCase")]
pub enum ColorMode {
    #[default]
    Auto,
    Edid,
    Hdr,
    HdrEdid,
    Srgb,
    Unknown,
    Wide,
}

impl std::fmt::Display for ColorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Edid => write!(f, "edid"),
            Self::Hdr => write!(f, "hdr"),
            Self::HdrEdid => write!(f, "hdredid"),
            Self::Srgb => write!(f, "srgb"),
            Self::Unknown => write!(f, ""),
            Self::Wide => write!(f, "wide"),
        }
    }
}

impl ColorMode {
    pub fn menu_entry(&self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::Edid => "Edid",
            Self::Hdr => "Hdr",
            Self::HdrEdid => "HdrEdid",
            Self::Srgb => "Srgb",
            Self::Unknown => "",
            Self::Wide => "Wide",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Unknown => Self::Auto,
            Self::Auto => Self::Edid,
            Self::Edid => Self::Hdr,
            Self::Hdr => Self::HdrEdid,
            Self::HdrEdid => Self::Srgb,
            Self::Srgb => Self::Wide,
            Self::Wide => Self::Unknown,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Self::Unknown => Self::Wide,
            Self::Auto => Self::Unknown,
            Self::Edid => Self::Auto,
            Self::Hdr => Self::Edid,
            Self::HdrEdid => Self::Hdr,
            Self::Srgb => Self::HdrEdid,
            Self::Wide => Self::Srgb,
        }
    }
}
