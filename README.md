# Memoria: Typed Resource Manager

[![CI](https://github.com/anaslimem/Memoria/actions/workflows/rust.yml/badge.svg)](https://github.com/anaslimem/Memoria/actions/workflows/rust.yml)

**Memoria** is a production-grade, memory-safe resource management system built in Rust. It demonstrates advanced concepts such as ownership, enums with associated data, safe error handling, and automated testing.

## Features

- **Generic Key Storage**: flexible `Vault<K>` architecture supporting any Hashable key type (String, UUID, Integers).
- **Key-Value Resource Storage**: Securely manage resources using **HashMaps**, allowing fast retrieval by unique names.
- **Robust Error Handling**: Custom `VaultError` types and fail-safe I/O handling ensuring the system never panics unexpectedly.
- **Typed Resource Support**: Handle `TextMessage`, `SensorData`, and `SystemLogs` uniformly.
- **Intelligent Capacity Management**: Automatic size estimation and enforcement of storage limits.
- **Dynamic CLI Interface**: An interactive, user-friendly terminal interface that adapts commands based on the system state.

## Technical Architecture

The system follows a clean modular architecture with separation of concerns:

### Core Components
- **`MemorySize`** (`src/memory.rs`): Enum managing storage units (KB, MB, GB) with byte conversion.
- **`Resource`** (`src/resource.rs`): Core data structure with variant-specific size calculation.
- **`Vault<K>`** (`src/vault.rs`): Generic management struct handling ownership transfer using `HashMap<K, Resource>`.
- **`VaultError`** (`src/error.rs`): Custom error types implementing `std::error::Error` for descriptive failure modes.
- **`ui`** (`src/ui/`): Terminal interaction utilities with safe `Result`-based input handling.

### Security & Safety
- **Generics & Trait Bounds**: The core Vault utilizes `where K: Eq + Hash + Display` to enforce type safety at compile time.
- **Ownership Transfer**: Resources are moved into the vault, preventing double-free or use-after-move errors.
- **Reference Safety**: Retrieval methods return `Option<&Resource>`, ensuring memory access is always validated.
- **Key Uniqueness**: HashMaps ensure that every resource has a unique identifier, preventing duplicate overwrites without checks.

## Project Structure

The codebase follows a clean modular architecture:

```
src/
├── lib.rs          # Public API and module declarations
├── main.rs         # CLI binary application
├── resource.rs     # Resource enum and implementations
├── memory.rs       # MemorySize enum and implementations
├── vault.rs        # Vault struct and core logic
├── error.rs        # Custom VaultError definitions
└── ui/
    └── mod.rs      # User interface utilities
```

### Module Overview

- **`resource`**: Defines the `Resource` enum with variants for different data types and size calculation logic.
- **`memory`**: Provides `MemorySize` enum for storage capacity management with byte conversion.
- **`vault`**: Core `Vault<K>` struct implementing generic resource management operations.
- **`error`**: Defines `VaultError` for handling specific error conditions (Full, NotFound, InvalidInput).
- **`ui`**: User interface utilities for terminal interaction.
- **`lib.rs`**: Exports public API (`Vault`, `Resource`, `MemorySize`, `VaultError`, `ui`) for library usage.
- **`main.rs`**: Interactive CLI application using the memoria library.

## Installation & Usage

### Prerequisites
- [Rust & Cargo](https://www.rust-lang.org/tools/install) (latest stable)

### Build and Run
```bash
# Clone the repository
git clone https://github.com/anaslimem/Memoria.git
cd Memoria

# Run the interactive CLI
cargo run
```

### Running Tests
```bash
cargo test
```

This runs both unit tests (defined in `src/lib.rs`) and integration tests (in `tests/integration_cli.rs`), ensuring the library and CLI functionality work correctly. Tests are automatically run in CI on every push and pull request.

### Continuous Integration
This project uses GitHub Actions for CI. The workflow (`.github/workflows/rust.yml`) builds and tests the code on Ubuntu with the latest stable Rust toolchain. CI ensures code quality and prevents regressions.

### Using as a Library

Memoria can be used as a library in other Rust projects. Notice the use of **Turbofish syntax** to specify the key type!

```rust
use memoria::{Vault, MemorySize, Resource};

fn main() {
    // Explicitly create a Vault using String keys
    let mut vault = Vault::<String>::new("My Vault".to_string(), MemorySize::MB(100));
    
    // Add resources with a unique key
    vault.add("greeting".to_string(), Resource::TextMessage("Hello".to_string())).unwrap();
    vault.add("sensor_1".to_string(), Resource::SensorData(42.5)).unwrap();
    
    vault.summary();
}
```

## Interactive Commands

- `add`: Interactive wizard to create and store new resources. You will be asked to provide a unique **name (key)**.
- `summary`: High-level overview of vault occupancy and resource counts.
- `get`: Safely retrieve and inspect a resource by its **name (key)**.
- `delete`: Permanently remove a resource by its **name (key)**.
- `exit`: Securely close the management session.
