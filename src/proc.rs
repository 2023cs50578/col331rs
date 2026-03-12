use crate::lapic::lapicid;
use crate::mp::MP_ONCE;
use crate::x86::readeflags;

const FL_IF: u32 = 0x00000200; // Interrupt flag

#[derive(Debug, Clone, Copy)]
pub struct Cpu {
    pub apicid: u8,  // Local APIC ID
}

impl Cpu {
    pub const fn new() -> Self {
        Self { apicid: 0 }
    }
}

// Must be called with interrupts disabled to avoid the caller being
// rescheduled between reading lapicid and running through the loop.
pub fn mycpu() -> &'static Cpu {
    if readeflags() & FL_IF != 0 {
        panic!("mycpu called with interrupts enabled");
    }
    let apicid = lapicid() as u8;
    let cpus = MP_ONCE.cpus.get().unwrap();
    let ncpu = *MP_ONCE.ncpu.get().unwrap();
    for i in 0..ncpu {
        if cpus[i].apicid == apicid {
            return &cpus[i];
        }
    }
    panic!("unknown apicid");
}

// Must be called with interrupts disabled
pub fn cpuid() -> usize {
    let cpu = mycpu() as *const Cpu;
    let cpus = MP_ONCE.cpus.get().unwrap().as_ptr();
    unsafe { cpu.offset_from(cpus) as usize }
}