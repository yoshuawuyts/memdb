#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate parking_lot;

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
  pub fn new() -> Self {
    Self {
      hashmap: Arc::new(RwLock::new(HashMap::<K, V>::new())),
    }
  }

  /// Set a value in the database.
  #[inline]
  pub fn set(&mut self, key: K, value: V) -> Option<V> {
    let hashmap = self.hashmap.clone();
    let mut hashmap = hashmap.write();
    hashmap.insert(key, value)
  }

  /// Get a value from the database.
  #[must_use]
  #[inline]
  pub fn get(&self, key: K) -> Option<V> {
    let hashmap = &self.hashmap.read();
    hashmap.get(&key).cloned()
  }

  /// Delete a value from the database.
  #[inline]
  pub fn del(&mut self, key: K) -> Option<V> {
    let hashmap = &mut self.hashmap.write();
    hashmap.remove(&key)
  }
}

impl Default for Memdb<String, String> {
  #[inline]
  fn default() -> Self {
    Self {
      hashmap: Arc::new(RwLock::new(HashMap::<String, String>::new())),
    }
  }
}
