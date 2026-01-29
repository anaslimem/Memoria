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
        match self {
            VaultError::VaultFull {
                capacity,
                current,
                new_size,
            } => write!(
                f,
                "Vault full! Capacity: {} bytes, Current: {} bytes, New Resource: {} bytes",
                capacity.to_string().red(),
                current.to_string().yellow(),
                new_size.to_string().red()
            ),
            VaultError::ResourceNotFound(key) => write!(f, "Resource '{}' not found", key),
            VaultError::InvalidInput(msg) => write!(f, "Input error: {}", msg),
        }
    }
}

pub fn print_error(err: &VaultError) {
    eprintln!("{} {}", "Error:".red().bold(), err);
}
