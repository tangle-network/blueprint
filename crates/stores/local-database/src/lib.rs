mod error;
pub use error::Error;

use blueprint_std::collections::HashMap;
use blueprint_std::fs;
use blueprint_std::path::{Path, PathBuf};
use blueprint_std::sync::Mutex;
use serde::{Serialize, de::DeserializeOwned};
use std::io::ErrorKind;

/// A local database for storing key-value pairs.
///
/// The database is stored in a JSON file, which is updated every time
/// a key-value pair is added, updated, or removed. Writes are atomic
/// (write to temporary file, then rename) to prevent corruption.
///
/// # Example
///
/// ```no_run
/// use blueprint_store_local_database::LocalDatabase;
///
/// # fn main() -> Result<(), blueprint_store_local_database::Error> {
/// let db = LocalDatabase::<u64>::open("data.json")?;
///
/// db.set("key", 42)?;
/// assert_eq!(db.get("key")?, Some(42));
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct LocalDatabase<T> {
    path: PathBuf,
    data: Mutex<HashMap<String, T>>,
}

impl<T> LocalDatabase<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    /// Reads a `LocalDatabase` from the given path.
    ///
    /// If the file does not exist, an empty database is created.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use blueprint_store_local_database::LocalDatabase;
    ///
    /// # fn main() -> Result<(), blueprint_store_local_database::Error> {
    /// let db = LocalDatabase::<u64>::open("data.json")?;
    /// assert!(db.is_empty()?);
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// * The parent of `path` is not a directory
    /// * Unable to write to `path`
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();
        let parent_dir = path.parent().ok_or(Error::Io(std::io::Error::new(
            ErrorKind::NotFound,
            "parent directory not found",
        )))?;

        // Create the parent directory if it doesn't exist
        fs::create_dir_all(parent_dir)?;

        let data = if path.exists() {
            let content = fs::read_to_string(path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            // Create an empty file with default empty JSON object
            let empty_data = HashMap::new();
            let json_string = serde_json::to_string(&empty_data)?;
            fs::write(path, json_string)?;
            empty_data
        };

        Ok(Self {
            path: path.to_owned(),
            data: Mutex::new(data),
        })
    }

    /// Returns the number of key-value pairs in the database.
    pub fn len(&self) -> Result<usize, Error> {
        let data = self.lock()?;
        Ok(data.len())
    }

    /// Checks if the database is empty.
    pub fn is_empty(&self) -> Result<bool, Error> {
        let data = self.lock()?;
        Ok(data.is_empty())
    }

    /// Adds or updates a key-value pair in the database.
    ///
    /// # Errors
    ///
    /// * Unable to serialize the data
    /// * Unable to write to disk
    pub fn set(&self, key: &str, value: T) -> Result<(), Error> {
        let mut data = self.lock()?;
        data.insert(key.to_string(), value);
        self.flush(&data)
    }

    /// Retrieves a value associated with the given key.
    pub fn get(&self, key: &str) -> Result<Option<T>, Error> {
        let data = self.lock()?;
        Ok(data.get(key).cloned())
    }

    /// Removes a key-value pair from the database.
    ///
    /// Returns the removed value if the key existed.
    pub fn remove(&self, key: &str) -> Result<Option<T>, Error> {
        let mut data = self.lock()?;
        let removed = data.remove(key);
        if removed.is_some() {
            self.flush(&data)?;
        }
        Ok(removed)
    }

    /// Returns a clone of all values in the database.
    pub fn values(&self) -> Result<Vec<T>, Error> {
        let data = self.lock()?;
        Ok(data.values().cloned().collect())
    }

    /// Returns a clone of all key-value pairs in the database.
    pub fn entries(&self) -> Result<Vec<(String, T)>, Error> {
        let data = self.lock()?;
        Ok(data
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect())
    }

    /// Finds the first value matching a predicate.
    pub fn find<F>(&self, predicate: F) -> Result<Option<T>, Error>
    where
        F: Fn(&T) -> bool,
    {
        let data = self.lock()?;
        Ok(data.values().find(|v| predicate(v)).cloned())
    }

    /// Atomically gets a value by key, applies a mutation, and flushes.
    ///
    /// Returns `true` if the key was found and updated.
    pub fn update<F>(&self, key: &str, f: F) -> Result<bool, Error>
    where
        F: FnOnce(&mut T),
    {
        let mut data = self.lock()?;
        if let Some(value) = data.get_mut(key) {
            f(value);
            self.flush(&data)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Replaces the entire database contents with a new map.
    pub fn replace(&self, new_data: HashMap<String, T>) -> Result<(), Error> {
        let mut data = self.lock()?;
        *data = new_data;
        self.flush(&data)
    }

    /// Checks if a key exists in the database.
    pub fn contains_key(&self, key: &str) -> Result<bool, Error> {
        let data = self.lock()?;
        Ok(data.contains_key(key))
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, HashMap<String, T>>, Error> {
        self.data.lock().map_err(|_| Error::Poisoned)
    }

    /// Atomically write to a temporary file and rename to the target path.
    fn flush(&self, data: &HashMap<String, T>) -> Result<(), Error> {
        let tmp = self.path.with_extension("tmp");
        let json_string = serde_json::to_string(data)?;
        fs::write(&tmp, &json_string)?;
        fs::rename(&tmp, &self.path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use tempfile::tempdir;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    struct TestStruct {
        field1: String,
        field2: i32,
    }

    #[test]
    fn test_create_new_database() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.json");

        let db = LocalDatabase::<u32>::open(&db_path).unwrap();
        assert!(db.is_empty().unwrap());
        assert_eq!(db.len().unwrap(), 0);
        assert!(db_path.exists());
    }

    #[test]
    fn test_set_and_get() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.json");

        let db = LocalDatabase::<u32>::open(&db_path).unwrap();
        db.set("key1", 42).unwrap();
        db.set("key2", 100).unwrap();

        assert_eq!(db.get("key1").unwrap(), Some(42));
        assert_eq!(db.get("key2").unwrap(), Some(100));
        assert_eq!(db.get("nonexistent").unwrap(), None);
        assert_eq!(db.len().unwrap(), 2);
    }

    #[test]
    fn test_complex_type() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.json");

        let db = LocalDatabase::<TestStruct>::open(&db_path).unwrap();

        let test_struct = TestStruct {
            field1: "test".to_string(),
            field2: 42,
        };

        db.set("key1", test_struct.clone()).unwrap();
        assert_eq!(db.get("key1").unwrap(), Some(test_struct));
    }

    #[test]
    fn test_persistence() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.json");

        // Write data
        {
            let db = LocalDatabase::<u32>::open(&db_path).unwrap();
            db.set("key1", 42).unwrap();
            db.set("key2", 100).unwrap();
        }

        // Read data in new instance
        {
            let db = LocalDatabase::<u32>::open(&db_path).unwrap();
            assert_eq!(db.get("key1").unwrap(), Some(42));
            assert_eq!(db.get("key2").unwrap(), Some(100));
            assert_eq!(db.len().unwrap(), 2);
        }
    }

    #[test]
    fn test_overwrite() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.json");

        let db = LocalDatabase::<u32>::open(&db_path).unwrap();
        db.set("key1", 42).unwrap();
        assert_eq!(db.get("key1").unwrap(), Some(42));

        db.set("key1", 100).unwrap();
        assert_eq!(db.get("key1").unwrap(), Some(100));
        assert_eq!(db.len().unwrap(), 1);
    }

    #[test]
    fn test_invalid_json() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.json");

        // Write invalid JSON
        fs::write(&db_path, "{invalid_json}").unwrap();

        // Should create empty database when JSON is invalid
        let db = LocalDatabase::<u32>::open(&db_path).unwrap();
        assert!(db.is_empty().unwrap());
    }

    #[test]
    fn test_remove() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.json");

        let db = LocalDatabase::<u32>::open(&db_path).unwrap();
        db.set("key1", 42).unwrap();
        db.set("key2", 100).unwrap();

        let removed = db.remove("key1").unwrap();
        assert_eq!(removed, Some(42));
        assert_eq!(db.get("key1").unwrap(), None);
        assert_eq!(db.len().unwrap(), 1);

        // Verify persistence after remove
        let db2 = LocalDatabase::<u32>::open(&db_path).unwrap();
        assert_eq!(db2.get("key1").unwrap(), None);
        assert_eq!(db2.get("key2").unwrap(), Some(100));
    }

    #[test]
    fn test_remove_nonexistent() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.json");

        let db = LocalDatabase::<u32>::open(&db_path).unwrap();
        let removed = db.remove("nonexistent").unwrap();
        assert_eq!(removed, None);
    }

    #[test]
    fn test_values() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.json");

        let db = LocalDatabase::<u32>::open(&db_path).unwrap();
        db.set("a", 1).unwrap();
        db.set("b", 2).unwrap();
        db.set("c", 3).unwrap();

        let mut values = db.values().unwrap();
        values.sort();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_find() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.json");

        let db = LocalDatabase::<TestStruct>::open(&db_path).unwrap();
        db.set(
            "a",
            TestStruct {
                field1: "hello".into(),
                field2: 10,
            },
        )
        .unwrap();
        db.set(
            "b",
            TestStruct {
                field1: "world".into(),
                field2: 20,
            },
        )
        .unwrap();

        let found = db.find(|v| v.field2 == 20).unwrap();
        assert_eq!(found.unwrap().field1, "world");

        let not_found = db.find(|v| v.field2 == 99).unwrap();
        assert!(not_found.is_none());
    }

    #[test]
    fn test_update() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.json");

        let db = LocalDatabase::<TestStruct>::open(&db_path).unwrap();
        db.set(
            "key1",
            TestStruct {
                field1: "original".into(),
                field2: 0,
            },
        )
        .unwrap();

        let updated = db
            .update("key1", |v| {
                v.field1 = "modified".into();
                v.field2 = 42;
            })
            .unwrap();
        assert!(updated);

        let value = db.get("key1").unwrap().unwrap();
        assert_eq!(value.field1, "modified");
        assert_eq!(value.field2, 42);

        // Verify persistence
        let db2 = LocalDatabase::<TestStruct>::open(&db_path).unwrap();
        let value = db2.get("key1").unwrap().unwrap();
        assert_eq!(value.field1, "modified");
    }

    #[test]
    fn test_update_nonexistent() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.json");

        let db = LocalDatabase::<u32>::open(&db_path).unwrap();
        let updated = db.update("nonexistent", |v| *v += 1).unwrap();
        assert!(!updated);
    }

    #[test]
    fn test_replace() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.json");

        let db = LocalDatabase::<u32>::open(&db_path).unwrap();
        db.set("old", 1).unwrap();

        let mut new_data = HashMap::new();
        new_data.insert("new1".to_string(), 10);
        new_data.insert("new2".to_string(), 20);
        db.replace(new_data).unwrap();

        assert_eq!(db.get("old").unwrap(), None);
        assert_eq!(db.get("new1").unwrap(), Some(10));
        assert_eq!(db.get("new2").unwrap(), Some(20));

        // Verify persistence
        let db2 = LocalDatabase::<u32>::open(&db_path).unwrap();
        assert_eq!(db2.get("new1").unwrap(), Some(10));
    }

    #[test]
    fn test_contains_key() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.json");

        let db = LocalDatabase::<u32>::open(&db_path).unwrap();
        db.set("exists", 42).unwrap();

        assert!(db.contains_key("exists").unwrap());
        assert!(!db.contains_key("missing").unwrap());
    }

    #[test]
    fn test_entries() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.json");

        let db = LocalDatabase::<u32>::open(&db_path).unwrap();
        db.set("a", 1).unwrap();
        db.set("b", 2).unwrap();

        let mut entries = db.entries().unwrap();
        entries.sort_by_key(|(k, _)| k.clone());
        assert_eq!(
            entries,
            vec![("a".to_string(), 1), ("b".to_string(), 2)]
        );
    }

    #[test]
    fn test_concurrent_access() {
        use blueprint_std::sync::Arc;
        use blueprint_std::thread;

        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.json");

        let db = Arc::new(LocalDatabase::<u32>::open(&db_path).unwrap());
        let mut handles = vec![];

        // Spawn multiple threads to write to the database
        for i in 0..10 {
            let db_clone = Arc::clone(&db);
            let handle = thread::spawn(move || {
                db_clone.set(&format!("key{}", i), i).unwrap();
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(db.len().unwrap(), 10);
        for i in 0..10 {
            assert_eq!(db.get(&format!("key{}", i)).unwrap(), Some(i));
        }
    }
}
