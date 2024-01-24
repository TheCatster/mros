#![allow(dead_code)]
#![allow(unused_variables)]

use core::arch::asm;
use core::ops::{Index, IndexMut};
use super::gdt;
use crate::syscall::syscall;

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

/// Load idtr.
#[cfg(target_arch = "x86_64")]
pub fn lidt(mut _val: u64){
    unsafe{
        asm!("lidt {}", in(reg) _val);
    }
}

/// Gate type and DPL for interrupts.
pub const ATTR_INT_GATE: u8 = 0x8e;
/// Gate type and DPL for traps.
pub const ATTR_TRAP_GATE: u8 = 0x8f;

/// Total number of interrupt descriptor entries.
pub const NUM_INTERRUPT_DESP_ENTRIES: u32 = 256;
pub const MASTER_PIC_BOUND: u32 = 0x20;
pub const SLAVE_PIC_BOUND: u32 = 0x28;

/// Type for interrupt handler functions.
pub type InterruptHandler = extern "C" fn();

enum InterruptTypes{
    IV_DEVIDE_ERROR,
    IV_DEBUG,
    IV_NMI,
    IV_BREAKPOINT,
    IV_OVERFLOW,
    IV_BOUND_CHECK,
    IV_INVALID_OPCODE,
    IV_DEVICE_NOT_AVAILABLE,
    IV_DOUBLE_FAULT,
    IV_COPROCESSOR_SEG_OVERRUN,
    IV_INVALID_TSS,
    IV_SEGMENT_NOT_PRESENT,
    IV_STACK_SEGMENT,
    IV_GENERAL_PROTECTION,
    IV_PAGE_FAULT,
    IV_INTEL_RESERVED,
    IV_FLOATING_POINT_ERROR,
    IV_ALIGNMENT_CHECK,

    IV_TIMER = 0x20,
    IV_KEYBOARD,
    IV_PIC_CASCADE,
    IV_NETWORK,
    IV_MOUSE,
    IV_IDE = 0x2e,
    IV_SYSCALL = 0x80,
}

/// Interrupt Vectors.
#[repr(C, packed)]
pub struct IV64{
    entries: [InterruptHandler; NUM_INTERRUPT_DESP_ENTRIES],
}

impl Index<usize> for IV64{
    type Output = InterruptHandler;

    fn index(&self, index: usize) -> &InterruptHandler{
        &entries[index]
    }
}

impl IndexMut<usize> for IV64{
    fn index_mut(&mut self, index: usize) -> &mut InterruptHandler{
        &mut entries[index]
    }
}

/// Temporary code.
extern "C"{
    pub fn timer_apic();
}

impl IV64{
    /// Create a new interrupt vector.
    pub fn new() -> Self{
        Self{
            entries: [0; NUM_INTERRUPT_DESP_ENTRIES],
        }
    }

    /// Setup the interrupt vector in default.
    pub fn default_setup(){
        entries[IV_TIMER] = timer_apic;
        // TODO: syscall.
    }
}

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
    /// Create a new Interrupt Descriptor Entry.
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

    /// Create a null Interrupt Descriptor Entry.
    pub fn null() -> Self{
        Self{ 
            offset_low: 0, selector: 0, ist: 0, gate_and_dpl: 0, offset_mid: 0,
            offset_high: 0, reserved: 0
        }
}

/// Interrupt Descriptor Table Pointer.
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IDTR{
    idtr: [u32; 3],
}

impl IDTR{
    /// Create a null idtr.
    pub fn null() -> Self{
        Self{
            idtr: [0, 0, 0,]
        }
    }

    /// Create a new idtr.
    pub fn new(num_entries: u32, idt_base: u64) -> Self{
        Self{
            idtr: [
                num_entries,
                idt_base & 0xffffffff,
                (idt_base >> 32) & 0xffffffff,
            ]
        }
    }

    /// Convert to pointer.
    pub fn to_ptr(&self) -> *mut u32{
        self.idtr.to_ptr()
    }

    /// Conver to u64.
    pub fn to_u64(&self) -> u64{
        self.idtr.to_ptr() as u64
    }

}

/// Interrupt Descriptor Table.
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IDT64{
    iv: IV64,
    entries: [IDE64; NUM_INTERRUPT_DESP_ENTRIES],
    idtr: IDTR,
}

impl IDT64{
    /// Create a new idt.
    pub fn new() -> Self{
        Self{
            iv: IV64::new(),
            entries: [IDE64::null(); NUM_INTERRUPT_DESP_ENTRIES],
            idtr: [IDTR::null(); 3]
        }
    }

    /// Defaultly setup the idt.
    pub fn default_setup(&mut self){
        // Setup interrupt vector.
        self.iv.default_setup();

        for i in 0..NUM_INTERRUPT_DESP_ENTRIES{
            self.ntries[i] = IDE64::new(self.iv[i] as u64, GDT_KERNEL_CODE, 0, ATTR_INT_GATE);
        }
    }

    /// Enable the idt.
    pub fn enable(&mut self){
        self.idtr = IDTR::new(NUM_INTERRUPT_DESP_ENTRIES, self.entries.to_ptr() as u64);
        let mut ptr: u64 = self.idtr.to_u64();
        lidt(ptr);
    }
}

/// Interrupt Stack Frame.
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IntrStackFrame{
    
}