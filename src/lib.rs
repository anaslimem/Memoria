mod resource;  // Declare the resource module
pub use resource::Resource;  // Import Resource enum from resource module
mod memory;  
pub use memory::MemorySize;  

pub struct Vault{
    pub location: String,
    pub storage_capacity: MemorySize,
    pub resources: Vec<Resource>,
}

impl Vault {
    pub fn current_usage(&self) -> u64 {
        self.resources.iter().map(|res| res.size_bytes()).sum()
    }
    pub fn new(location: String, capacity: MemorySize) -> Self{
        println!("Vault created at {} with capacity {:?}", location, capacity);
        Self {
            location,
            storage_capacity: capacity,
            resources: Vec::new(),
        }
    }
    pub fn add(&mut self, resource: Resource) -> Result<(), String> {
        let size = resource.size_bytes();
        let current = self.current_usage();
        let capacity = self.storage_capacity.size_bytes();
        if current + size > capacity {
            return Err(format!(
                "Vault full! Capacity: {:?}, Current: {} bytes, New Resource: {} bytes",
                self.storage_capacity, current, size
            ));
        }
        self.resources.push(resource);
        println!("Resource added to vault at {}", self.location);
        Ok(())
    }
    pub fn get(&self, index: usize) -> Option<&Resource>{
        self.resources.get(index)
    }
    pub fn summary(&self){
        let mut text_count = 0;
        let mut sensor_count = 0;
        let mut log_count = 0;
        for res in &self.resources {
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
    pub fn remove(&mut self, index: usize) -> Result<Resource, String>{
        if index < self.resources.len() {
            let removed = self.resources.remove(index);
            println!("Resource removed from vault at {}", self.location);
            Ok(removed)
        } else {
            Err(format!("Index out of bounds: {}", index))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_vault() {
        let vault = Vault::new("Global Vault".to_string(), MemorySize::GB(50));
        assert_eq!(vault.location, "Global Vault");
        assert_eq!(vault.storage_capacity, MemorySize::GB(50));
        assert_eq!(vault.resources.len(), 0);
    }
    #[test]
    fn test_add_resource() {
        let mut vault = Vault::new("Global Vault".to_string(), MemorySize::GB(50));
        vault.add(Resource::TextMessage("Hello".to_string())).unwrap();
        assert_eq!(vault.resources.len(), 1);
    }
    #[test]
    fn test_get_resource() {
        let mut vault = Vault::new("Global Vault".to_string(), MemorySize::GB(50));
        vault.add(Resource::TextMessage("Hello".to_string())).unwrap();
        assert_eq!(vault.get(0), Some(&Resource::TextMessage("Hello".to_string())));
    }
    #[test]
    fn test_summary() {
        let mut vault = Vault::new("Global Vault".to_string(), MemorySize::GB(50));
        vault.add(Resource::TextMessage("Hello".to_string())).unwrap();
        vault.add(Resource::SensorData(24.5)).unwrap();
        vault.add(Resource::SystemLogs(vec!["Boot successful".to_string(), "Login detected".to_string(), "Error 404".to_string()])).unwrap();
        vault.summary();
    }
    #[test]
    fn test_safe_retrieval() {
        let vault = Vault::new("Test Vault".to_string(), MemorySize::MB(100));
        // Testing that an empty vault returns None, not a crash!
        assert_eq!(vault.get(0), None);
    }
    #[test]
    fn test_remove_resource() {
        let mut vault = Vault::new("Test Vault".to_string(), MemorySize::MB(100));
        vault.add(Resource::TextMessage("To be deleted".to_string())).unwrap();
        assert_eq!(vault.resources.len(), 1);
        
        let removed = vault.remove(0).unwrap();
        assert_eq!(vault.resources.len(), 0);
        assert_eq!(removed, Resource::TextMessage("To be deleted".to_string()));
    }
}