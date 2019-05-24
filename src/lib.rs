#![feature(async_await)]

use parking_lot::RwLock;

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

/// Key-value database.
#[derive(Debug, Clone)]
pub struct Memdb<K: Hash + Eq, V> {
    hashmap: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> Memdb<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    /// Create a new instance.
    #[inline]
    pub async fn open() -> Self {
        Self {
            hashmap: Arc::new(RwLock::new(HashMap::<K, V>::new())),
        }
    }

    /// Set a value in the database.
    #[inline]
    pub async fn set(&mut self, key: K, value: V) -> Option<V> {
        let hashmap = self.hashmap.clone();
        let mut hashmap = hashmap.write();
        hashmap.insert(key, value)
    }

    /// Get a value from the database.
    #[must_use]
    #[inline]
    pub async fn get(&self, key: K) -> Option<V> {
        let hashmap = &self.hashmap.read();
        hashmap.get(&key).cloned()
    }

    /// Delete a value from the database.
    #[inline]
    pub async fn del(&mut self, key: K) -> Option<V> {
        let hashmap = &mut self.hashmap.write();
        hashmap.remove(&key)
    }
}
