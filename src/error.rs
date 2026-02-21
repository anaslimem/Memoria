use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt; // For colored terminal output

#[derive(Debug, Serialize, Deserialize)]
pub enum VaultError {
    VaultFull {
        capacity: u64,
        current: u64,
        new_size: u64,
    },
    ResourceNotFound(String),
    InvalidInput(String),
    #[serde(skip)]
    IoError(std::io::Error),
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
            VaultError::IoError(e) => format!("I/O error: {}", e),
        };
        write!(f, "{}", msg.red())
    }
}

impl std::error::Error for VaultError {}

impl From<std::io::Error> for VaultError {
    fn from(err: std::io::Error) -> Self {
        VaultError::IoError(err)
    }
}
