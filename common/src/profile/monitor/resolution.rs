use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
    pub hz: f64,
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            ",{w}x{h}@{hz:.2}",
            w = self.width,
            h = self.height,
            hz = self.hz
        )
    }
}
