use util::error::Error;

/// IndexTrait is for non-unique indexes. Multiple values can be associated with a single key.
/// - Do not use this trait for unique indexes. Use UniqueIndexTrait instead.
pub trait IndexTrait<K, V> {
    /// Get values associated with the given key.
    fn get(&self, key: K) -> Vec<V>;
    /// Check if value exists against the given key.
    fn exists(&self, key: K, value: V) -> bool;

    /// Add new value to the index against the given key.
    /// Returns tuple for syncing.
    fn insert(&mut self, key: K, value: V) -> Result<Vec<u8>, Error>;

    /// Remove value from the index against the given key.
    /// Returns tuple for syncing.
    fn remove(&mut self, key: K, value: V) -> Result<Vec<u8>, Error>;

    /// Delete the key from the index.
    /// Returns tuple for syncing.
    fn delete(&mut self, key: K) -> Result<Vec<u8>, Error>;
}

/// UniqueIndexTrait is for unique indexes. Only one value can be associated with a single key.
/// - Do not use this trait for non-unique indexes. Use IndexTrait instead.
pub trait UniqueIndexTrait<K, V> {
    /// Get values associated with the given key.
    fn get(&self, key: K) -> Option<V>;
    /// Check if value exists against the given key.
    fn exists(&self, key: K, value: V) -> bool;

    /// Set unique value for the given key.
    /// - if overwrite: true - overwrites existing value if any.
    /// - else: returns error if value already exists.
    /// Returns tuple for syncing.
    fn set(&mut self, key: K, value: V, overwrite: bool) -> Result<Vec<u8>, Error>;

    /// Delete the key from the index.
    /// Returns tuple for syncing.
    fn delete(&mut self, key: K) -> Result<Vec<u8>, Error>;
}

/// Trait to serialize index to bytes and deserialize bytes to index.
pub trait IndexSerializationTrait<S, I> {
    /// Parse bytes and produce a new index.
    fn from_bytes(bytes: &[u8]) -> Result<S, Error>;

    /// Serialize the index into bytes.
    fn to_bytes(&self) -> Vec<u8>;
}

/// Untility trait for index. Useful for tests.
pub trait IndexCloneTrait<S, I> {
    /// Clone the index.
    fn clone(&self) -> S;

    /// Clone the index map.
    /// - Clone BTreeMap for BTreeIndex.
    /// - Clone HashMap for HashMapIndex.
    fn index_clone(&self) -> I;
}
