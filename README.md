# Memoria: Typed Resource Manager

[![CI](https://github.com/anaslimem/Memoria/actions/workflows/rust.yml/badge.svg)](https://github.com/anaslimem/Memoria/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Memoria** is a production-grade, memory-safe resource management system built in Rust. It provides a generic, type-safe vault for storing and retrieving resources with automatic capacity management, robust error handling, configurable settings via environment variables, and an interactive CLI interface with colored error output. Designed to demonstrate advanced Rust concepts like ownership, generics, safe concurrency patterns, and I/O operations.

## Features

- **Generic Key Storage**: Flexible `Vault<K>` supporting any `Hashable` key type (e.g., `String`, `UUID`, integers).
- **Typed Resource Variants**: Store `TextMessage`, `SensorData`, and `SystemLogs` with automatic size calculation.
- **Capacity Management**: Enforce storage limits with byte-accurate tracking and overflow prevention.
- **Robust Error Handling**: Custom `VaultError` types with descriptive, colored messages on stderr, ensuring no panics.
- **Environment Configuration**: Configurable vault name and capacity via `.env` file for easy customization.
- **Interactive CLI**: User-friendly terminal interface with dynamic command availability and realistic error display.
- **Comprehensive Testing**: Unit tests for core logic and extensive integration tests for CLI workflows.
- **Memory Safety**: Leverages Rust's ownership system to prevent data races and invalid access.

## Installation

### Prerequisites
- [Rust & Cargo](https://www.rust-lang.org/tools/install) (version 1.70+ recommended)

### Build from Source
```bash
# Clone the repository
git clone https://github.com/anaslimem/Memoria.git
cd Memoria

# Build the project
cargo build --release

# Run tests
cargo test
```

### Configuration
Create a `.env` file in the project root to customize settings:

```bash
# Configuration for Memoria vault
VAULT_NAME="Global Vault"
VAULT_CAPACITY_GB=50
```

- `VAULT_NAME`: Display name for the vault (default: "Default Vault").
- `VAULT_CAPACITY_GB`: Storage capacity in GB (default: 50).

## Usage

### Command-Line Interface (CLI)

The CLI provides an interactive way to manage resources with configuration loaded from `.env`:

```bash
cargo run
```

**Available Commands:**
- `add`: Create and store a new resource (prompts for type: text, sensor, or log, then key).
- `summary`: Display vault statistics (counts by resource type).
- `get`: Retrieve and display a resource by key.
- `delete`: Remove a resource by key.
- `exit`: Quit the application.

Errors are displayed in red on stderr for better visibility.

Example session:
```
Vault created at Global Vault with capacity GB(50)
Welcome to Memoria - Global Vault!

Available Commands: [add] [exit]
> add
What type? [text] [sensor] [log]
> text
Enter text:
> Hello, World!
Enter a unique name (key) for this resource:
> greeting
Successfully added!

Available Commands: [add] [summary] [get] [delete] [exit]
> summary
Vault summary at Global Vault:
Text messages: 1
Sensor data: 0
System logs: 0

> get
Enter name (key) of resource:
> nonexistent
Error: Resource 'nonexistent' not found

> exit
```

### Using as a Library

Add Memoria to your `Cargo.toml`:

```toml
[dependencies]
memoria = { git = "https://github.com/anaslimem/Memoria" }
```

Basic usage:

```rust
use memoria::{Vault, MemorySize, Resource};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a vault with String keys and 100MB capacity
    let mut vault = Vault::<String>::new("My Vault".to_string(), MemorySize::MB(100));

    // Add resources
    vault.add("msg".to_string(), Resource::TextMessage("Hello!".to_string()))?;
    vault.add("temp".to_string(), Resource::SensorData(23.5))?;
    vault.add("logs".to_string(), Resource::SystemLogs(vec!["Started".to_string(), "Running".to_string()]))?;

    // Retrieve a resource
    if let Some(resource) = vault.get(&"msg".to_string()) {
        println!("Retrieved: {:?}", resource);
    }

    // Get summary
    vault.summary();

    Ok(())
}
```

## API Reference

### Core Types

- **`Vault<K>`**: Generic vault struct where `K: Eq + Hash + Display`.
  - `new(location: String, capacity: MemorySize) -> Vault<K>`
  - `add(&mut self, key: K, resource: Resource) -> Result<(), VaultError>`
  - `get(&self, key: &K) -> Option<&Resource>`
  - `remove(&mut self, key: &K) -> Result<Resource, VaultError>`
  - `summary(&self)`: Prints resource counts.
  - `current_usage(&self) -> u64`: Returns bytes used.

- **`Resource`**: Enum for resource types.
  - `TextMessage(String)`
  - `SensorData(f64)`
  - `SystemLogs(Vec<String>)`
  - `size_bytes(&self) -> u64`: Estimates memory usage.

- **`MemorySize`**: Enum for capacity units.
  - `KB(u64)`, `MB(u64)`, `GB(u64)`
  - `size_bytes(&self) -> u64`: Converts to bytes.

- **`VaultError`**: Custom error enum.
  - `VaultFull { capacity, current, new_size }`
  - `ResourceNotFound(String)`
  - `InvalidInput(String)`
  - `print_error(err: &VaultError)`: Prints colored error to stderr.

### CLI Utilities

- **`ui::prompt(message: &str) -> io::Result<String>`**: Reads user input from stdin.

## Project Structure

```
.
├── .env                    # Environment configuration (optional)
├── .env.example            # Example configuration file
├── Cargo.toml              # Project metadata and dependencies
├── LICENSE                 # MIT License
├── README.md               # This file
├── src/
│   ├── lib.rs              # Library entry point and re-exports
│   ├── main.rs             # CLI application with .env loading
│   ├── resource.rs         # Resource enum and size calculation
│   ├── memory.rs           # MemorySize enum and conversions
│   ├── vault.rs            # Vault struct and core operations
│   ├── error.rs            # Custom error types with colored output
│   └── ui/
│       └── mod.rs          # CLI input utilities
├── tests/
│   └── integration_cli.rs  # End-to-end CLI tests (5 tests)
└── .github/
    └── workflows/
        └── rust.yml        # GitHub Actions CI
```

## Technical Architecture

### Core Components
- **`MemorySize`** (`src/memory.rs`): Handles storage units with byte conversion.
- **`Resource`** (`src/resource.rs`): Defines resource variants with size estimation.
- **`Vault<K>`** (`src/vault.rs`): Generic container using `HashMap` for storage.
- **`VaultError`** (`src/error.rs`): Comprehensive error handling with colored terminal output.
- **`ui`** (`src/ui/`): Safe input handling for CLI.
- **`.env` Configuration**: Environment-based settings loaded via `dotenv` crate.
- **`tests`** (`src/lib.rs`, `tests/integration_cli.rs`): Ensures reliability with unit and integration coverage.

### Design Principles
- **Type Safety**: Generics prevent runtime type errors.
- **Ownership & Borrowing**: Resources are moved into the vault, ensuring safety.
- **Capacity Enforcement**: Prevents overflow with pre-checks.
- **Error Propagation**: Uses `Result` for all operations, with colored stderr output for CLI.
- **Configuration**: External `.env` file for easy customization without recompilation.
- **Modularity**: Clean separation of concerns across modules.

## Testing

Run the full test suite:

```bash
cargo test --all --all-features
```

- **Unit Tests** (10 tests in `src/lib.rs`): Cover vault operations, size calculations, and error cases.
- **Integration Tests** (5 tests in `tests/integration_cli.rs`): Validates CLI workflows, including add/get, summary, delete, invalid commands, and duplicate keys.

All tests pass, ensuring code quality and preventing regressions.

## Continuous Integration

GitHub Actions runs on every push/PR with comprehensive checks:
- **Test Job**: Builds with stable Rust and executes all tests (15 total: 10 unit + 5 integration).
- **Lint Job**: Checks code formatting with `rustfmt` and runs `clippy` for linting.
- **Audit Job**: Scans dependencies for security vulnerabilities using `cargo audit`.
- Ensures cross-platform compatibility (Ubuntu) and prevents regressions.

## Contributing

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature-name`.
3. Make changes and add tests.
4. Run `cargo test` and `cargo clippy`.
5. Submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
