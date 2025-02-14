use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::breed::Breed;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Dog {
    #[serde(with = "uuid::serde::braced")]
    pub id: Uuid,
    pub breed: Breed,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
}

impl Dog {
    pub fn new(name: &str, breed: Breed) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            name: String::from(name),
            breed,
            description: None,
            image: None,
        }
    }
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(String::from(description));
        self
    }

    pub fn image(mut self, image: &str) -> Self {
        self.image = Some(String::from(image));
        self
    }
}

impl std::fmt::Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is a {}", self.name, self.breed)
    }
}
