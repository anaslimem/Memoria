# Memoria

[![CI](https://github.com/anaslimem/Memoria/actions/workflows/rust.yml/badge.svg)](https://github.com/anaslimem/Memoria/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Memoria** is a fast, safe, and flexible memory storage system built in Rust. Store different types of data (messages, sensor readings, logs), manage storage capacity automatically, and persist your data between sessions—all without crashes or data loss.

Perfect for learning advanced Rust patterns or as a foundation for agent memory systems.

## What Can You Do?

- Store anything - Text, numbers, logs with a single unified interface
- Save and load - Your data persists between sessions (with `--save` flag)
- Safe by default - Type-safe, memory-safe, no panics
- Capacity aware - Automatic storage limit enforcement
- Configurable - Set vault name and size via `.env`
- Interactive CLI - Beautiful, colored terminal interface
- Just works - Comprehensive tests ensure reliability

## Quick Start

### Build and Run
```bash
git clone https://github.com/anaslimem/Memoria.git
cd Memoria
cargo build --release
cargo run
```

### With Data Persistence
```bash
cargo run -- --save
```

This saves your vault to `.memoria/vault.json` on exit and loads it on restart.

### Configuration (.env)
```env
VAULT_NAME="My Memory"
VAULT_CAPACITY_GB=50
```

## Interactive CLI Demo

```
Welcome to Memoria - My Memory!

Available Commands: [add] [exit]
> add
What type? [text] [sensor] [log]
> text
Enter text:
> Remember this important thing
Enter a unique name (key) for this resource:
> important
✓ Successfully added!

Available Commands: [add] [summary] [get] [delete] [exit]
> summary
Vault summary at My Memory:
Text messages: 1
Sensor data: 0
System logs: 0

> exit
✓ Vault saved to .memoria/vault.json
```

## Use as a Library

```toml
[dependencies]
memoria = { git = "https://github.com/anaslimem/Memoria" }
```

```rust
use memoria::{Vault, MemorySize, Resource};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vault = Vault::<String>::new("My Vault".to_string(), MemorySize::MB(100));
    
    vault.add("note".to_string(), Resource::TextMessage("Hello!".to_string()))?;
    vault.add("temp".to_string(), Resource::SensorData(23.5))?;
    vault.add("logs".to_string(), Resource::SystemLogs(vec!["Log 1".to_string()]))?;
    
    if let Some(resource) = vault.get(&"note".to_string()) {
        println!("Found: {:?}", resource);
    }
    
    // Persist to file
    vault.save_to_file("backup.json")?;
    
    Ok(())
}
```

## Core API

### Vault<K>
```rust
Vault::new(location, capacity) -> Vault<K>
vault.add(key, resource) -> Result<()>         // Store a resource
vault.get(&key) -> Option<&Resource>           // Retrieve by key
vault.remove(&key) -> Result<Resource>         // Delete and return
vault.summary()                                 // Display stats
vault.save_to_file(path) -> Result<()>        // Persist to JSON
Vault::load_from_file(path) -> Result<Vault>  // Load from JSON
```

### Resource Types
```rust
Resource::TextMessage(String)           // Any text
Resource::SensorData(f64)               // Numeric data
Resource::SystemLogs(Vec<String>)       // Multiple log entries
```

### Memory Units
```rust
MemorySize::KB(u64)    // Kilobytes
MemorySize::MB(u64)    // Megabytes
MemorySize::GB(u64)    // Gigabytes
```

## Project Structure

```
.
├── src/
│   ├── lib.rs              // Library root
│   ├── main.rs             // Interactive CLI
│   ├── vault.rs            // Core vault logic + persistence
│   ├── resource.rs         // Resource types (serializable)
│   ├── memory.rs           // Memory size helpers (serializable)
│   ├── error.rs            // Error types (serializable)
│   └── ui/mod.rs           // CLI prompt utilities
├── tests/
│   └── integration_cli.rs   // E2E CLI tests
├── Cargo.toml              // Dependencies
├── .env.example            // Config template
└── README.md               // This file
```

## Features

### Storage and Capacity
- Generic Vault<K> works with any hashable key type
- Byte-accurate capacity tracking
- Automatic overflow prevention with helpful errors

### Serialization and Persistence
- All core types are serde-compatible (JSON support)
- Save entire vault to file with save_to_file()
- Load vault from file with load_from_file()
- Auto-save on exit with --save flag

### Error Handling
- Custom VaultError type for all operations
- Colored error output on terminal (red text)
- No panics—all errors are recoverable

### Testing
- 20 tests total (15 unit + 5 integration)
  - Vault operations (add, get, remove, summary)
  - Serialization round-trips
  - File I/O and persistence

## Testing

Run all tests:

```bash
cargo test --all --all-features
```

Run with output:

```bash
cargo test -- --nocapture
```

Run specific test:

```bash
cargo test test_vault_load_from_file -- --nocapture
```

## Linting and Formatting

Check code quality:

```bash
cargo fmt --all -- --check
cargo clippy
```

Auto-format code:

```bash
cargo fmt --all
```

## Design Principles

- Type Safety: Generics prevent runtime type errors
- Ownership and Borrowing: Resources are moved into the vault, ensuring safety
- Capacity Enforcement: Prevents overflow with pre-checks
- Error Propagation: Uses Result for all operations
- Modularity: Clean separation of concerns across modules
- Testability: Comprehensive test coverage for reliability

## Dependencies

- serde (1.0) - Serialization framework
- serde_json (1.0) - JSON support
- colored (2.0) - Terminal colors
- dotenv (0.15) - Environment configuration
- assert_cmd (2.0) - CLI testing
- predicates (2.1) - Test assertions

## Continuous Integration

GitHub Actions runs on every push with:
- Build and test on stable Rust
- Code formatting checks
- Linting with clippy
- Security audit with cargo audit

## Contributing

1. Fork the repository
2. Create a feature branch: git checkout -b feature-name
3. Make changes and add tests
4. Run cargo test and cargo clippy
5. Submit a pull request


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
