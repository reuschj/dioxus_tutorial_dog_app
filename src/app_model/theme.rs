#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Theme {
    Dark,
    Light,
    System,
}

impl Theme {
    pub fn class_name(&self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
            Theme::System => "system",
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Dark => write!(f, "Dark"),
            Theme::Light => write!(f, "Light"),
            Theme::System => write!(f, "System"),
        }
    }
}
