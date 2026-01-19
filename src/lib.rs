mod resource;  // Declare the resource module
pub use resource::Resource;  // Import Resource enum from resource module
mod memory;  
pub use memory::MemorySize;  
mod vault;
pub use vault::Vault;
pub mod ui;
pub mod error;
pub use error::VaultError;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_vault() {
        // We explicitly tell it to be a Vault<String>
        let vault = Vault::<String>::new("Global Vault".to_string(), MemorySize::GB(50));
        assert_eq!(vault.location, "Global Vault");
        assert_eq!(vault.storage_capacity, MemorySize::GB(50));
        assert_eq!(vault.resources.len(), 0);
    }
    #[test]
    fn test_add_resource() {
        // Inference might work here, but explicit is safer for tests
        let mut vault = Vault::<String>::new("Global Vault".to_string(), MemorySize::GB(50));
        vault.add("greeting".to_string(), Resource::TextMessage("Hello".to_string())).unwrap();
        assert_eq!(vault.resources.len(), 1);
    }
    #[test]
    fn test_get_resource() {
        let mut vault = Vault::<String>::new("Global Vault".to_string(), MemorySize::GB(50));
        vault.add("greeting".to_string(), Resource::TextMessage("Hello".to_string())).unwrap();
        // Since get takes &K, and K is String, we pass &String (or &str that coerces)
        assert_eq!(vault.get(&"greeting".to_string()), Some(&Resource::TextMessage("Hello".to_string())));
    }
    #[test]
    fn test_summary() {
        let mut vault = Vault::<String>::new("Global Vault".to_string(), MemorySize::GB(50));
        vault.add("greeting".to_string(), Resource::TextMessage("Hello".to_string())).unwrap();
        vault.add("temp".to_string(), Resource::SensorData(24.5)).unwrap();
        vault.add("syslog".to_string(), Resource::SystemLogs(vec!["Boot successful".to_string(), "Login detected".to_string(), "Error 404".to_string()])).unwrap();
        vault.summary();
    }
    #[test]
    fn test_safe_retrieval() {
        let vault = Vault::<String>::new("Test Vault".to_string(), MemorySize::MB(100));
        // Testing that a missing key returns None
        assert_eq!(vault.get(&"non_existent".to_string()), None);
    }
    #[test]
    fn test_remove_resource() {
        let mut vault = Vault::<String>::new("Test Vault".to_string(), MemorySize::MB(100));
        vault.add("note".to_string(), Resource::TextMessage("To be deleted".to_string())).unwrap();
        assert_eq!(vault.resources.len(), 1);
        
        let removed = vault.remove(&"note".to_string()).unwrap();
        assert_eq!(vault.resources.len(), 0);
        assert_eq!(removed, Resource::TextMessage("To be deleted".to_string()));
    }
}