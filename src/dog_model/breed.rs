use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Breed {
    Corgi,
    Dalmation,
    Lab,
    Husky,
    Poodle,
    Retriever,
    Sheperd,
    Other(String),
}

impl Breed {
    pub fn get_value(&self) -> String {
        match self {
            Breed::Corgi => "corgi".to_string(),
            Breed::Dalmation => "dalmation".to_string(),
            Breed::Lab => "lab".to_string(),
            Breed::Husky => "husky".to_string(),
            Breed::Poodle => "poodle".to_string(),
            Breed::Sheperd => "sheperd".to_string(),
            Breed::Retriever => "retriever".to_string(),
            Breed::Other(breed) => breed.to_lowercase(),
        }
    }
}

impl std::fmt::Display for Breed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Breed::Corgi => write!(f, "Corgi"),
            Breed::Dalmation => write!(f, "Dalmation"),
            Breed::Lab => write!(f, "Lab"),
            Breed::Husky => write!(f, "Husky"),
            Breed::Poodle => write!(f, "Poodle"),
            Breed::Sheperd => write!(f, "Sheperd"),
            Breed::Retriever => write!(f, "Retriever"),
            Breed::Other(breed) => write!(f, "{}", breed),
        }
    }
}

impl From<String> for Breed {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "corgi" => Self::Corgi,
            "dalmation" => Self::Dalmation,
            "lab" => Self::Lab,
            "husky" => Self::Husky,
            "poodle" => Self::Poodle,
            "retriever" => Self::Retriever,
            "sheperd" => Self::Sheperd,
            other => Self::Other(String::from(other)),
        }
    }
}

impl Into<String> for Breed {
    fn into(self) -> String {
        match self {
            Self::Corgi => String::from("Corgi"),
            Self::Dalmation => String::from("Dalmation"),
            Self::Lab => String::from("Lab"),
            Self::Husky => String::from("Husky"),
            Self::Poodle => String::from("Poodle"),
            Self::Retriever => String::from("Retriever"),
            Self::Sheperd => String::from("Sheperd"),
            Self::Other(breed) => breed,
        }
    }
}

impl Breed {
    pub fn from_raw_str(value: String) -> Option<Self> {
        if value.is_empty() {
            None
        } else if value == "none" {
            None
        } else {
            Some(Self::from(value))
        }
    }
}
