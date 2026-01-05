# Memoria: Typed Resource Manager

**Memoria** is a production-grade, memory-safe resource management system built in Rust. It demonstrates advanced concepts such as ownership, enums with associated data, safe error handling, and automated testing.

## Features

- **Typed Resource Storage**: Securely manage different data types including `TextMessage`, `SensorData`, and `SystemLogs`.
- **Intelligent Capacity Management**: Automatic size estimation and enforcement of storage limits.
- **Dynamic CLI Interface**: An interactive, user-friendly terminal interface that adapts commands based on the system state.
- **Robust Error Handling**: Safe parsing and validation of user input to prevent runtime panics.
- **Verified Stability**: Comprehensive unit test suite ensuring 100% logic reliability.

## Technical Architecture

### Core Components
- **`MemorySize`**: Enum managing storage units (KB, MB, GB) with byte conversion.
- **`Resource`**: Core data structure with variant-specific size calculation.
- **`Vault`**: Central management struct handling ownership transfer and retrieval safety.

### Security & Safety
- **Ownership Transfer**: Resources are moved into the vault, preventing double-free or use-after-move errors.
- **Reference Safety**: Retrieval methods return `Option<&Resource>`, ensuring memory access is always validated.
- **Bounds Protection**: All indexing operations are protected by explicit range checks.

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

## Interactive Commands

- `add`: Interactive wizard to create and store new resources.
- `summary`: High-level overview of vault occupancy and resource counts.
- `get`: Safely retrieve and inspect a resource by its index.
- `delete`: Permanently remove a resource from the vault.
- `exit`: Securely close the management session.
