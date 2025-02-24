use std::fs::{File};
use std::io::{Write, Read};
use std::collections::HashMap;

pub struct Storage;

impl Storage {
    pub fn save_to_file(filename: &str, data: &HashMap<String, String>) {
        let mut file = File::create(filename).expect("Failed to create file");
        for (key, value) in data {
            writeln!(file, "{}:{}", key, value).expect("Failed to write to file");
        }
    }

    pub fn load_from_file(filename: &str) -> HashMap<String, String> {
        let mut file = File::open(filename).unwrap_or_else(|_| File::create(filename).expect("Failed to create file"));
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");

        let mut data = HashMap::new();
        for line in contents.lines() {
            if let Some((key, value)) = line.split_once(':') {
                data.insert(key.to_string(), value.to_string());
            }
        }
        data
    }
}