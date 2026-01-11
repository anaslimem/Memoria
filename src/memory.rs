#[derive(Debug, Clone, PartialEq)]
pub enum MemorySize {
    KB(u64),
    MB(u64),
    GB(u64),
}

impl MemorySize {
    // Returns the estimated size of the memory in bytes
    pub fn size_bytes(&self) -> u64 {
        match self {
            MemorySize::KB(kb) => kb * 1024,
            MemorySize::MB(mb) => mb * 1024 * 1024,
            MemorySize::GB(gb) => gb * 1024 * 1024 * 1024,
        }
    }
}
