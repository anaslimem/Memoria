#[derive(Debug, Clone, PartialEq)]
pub enum MemorySize {
    KB(u64),
    MB(u64),
    GB(u64),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Resource {
    TextMessage(String),
    SensorData(f64),
    SystemLogs(Vec<String>),
    
}
pub struct Vault{
    pub location: String,
    pub storage_capacity: MemorySize,
    pub resources: Vec<Resource>,
}