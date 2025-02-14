use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use dioxus::signals::Signal;
use uuid::Uuid;

use crate::dog_model::dog_db::DogDb;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub db: Arc<Mutex<DogDb>>,
    pub archived: Signal<HashSet<Uuid>>,
    pub favorites: Signal<HashSet<Uuid>>,
}

impl AppContext {
    pub fn new(db: DogDb) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            archived: Signal::new(HashSet::new()),
            favorites: Signal::new(HashSet::new()),
        }
    }
}
