use std::collections::HashMap;
use crate::storage::Storage;

pub struct NoSQLDB {
    data: HashMap<String, String>,
    filename: String, // Persistent storage file
}

impl NoSQLDB {
    pub fn new(filename: &str) -> Self {
        let data = Storage::load_from_file(filename);
        NoSQLDB {
            data,
            filename: filename.to_string(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key.clone(), value.clone());
        Storage::save_to_file(&self.filename, &self.data);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
        Storage::save_to_file(&self.filename, &self.data);
    }
}
