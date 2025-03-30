use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum SortDir {
    Asc,
    Dsc,
}

impl Display for SortDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortDir::Asc => write!(f, "Ascending"),
            SortDir::Dsc => write!(f, "Descending"),
        }
    }
}

impl SortDir {
    pub fn get_emoji(&self) -> &'static str {
        match self {
            SortDir::Asc => "🔼",
            SortDir::Dsc => "🔽",
        }
    }
}
