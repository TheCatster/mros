#![allow(dead_code)]
#![allow(unused_variables)]

use core::arch::asm;
use super::gdt;

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

/// Gate type and DPL for interrupts.
pub const ATTR_INT_GATE: u8 = 0x8e;
/// Gate type and DPL for traps.
pub const ATTR_TRAP_GATE: u8 = 0x8f;

/// Interrupt Descriptor Entry.
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IDE64{
    offset_low: u16,
    selector: u16,
    ist: u8,
    gate_and_dpl: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

impl IDE64{
    // Create a new Interrupt Descriptor Entry.
    pub fn new(offset: u64, gdt_selector: u16, ist_offset: u8, privilege: u8)-> Self{
        Self{
            offset_low: offset & 0xffff,
            selector: gdt_selector,
            ist: ist_offset,
            gate_and_dpl: privilege,
            offset_mid: (offset >> 16) & 0xffff,
            offset_high: (offset >> 32) & 0xffff,
            reserved: 0
        }
    }
}

/// Interrupt Descriptor Table Pointer.
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IDTR{
    idtr: [u32; 3],
}
