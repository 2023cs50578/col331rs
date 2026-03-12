use core::arch::asm;

pub fn readeflags() -> u32 {
    let eflags: u32;
    unsafe {
        asm!(
            "pushfd",
            "pop {0:e}",
            out(reg) eflags,
            options(nomem, nostack)
        );
    }
    eflags
}

pub fn inb(port: u16) -> u8 {
    let result: u8;
    unsafe { 
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") result,
            options(nomem, nostack)
        );
        result    
    }
}

pub fn outb(port: u16, value: u8) {
    unsafe { 
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack)
        );    
    }
}

pub fn inw(port: u16) -> u16 {
    let result: u16;
    unsafe { 
        asm!(
            "in ax, dx",
            in("dx") port,
            out("ax") result,
            options(nomem, nostack)
        );
        result    
    }
}

pub fn outw(port: u16, value: u16) {
    unsafe { 
        asm!(
            "out dx, ax",
            in("dx") port,
            in("ax") value,
            options(nomem, nostack)
        );    
    }
}

pub fn inl(port: u16) -> u32 {
    unsafe  { 
        let result: u32;
        asm!(
            "in eax, dx",
            in("dx") port,
            out("eax") result,
            options(nomem, nostack)
        );
        result    
    }
}

pub fn outl(port: u16, value: u32) {
    unsafe { 
        asm!(
            "out dx, eax",
            in("dx") port,
            in("eax") value,
            options(nomem, nostack)
        );    
    }
}