use gloo_storage::{errors::StorageError, LocalStorage, Result, Storage};
use serde::{Deserialize, Serialize};

use crate::constants::APP_KEY;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppLocalStorage {}

impl Storage for AppLocalStorage {
    fn set<T>(key: impl AsRef<str>, value: T) -> Result<()>
    where
        T: Serialize,
    {
        LocalStorage::set(Self::prepend_app_key(key), value)
    }

    fn get<T>(key: impl AsRef<str>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        LocalStorage::get(Self::prepend_app_key(key))
    }

    fn delete(key: impl AsRef<str>) {
        LocalStorage::delete(Self::prepend_app_key(key));
    }

    fn raw() -> web_sys::Storage {
        LocalStorage::raw()
    }

    fn clear() {
        LocalStorage::clear();
    }
}

impl AppLocalStorage {
    pub fn remove<T>(key: impl AsRef<str>) -> Option<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let combined_key = Self::prepend_app_key(key);
        let current = LocalStorage::get::<T>(&combined_key);
        LocalStorage::delete(combined_key);
        match current {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    // Private ------------------------ /

    fn prepend_app_key(key: impl AsRef<str>) -> String {
        format!("{}_{}", APP_KEY, key.as_ref())
    }
}
