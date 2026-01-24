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

    // Test that creating a new Vault initializes fields correctly.
    #[test]
    fn test_new_vault() {
        // We explicitly tell it to be a Vault
        let vault = Vault::<String>::new("Global Vault".to_string(), MemorySize::GB(50));
        assert_eq!(vault.location, "Global Vault");
        assert_eq!(vault.storage_capacity, MemorySize::GB(50));
        assert_eq!(vault.resources.len(), 0);
    }
    // Test adding a resource to the vault and ensuring it is stored.
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
    #[test]
    fn test_overflow() {
        let mut vault = Vault::<String>::new("Overflow Vault".to_string(), MemorySize::KB(1));
        let big_string = "x".repeat(2000); // 2000 bytes
        let res = vault.add("big".to_string(), Resource::TextMessage(big_string));
        assert!(matches!(res, Err(VaultError::VaultFull{..})), "Expected overflow error");
    }
    #[test]
    fn test_duplicate_key() {
        let mut vault = Vault::<String>::new("Dup Vault".to_string(), MemorySize::GB(1));
        vault.add("dup".to_string(), Resource::TextMessage("first".to_string())).unwrap();
        let res = vault.add("dup".to_string(), Resource::TextMessage("second".to_string()));
        assert!(matches!(res, Err(VaultError::InvalidInput(_))), "Expected duplicate key error");
    }
    #[test]
    fn test_memorysize_bytes() {
        assert_eq!(MemorySize::KB(1).size_bytes(), 1024);
        assert_eq!(MemorySize::MB(1).size_bytes(), 1024 * 1024);
        assert_eq!(MemorySize::GB(1).size_bytes(), 1024 * 1024 * 1024);
    }
    #[test]
    fn test_resource_size_bytes() {
        let text = Resource::TextMessage("abc".to_string());
        assert_eq!(text.size_bytes(), 3);
        let sensor = Resource::SensorData(42.0);
        assert_eq!(sensor.size_bytes(), 8);
        let logs = Resource::SystemLogs(vec!["log1".to_string(), "log2".to_string()]);
        assert_eq!(logs.size_bytes(), 8); // 4 + 4
    }
}
