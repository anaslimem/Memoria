mod resource;  // Declare the resource module
pub use resource::Resource;  // Import Resource enum from resource module
mod memory;  
pub use memory::MemorySize;  
mod vault;
pub use vault::Vault;
pub mod ui;

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