mod resource;
pub use resource::Resource;
mod memory;
pub use memory::MemorySize;
mod vault;
pub use vault::Vault;
pub mod error;
pub mod ui;
pub use error::VaultError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_vault() {
        let vault = Vault::<String>::new("Global Vault".to_string(), MemorySize::GB(50));
        assert_eq!(vault.location, "Global Vault");
        assert_eq!(vault.storage_capacity, MemorySize::GB(50));
        assert_eq!(vault.resources.len(), 0);
    }

    #[test]
    fn test_add_resource() {
        let mut vault = Vault::<String>::new("Global Vault".to_string(), MemorySize::GB(50));
        vault
            .add(
                "greeting".to_string(),
                Resource::TextMessage("Hello".to_string()),
            )
            .unwrap();
        assert_eq!(vault.resources.len(), 1);
    }

    #[test]
    fn test_get_resource() {
        let mut vault = Vault::<String>::new("Global Vault".to_string(), MemorySize::GB(50));
        vault
            .add(
                "greeting".to_string(),
                Resource::TextMessage("Hello".to_string()),
            )
            .unwrap();
        assert_eq!(
            vault.get(&"greeting".to_string()),
            Some(&Resource::TextMessage("Hello".to_string()))
        );
    }

    #[test]
    fn test_summary() {
        let mut vault = Vault::<String>::new("Global Vault".to_string(), MemorySize::GB(50));
        vault
            .add(
                "greeting".to_string(),
                Resource::TextMessage("Hello".to_string()),
            )
            .unwrap();
        vault
            .add("temp".to_string(), Resource::SensorData(24.5))
            .unwrap();
        vault
            .add(
                "syslog".to_string(),
                Resource::SystemLogs(vec![
                    "Boot successful".to_string(),
                    "Login detected".to_string(),
                    "Error 404".to_string(),
                ]),
            )
            .unwrap();
        vault.summary();
    }

    #[test]
    fn test_safe_retrieval() {
        let vault = Vault::<String>::new("Test Vault".to_string(), MemorySize::MB(100));
        assert_eq!(vault.get(&"non_existent".to_string()), None);
    }

    #[test]
    fn test_remove_resource() {
        let mut vault = Vault::<String>::new("Test Vault".to_string(), MemorySize::MB(100));
        vault
            .add(
                "note".to_string(),
                Resource::TextMessage("To be deleted".to_string()),
            )
            .unwrap();
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
        assert!(
            matches!(res, Err(VaultError::VaultFull { .. })),
            "Expected overflow error"
        );
    }

    #[test]
    fn test_duplicate_key() {
        let mut vault = Vault::<String>::new("Dup Vault".to_string(), MemorySize::GB(1));
        vault
            .add(
                "dup".to_string(),
                Resource::TextMessage("first".to_string()),
            )
            .unwrap();
        let res = vault.add(
            "dup".to_string(),
            Resource::TextMessage("second".to_string()),
        );
        assert!(
            matches!(res, Err(VaultError::InvalidInput(_))),
            "Expected duplicate key error"
        );
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
        assert_eq!(logs.size_bytes(), 8);
    }
}
