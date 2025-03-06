use crate::index::NoSQLDB;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref DB: Mutex<NoSQLDB> = Mutex::new(NoSQLDB::new("database.txt"));
}

pub fn insert(key: String, value: String) -> Result<(), String> {
    let mut db = DB.lock().map_err(|_| "Database lock failed")?;
    db.insert(key, value);
    Ok(())
}

pub fn get(key: &str) -> Result<Option<String>, String> {
    let db = DB.lock().map_err(|_| "Database lock failed")?;
    Ok(db.get(key).cloned())
}

pub fn delete(key: &str) -> Result<(), String> {
    let mut db = DB.lock().map_err(|_| "Database lock failed")?;
    db.delete(key);
    Ok(())
}
