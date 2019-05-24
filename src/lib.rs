#![feature(async_await)]

use parking_lot::RwLock;

use std::collections::HashMap;
use std::sync::Arc;

/// Key-value database.
#[derive(Debug, Clone)]
pub struct Memdb {
    hashmap: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl Memdb {
    /// Create a new instance.
    #[inline]
    pub async fn open() -> Self {
        Self {
            hashmap: Arc::new(RwLock::new(HashMap::<Vec<u8>, Vec<u8>>::new())),
        }
    }

    /// Set a value in the database.
    #[inline]
    pub async fn set(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Option<Vec<u8>> {
        let hashmap = self.hashmap.clone();
        let mut hashmap = hashmap.write();
        hashmap.insert(key.as_ref().to_owned(), value.as_ref().to_owned())
    }

    /// Get a value from the database.
    #[must_use]
    #[inline]
    pub async fn get(&self, key: impl AsRef<[u8]>) -> Option<Vec<u8>> {
        let key = key.as_ref().to_owned();
        let hashmap = &self.hashmap.read();
        hashmap.get(&key).cloned()
    }

    /// Delete a value from the database.
    #[inline]
    pub async fn del(&mut self, key: impl AsRef<[u8]>) -> Option<Vec<u8>> {
        let key = key.as_ref().to_owned();
        let hashmap = &mut self.hashmap.write();
        hashmap.remove(&key)
    }
}
