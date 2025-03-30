use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use dioxus::signals::{Readable, Signal, Writable};
use gloo_storage::Storage;
use uuid::Uuid;

use crate::{dog_model::dog_db::DogDb, util::app_local_storage::AppLocalStorage};

static DB_KEY: &str = "dog_db";
static ARCHIVED_KEY: &str = "archived";
static FAVORITES_KEY: &str = "favorites";

/// AppContext represents the application context.
#[derive(Debug, Clone)]
pub struct AppContext {
    pub db: Arc<Mutex<DogDb>>,
    pub archived: Signal<HashSet<Uuid>>,
    pub favorites: Signal<HashSet<Uuid>>,
}

impl AppContext {
    pub fn new(db: DogDb) -> Self {
        let archived =
            AppLocalStorage::get::<HashSet<Uuid>>(ARCHIVED_KEY).unwrap_or(HashSet::new());
        let favorites =
            AppLocalStorage::get::<HashSet<Uuid>>(FAVORITES_KEY).unwrap_or(HashSet::new());
        Self {
            db: Arc::new(Mutex::new(db)),
            archived: Signal::new(archived),
            favorites: Signal::new(favorites),
        }
    }

    pub fn init() -> Self {
        Self::new(DogDb::new())
    }

    pub fn restore() -> Self {
        Self::restore_or(DogDb::new())
    }

    pub fn restore_or(db: DogDb) -> Self {
        let db = AppLocalStorage::get(DB_KEY).unwrap_or(db);
        Self::new(db)
    }

    // DB -------------------------------------------------------------------------------- /

    pub fn save_db(&self) {
        AppLocalStorage::set(DB_KEY, self.db.lock().unwrap().clone())
            .expect("Failed to save database");
    }

    pub fn set_db(&mut self, db: DogDb) {
        *self.db.lock().unwrap() = db;
        self.save_db();
    }

    pub fn restore_db(&self) {
        let db = AppLocalStorage::get(DB_KEY).unwrap_or(DogDb::new());
        *self.db.lock().unwrap() = db;
    }

    pub fn clear_db(&mut self) {
        AppLocalStorage::delete(DB_KEY);
        *self.db.lock().unwrap() = DogDb::new();
        self.save_db();
    }

    // Archived -------------------------------------------------------------------------------- /

    pub fn save_archived(&self) {
        AppLocalStorage::set(ARCHIVED_KEY, self.archived.read().clone())
            .expect("Failed to save archived dogs");
    }

    pub fn set_archived(&mut self, archived: HashSet<Uuid>) {
        self.archived.set(archived);
        self.save_archived();
    }

    pub fn restore_archived(&mut self) {
        let archived = AppLocalStorage::get(ARCHIVED_KEY).unwrap_or(HashSet::new());
        self.archived.set(archived);
    }

    pub fn clear_archived(&mut self) {
        AppLocalStorage::delete(ARCHIVED_KEY);
        log::info!("clear_archived !");
        self.archived.set(HashSet::new());
    }

    // Favorites -------------------------------------------------------------------------------- /

    pub fn save_favorites(&self) {
        AppLocalStorage::set(FAVORITES_KEY, self.favorites.read().clone())
            .expect("Failed to save favorites");
    }

    pub fn set_favorites(&mut self, favorites: HashSet<Uuid>) {
        self.favorites.set(favorites);
        self.save_favorites();
    }

    pub fn clear_favorites(&mut self) {
        AppLocalStorage::delete(FAVORITES_KEY);
        self.favorites.set(HashSet::new());
    }

    pub fn restore_favorites(&mut self) {
        let favorites = AppLocalStorage::get(FAVORITES_KEY).unwrap_or(HashSet::new());
        self.favorites.set(favorites);
    }

    // Combined -------------------------------------------------------------------------------- /

    pub fn save_all(&self) {
        self.save_db();
        self.save_archived();
        self.save_favorites();
    }

    pub fn restore_all(&mut self) {
        self.restore_db();
        self.restore_archived();
        self.restore_favorites();
    }

    pub fn clear_all(&mut self) {
        // self.clear_db(); // TODO: Unimplemented
        self.clear_archived();
        self.clear_favorites();
    }

    // Getters -------------------------------------------------------------------------------- /

    pub fn get_db(&self) -> Arc<Mutex<DogDb>> {
        self.db.clone()
    }

    pub fn get_archived(&self) -> Signal<HashSet<Uuid>> {
        self.archived.clone()
    }

    pub fn get_favorites(&self) -> Signal<HashSet<Uuid>> {
        self.favorites.clone()
    }

    pub fn get(&self) -> Self {
        Self {
            db: self.get_db(),
            archived: self.get_archived(),
            favorites: self.get_favorites(),
        }
    }
}

pub struct AppContextMemory {
    pub archived: HashSet<Uuid>,
    pub favorites: HashSet<Uuid>,
}

impl From<AppContext> for AppContextMemory {
    fn from(ctx: AppContext) -> Self {
        Self {
            archived: ctx.archived.read().clone(),
            favorites: ctx.favorites.read().clone(),
        }
    }
}

impl From<AppContextMemory> for AppContext {
    fn from(memory: AppContextMemory) -> Self {
        Self {
            db: Arc::new(Mutex::new(DogDb::new())),
            archived: Signal::new(memory.archived),
            favorites: Signal::new(memory.favorites),
        }
    }
}
