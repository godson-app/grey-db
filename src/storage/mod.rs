use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader};
use std::collections::HashMap;

pub struct Storage;

impl Storage {
    /// Appends a new key-value pair to the file instead of overwriting it.
    pub fn append_to_file(filename: &str, key: &str, value: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .expect("Failed to open file");

        writeln!(file, "{}={}", key, value).expect("Failed to write to file");
    }

    /// Saves the entire HashMap to a file (for full persistence).
    pub fn save_to_file(filename: &str, data: &HashMap<String, String>) {
        let mut file = File::create(filename).expect("Failed to create file");
        for (key, value) in data {
            writeln!(file, "{}={}", key, value).expect("Failed to write to file");
        }
    }

    /// Loads key-value pairs from a file into a HashMap.
    pub fn load_from_file(filename: &str) -> HashMap<String, String> {
        let file = File::open(filename);

        let mut data = HashMap::new();

        if let Ok(file) = file {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Some((key, value)) = line.split_once('=') {
                        data.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }
        data
    }
}
