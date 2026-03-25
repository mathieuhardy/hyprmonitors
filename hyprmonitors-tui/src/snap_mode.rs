use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SnapMode {
    Both,
    Centers,
    Edges,
    Off,
}

impl SnapMode {
    pub fn next(self) -> Self {
        match self {
            SnapMode::Off => SnapMode::Edges,
            SnapMode::Edges => SnapMode::Centers,
            SnapMode::Centers => SnapMode::Both,
            SnapMode::Both => SnapMode::Off,
        }
    }
}

impl Display for SnapMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnapMode::Both => write!(f, "Both"),
            SnapMode::Centers => write!(f, "Centers"),
            SnapMode::Edges => write!(f, "Edges"),
            SnapMode::Off => write!(f, "Off"),
        }
    }
}
