use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::Deserialize;
use uuid::Uuid;

use super::dog::Dog;

// DogDb ------------------------------------------------------------ /

#[derive(Debug, Clone)]
/// Database containing dogs as a thread-safe map of UUIDs to Dog objects
///
/// Uses `Arc<Mutex<HashMap>>` to enable thread-safe shared access across GUI components.
///
/// # Example
/// ```
/// let db = DogDb::new();
/// let dog = Dog::new("Spot", Breed::Dalmation);
/// db.add(dog)?;
/// ```
pub struct DogDb {
    db: Arc<Mutex<HashMap<Uuid, Dog>>>,
}

impl DogDb {
    pub fn new() -> Self {
        Self {
            db: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add(&self, dog: Dog) -> Result<Uuid, DbError> {
        let mut db = self.db.lock().map_err(|_| DbError::Access)?;
        let id = dog.id;
        db.insert(dog.id, dog);
        Result::Ok(id)
    }

    pub fn remove(&self, id: Uuid) -> Result<Dog, DbError> {
        let mut db = self.db.lock().map_err(|_| DbError::Access)?;
        match db.remove(&id) {
            Some(dog) => Result::Ok(dog),
            None => Err(DbError::NotFound),
        }
    }

    pub fn get(&self, id: Uuid) -> Result<Dog, DbError> {
        let db = self.db.lock().map_err(|_| DbError::Access)?;
        match db.get(&id) {
            Some(dog) => Result::Ok(dog.to_owned()),
            None => Err(DbError::NotFound),
        }
    }

    pub fn all(&self) -> Vec<Dog> {
        match self.db.lock() {
            std::result::Result::Ok(db) => db.values().cloned().collect(),
            std::result::Result::Err(_) => vec![],
        }
    }

    pub fn iter_all(&self) -> impl Iterator<Item = Dog> {
        self.all().into_iter()
    }

    pub fn modify(&self, id: Uuid, modify_fn: impl FnOnce(&mut Dog)) -> Result<(), DbError> {
        let mut db = self.db.lock().map_err(|_| DbError::Access)?;
        if let Some(dog) = db.get_mut(&id) {
            modify_fn(dog);
            Result::Ok(())
        } else {
            Err(DbError::NotFound)
        }
    }
}

impl PartialEq for DogDb {
    fn eq(&self, other: &Self) -> bool {
        match (self.db.lock(), other.db.lock()) {
            (std::result::Result::Ok(my_db), std::result::Result::Ok(other_db)) => {
                let my_vec: Vec<Dog> = my_db.values().cloned().collect();
                let other_vec: Vec<Dog> = other_db.values().cloned().collect();
                my_vec == other_vec
            }
            _ => false,
        }
    }
}

impl DogDb {
    pub fn from_json(json_str: &str) -> Result<Self, DbError> {
        #[derive(Deserialize)]
        struct DogList {
            dogs: Vec<Dog>,
        }

        let dog_list: DogList = serde_json::from_str(json_str).map_err(|e| {
            log::error!("{}", e);
            let msg = format!("{}", e);
            DbError::Deserialize(msg)
        })?;
        let db = DogDb::new();

        for dog in dog_list.dogs {
            db.add(dog)?;
        }

        Result::Ok(db)
    }
}

// DbError ------------------------------------------------------------ /

#[derive(Debug, Clone)]
pub enum DbError {
    Access,
    NotFound,
    Deserialize(String),
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::Access => write!(f, "Could not access the database"),
            DbError::NotFound => write!(f, "Could not find the dog in the database"),
            DbError::Deserialize(reason) => {
                write!(f, "Could not deserialize the dog data: {}", reason)
            }
        }
    }
}

impl std::error::Error for DbError {}
