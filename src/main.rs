use std::collections::HashMap;

struct Database {
    storage: HashMap<String, String>,
}

impl Database {
    fn new() -> Self {
        Database {
            storage: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: String) {
        self.storage.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.storage.get(key)
    }

    fn delete(&mut self, key: &str) -> Option<String> {
        self.storage.remove(key)
    }
}

fn main() {
    let mut db = Database::new();

    db.insert("name".to_string(), "Godson".to_string());
    db.insert("age".to_string(), "20".to_string());
    db.insert("country".to_string(), "Nigeria".to_string());
    db.insert("city".to_string(), "Lagos".to_string());

    if let Some(name) = db.get("name") {
        println!("Name: {}", name);
    } else {
        println!("Name not found");
    }

    db.delete("city");

    if let Some(city) = db.get("city") {
        println!("City: {}", city);
    } else {
        println!("City not found");
    }
}
