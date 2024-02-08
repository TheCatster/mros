#![allow(dead_code)]
use core::arch::asm;

pub const MSR_EFER: u32   = 0xC0000080;
pub const MSR_STAR: u32   = 0xC0000081;
pub const MSR_LSTAR: u32  = 0xC0000082;
pub const MSR_SFMASK: u32 = 0xC0000084;

/// Read Model-specific register
#[cfg(target_arch = "x86_64")]
pub fn rdmsr(_reg: u32) -> u64{
    let mut low: u32;
    let mut high: u32;

    unsafe{
        asm!("rdmsr", out("eax") low, out("edx") high, in("ecx") _reg);
    }

    (high as u64) << 32 | (low as u64)
}

/// Write Model-specific register
#[cfg(target_arch = "x86_64")]
pub fn wrmsr(_reg: u32, val: u64){
    let low: u32 = val as u32;
    let high: u32 = (val >> 32) as u32;

    unsafe{
        asm!("wrmsr", in("ecx") _reg, in("eax") low, in("edx") _reg);
    }
}