# 🦀 HealthTrack Rust

A medical patient management system written in Rust, featuring appointment scheduling, prescription management, and patient records.

> 🚧 **Work in Progress** - This is a learning project to master Rust while building a practical healthcare application.

## 📋 Features

- ✅ Patient management (CRUD operations)
- 🔄 Appointment scheduling (in progress)
- 💊 Prescription tracking (planned)
- 🔍 Advanced patient search (planned)
- 📊 Medical records management (planned)

## 🛠️ Tech Stack

- **Language:** Rust 1.92.0
- **Database:** SQLite (via rusqlite)
- **Date handling:** chrono
- **Error handling:** anyhow

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Cargo (comes with Rust)

### Installation
```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/healthtrack-rs.git
cd healthtrack-rs

# Build the project
cargo build

# Run the application
cargo run
```

### Development
```bash
# Check code without compiling (fast)
cargo check

# Run with clippy lints
cargo clippy

# Format code
cargo fmt

# Run tests
cargo test
```

## 📁 Project Structure
```
healthtrack-rs/
├── src/
│   ├── main.rs          # Entry point
│   ├── models/          # Data models (Patient, Appointment, etc.)
│   │   ├── mod.rs
│   │   └── patient.rs
│   ├── db/              # Database operations
│   │   └── mod.rs
│   └── ui/              # User interface
│       └── mod.rs
├── Cargo.toml           # Dependencies
└── welcome.txt          # Startup message
```

## 🎯 Roadmap

- [x] Project setup
- [x] Database schema design
- [ ] Patient CRUD operations
- [ ] Appointment scheduling
- [ ] Prescription management
- [ ] Search functionality
- [ ] Data validation
- [ ] Unit tests
- [ ] Documentation

## 📚 Learning Resources

This project is part of my Rust learning journey. Key concepts explored:

- Ownership and borrowing
- Error handling with `Result<T, E>`
- Type safety with enums and structs
- Database integration
- Module system organization

## 🤝 Contributing

This is a personal learning project, but suggestions and feedback are welcome!

## 📄 License

MIT License - feel free to use this for learning purposes.

## 🔗 Related Projects

- [HealthTrack Python](https://github.com/YOUR_USERNAME/HealtTrack) - Original Python version

---

**Author:** Loïc Pilette  
**Status:** Active Development  
**Last Updated:** January 2026
