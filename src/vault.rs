use crate::error::VaultError;
use crate::memory::MemorySize;
use crate::resource::Resource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultMetadata {
    pub location: String,
    pub storage_capacity: MemorySize,
    pub current_usage: u64,
    pub resource_count: usize,
}

pub struct Vault<K>
where
    K: Eq + Hash + std::fmt::Display,
{
    pub location: String,
    pub storage_capacity: MemorySize,
    pub resources: HashMap<K, Resource>,
}

impl<K> Vault<K>
where
    K: Eq + Hash + std::fmt::Display,
{
    pub fn current_usage(&self) -> u64 {
        self.resources.values().map(|res| res.size_bytes()).sum()
    }

    pub fn new(location: String, capacity: MemorySize) -> Self {
        println!("Vault created at {} with capacity {:?}", location, capacity);
        Self {
            location,
            storage_capacity: capacity,
            resources: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: K, resource: Resource) -> Result<(), VaultError> {
        let size = resource.size_bytes();
        let current = self.current_usage();
        let capacity = self.storage_capacity.size_bytes();

        if current + size > capacity {
            return Err(VaultError::VaultFull {
                capacity,
                current,
                new_size: size,
            });
        }

        if self.resources.contains_key(&key) {
            return Err(VaultError::InvalidInput(format!(
                "Key '{}' already exists",
                key
            )));
        }

        self.resources.insert(key, resource);
        Ok(())
    }

    pub fn get(&self, key: &K) -> Option<&Resource> {
        self.resources.get(key)
    }

    pub fn summary(&self) {
        let mut text_count = 0;
        let mut sensor_count = 0;
        let mut log_count = 0;

        for res in self.resources.values() {
            match res {
                Resource::TextMessage(_) => text_count += 1,
                Resource::SensorData(_) => sensor_count += 1,
                Resource::SystemLogs(_) => log_count += 1,
            }
        }
        println!("Vault summary at {}:", self.location);
        println!("Text messages: {}", text_count);
        println!("Sensor data: {}", sensor_count);
        println!("System logs: {}", log_count);
    }

    pub fn remove(&mut self, key: &K) -> Result<Resource, VaultError> {
        match self.resources.remove(key) {
            Some(removed) => {
                Ok(removed)
            }
            None => Err(VaultError::ResourceNotFound(key.to_string())),
        }
    }
}
