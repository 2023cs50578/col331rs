#[derive(Debug, Clone, Copy)]
pub struct Cpu {
    pub apicid: u8,  // Local APIC ID
}

impl Cpu {
    pub const fn new() -> Self {
        Self { apicid: 0 }
    }
}