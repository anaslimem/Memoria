# Memoria: Typed Resource Manager

**Memoria** is a production-grade, memory-safe resource management system built in Rust. It demonstrates advanced concepts such as ownership, enums with associated data, safe error handling, and automated testing.

## Features

- **Typed Resource Storage**: Securely manage different data types including `TextMessage`, `SensorData`, and `SystemLogs`.
- **Intelligent Capacity Management**: Automatic size estimation and enforcement of storage limits.
- **Dynamic CLI Interface**: An interactive, user-friendly terminal interface that adapts commands based on the system state.
- **Robust Error Handling**: Safe parsing and validation of user input to prevent runtime panics.
- **Verified Stability**: Comprehensive unit test suite ensuring 100% logic reliability.

## Technical Architecture

The system follows a clean modular architecture with separation of concerns:

### Core Components
- **`MemorySize`** (`src/memory.rs`): Enum managing storage units (KB, MB, GB) with byte conversion.
- **`Resource`** (`src/resource.rs`): Core data structure with variant-specific size calculation.
- **`Vault`** (`src/vault.rs`): Central management struct handling ownership transfer and retrieval safety.
- **`ui`** (`src/ui/`): Terminal interaction utilities with input/output handling.

### Security & Safety
- **Ownership Transfer**: Resources are moved into the vault, preventing double-free or use-after-move errors.
- **Reference Safety**: Retrieval methods return `Option<&Resource>`, ensuring memory access is always validated.
- **Bounds Protection**: All indexing operations are protected by explicit range checks.

## Project Structure

The codebase follows a clean modular architecture:

```
src/
├── lib.rs          # Public API and module declarations
├── main.rs         # CLI binary application
├── resource.rs     # Resource enum and implementations
├── memory.rs       # MemorySize enum and implementations
├── vault.rs        # Vault struct and core logic
└── ui/
    └── mod.rs      # User interface utilities
```

### Module Overview

- **`resource`**: Defines the `Resource` enum with variants for different data types and size calculation logic.
- **`memory`**: Provides `MemorySize` enum for storage capacity management with byte conversion.
- **`vault`**: Core `Vault` struct implementing all resource management operations.
- **`ui`**: User interface utilities for terminal interaction.
- **`lib.rs`**: Exports public API (`Vault`, `Resource`, `MemorySize`, `ui`) for library usage.
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

### Using as a Library

Memoria can be used as a library in other Rust projects:

```rust
use memoria::{Vault, MemorySize, Resource};

fn main() {
    let mut vault = Vault::new("My Vault".to_string(), MemorySize::MB(100));
    
    vault.add(Resource::TextMessage("Hello".to_string())).unwrap();
    vault.add(Resource::SensorData(42.5)).unwrap();
    
    vault.summary();
}
```

## Interactive Commands

- `add`: Interactive wizard to create and store new resources.
- `summary`: High-level overview of vault occupancy and resource counts.
- `get`: Safely retrieve and inspect a resource by its index.
- `delete`: Permanently remove a resource from the vault.
- `exit`: Securely close the management session.
