# NoSQL Database in Rust

## 📌 Overview
This project is a simple, **lightweight NoSQL database** built using Rust. It is designed for learning purposes and explores how NoSQL databases store, retrieve, and manage unstructured data efficiently.

## 🚀 Features
- **Key-Value Store**: Supports basic CRUD operations.
- **Document Storage**: Stores JSON-like structured data.
- **Persistence**: Saves data to disk for long-term storage.
- **Indexing**: Uses basic indexing for faster lookups.
- **Concurrency Support** *(Upcoming)*: Multi-threaded read/write operations.
- **REST API** *(Upcoming)*: Query the database over HTTP.

## 🛠️ Tech Stack
- **Rust** (Primary language)
- **Serde** (JSON serialization/deserialization)
- **Tokio** (Async operations - planned)
- **Actix-web** (For API - planned)

## 🏗️ Project Structure
```
├── src
│   ├── main.rs        # Entry point of the project
│   ├── db.rs          # Core database logic
│   ├── storage.rs     # Persistence layer
│   ├── index.rs       # Indexing mechanism
│   ├── api.rs         # HTTP API (Planned)
│
├── tests              # Unit tests
├── Cargo.toml         # Rust package manager configuration
├── README.md          # This file
```

## 🛠️ Setup & Installation
### Prerequisites
Ensure you have **Rust** and **Cargo** installed. If not, install Rust via:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Clone the Repository
```sh
git clone https://github.com/yourusername/nosql-db.git
cd nosql-db
```

### Build the Project
```sh
cargo build
```

### Run the Database
```sh
cargo run
```

## 📖 Usage
### Insert Data
```rust
let mut db = NoSQLDB::new();
db.insert("user:1", json!({ "name": "Alice", "age": 25 }));
```

### Retrieve Data
```rust
let user = db.get("user:1");
println!("{:?}", user);
```

### Delete Data
```rust
db.delete("user:1");
```

## 📌 Roadmap
- [x] Basic Key-Value Store
- [x] Persistent File Storage
- [x] JSON Document Storage
- [ ] Indexing for Faster Queries
- [ ] REST API (Actix-Web)
- [ ] Multi-threaded Access
- [ ] Replication & Sharding

## 📜 License
This project is **MIT licensed**. Feel free to modify and distribute it as needed!

## 🤝 Contributing
Contributions are welcome! Feel free to submit issues or pull requests.

## 🔗 Contact
For discussions or improvements, reach out on **GitHub Issues**.

---
Happy Coding! 🚀

