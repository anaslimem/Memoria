use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Resource {
    TextMessage(String),
    SensorData(f64),
    SystemLogs(Vec<String>),
}

impl Resource {
    // Returns the estimated size of the resource in bytes
    pub fn size_bytes(&self) -> u64 {
        match self {
            Resource::TextMessage(s) => s.len() as u64,
            Resource::SensorData(_) => 8, // f64 is 8 bytes
            Resource::SystemLogs(logs) => logs.iter().map(|log| log.len() as u64).sum(),
        }
    }
}
