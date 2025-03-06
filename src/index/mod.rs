use std::collections::HashMap;
use crate::storage::Storage;

pub struct NoSQLDB {
    data: HashMap<String, String>,
    filename: String, // Persistent storage file
}

impl NoSQLDB {
    /// Initializes the database, loading existing data from a file.
    pub fn new(filename: &str) -> Self {
        let data = Storage::load_from_file(filename);
        NoSQLDB {
            data,
            filename: filename.to_string(),
        }
    }

    /// Inserts a new key-value pair and persists it.
    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key.clone(), value.clone());
        Storage::append_to_file(&self.filename, &key, &value);
    }

    /// Retrieves a value by key.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    /// Deletes a key from memory and saves changes to the file.
    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
        Storage::save_to_file(&self.filename, &self.data); // Rewrites file to reflect deletion
    }
}
