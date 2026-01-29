use colored::*;
use std::fmt; // For colored terminal output

#[derive(Debug)]
pub enum VaultError {
    VaultFull {
        capacity: u64,
        current: u64,
        new_size: u64,
    },
    ResourceNotFound(String),
    InvalidInput(String),
}

impl fmt::Display for VaultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            VaultError::VaultFull {
                capacity,
                current,
                new_size,
            } => format!(
                "Vault full! Capacity: {} bytes, Current: {} bytes, New Resource: {} bytes",
                capacity, current, new_size
            ),
            VaultError::ResourceNotFound(key) => format!("Resource '{}' not found", key),
            VaultError::InvalidInput(msg) => format!("Input error: {}", msg),
        };
        write!(f, "{}", msg.red())
    }
}

pub fn print_error(err: &VaultError) {
    eprintln!("{} {}", "Error:".red().bold(), err);
}
