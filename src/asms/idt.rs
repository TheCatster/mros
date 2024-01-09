#![allow(dead_code)]
#![allow(unused_variables)]

use core::arch::asm;

/// Clear interrupt flag.
#[cfg(target_arch = "x86_64")]
pub fn cli(){
    unsafe{
        asm!("cli");
    }
}

/// Set interrupt flag.
#[cfg(target_arch = "x86_64")]
pub fn sti(){
    unsafe{
        asm!("sti");
    }
}