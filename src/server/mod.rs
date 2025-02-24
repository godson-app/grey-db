use crate::index::NoSQLDB;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref DB: Mutex<NoSQLDB> = Mutex::new(NoSQLDB::new("database.txt"));
}

pub fn insert(key: String, value: String) {
    let mut db = DB.lock().unwrap();
    db.insert(key, value);
}

pub fn get(key: &str) -> Option<String> {
    let db = DB.lock().unwrap();
    db.get(key).cloned()
}

pub fn delete(key: &str) {
    let mut db = DB.lock().unwrap();
    db.delete(key);
}
