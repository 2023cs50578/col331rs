#![no_std]
#![no_main]

mod param;
mod x86;
mod uart;
mod console;
mod lapic;
mod ioapic;
mod picirq;
mod mp;
mod proc;

use core::panic::PanicInfo;

use uart::*;
use x86::*;
use lapic::*;
use ioapic::*;
use picirq::*;
use mp::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel Panic: {:?}", info);
    loop {}
}

fn halt() -> ! {
    println!("Bye COL{}\n\0", 331);
    loop {
        outw(0x604, 0x2000);
        // For older versions of QEMU, 
        outw(0xB004, 0x2000);
    }
}

#[no_mangle]
fn entryofrust() -> ! {
    uartinit();
    println!("Hello from Rust Kernel!");
    mpinit();
    lapicinit();
    println!("lapics initialized. !!");
    picinit();
    println!("pics disabled !!");
    ioapic_init();
    println!("ioapics initialized !!");

    halt();
}