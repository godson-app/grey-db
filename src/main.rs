mod index;
mod storage;

use index::NoSQLDB;

fn main() {
    let mut db = NoSQLDB::new("database.txt");

    db.insert("user:1".to_string(), "{ \"name\": \"Alice\", \"age\": \"25\" }".to_string());

    match db.get("user:1") {
        Some(data) => println!("Found: {}", data),
        None => println!("Not found"),
    }

    db.delete("user:1");

    match db.get("user:1") {
        Some(data) => println!("Found: {}", data),
        None => println!("Not found"),
    }
}
