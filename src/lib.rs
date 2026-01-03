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

impl Vault {
    pub fn new(location: String, capacity: MemorySize) -> Self{
        println!("Vault created at {} with capacity {:?}", location, capacity);
        Self {
            location,
            storage_capacity: capacity,
            resources: Vec::new(),
        }
    }
    pub fn add(&mut self, resource: Resource){
        self.resources.push(resource);
        println!("Resource added to vault at {}", self.location);
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
}