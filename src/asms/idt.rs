#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]

use core::arch::asm;
use core::ops::{Index, IndexMut};

use super::gdt::{GDT_KERNEL_CODE};

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
pub const NUM_INTERRUPT_DESP_ENTRIES: usize = 256;
pub const MASTER_PIC_BOUND: u32 = 0x20;
pub const SLAVE_PIC_BOUND: u32 = 0x28;

/// Type for interrupt handler functions.
pub type InterruptHandler = extern "C" fn(&mut IntrStackFrame) -> !;

/// Handler without error code.
macro_rules! handler_without_error_code{
    ($name: ident) => {{
        #[naked]
        extern "C" fn handler_wrapper() -> !{
            unsafe{
                asm!("mov rdi, rsp
                      sub rsp, 8
                      call $0" :: "i"($name as extern "C" fn(&mut IntrStackFrame)->!)
                      : "rdi" : "intel");
            }
        }
        handler_wrapper()
    }}
}

/// Handler with error code.
macro_rules! handler_with_error_code{
    ($name: ident) => {{
        #[naked]
        extern "C" fn handler_wrapper() -> !{
            unsafe{
                asm!("pop rsi
                      mov rdi, rsp
                      sub rsp, 8
                      call $0" :: "i"($name as extern "C" fn(&mut IntrStackFrame)->!)
                      : "rdi" : "intel");
            }
        }
        handler_wrapper()
    }}
}

pub enum InterruptTypes{
    IvDevideError,
    IvDebug,
    IvNMI,
    IvBreakpoint,
    IvOverflow,
    IvBoundCheck,
    IvInvalidOpcode,
    IvDeviceNotAvailable,
    IvDoubleFault,
    IvCoprocessorSegOverrun,
    IvInvalidTss,
    IvSegmentNotPresent,
    IvStackSegment,
    IvGeneralProtection,
    IVPageFault,
    IvIntelReserved,
    IvFloatingPointError,
    IvAlignmentCheck,

    IvTimer = 0x20,
    IvKeyboard,
    IvPicCascade,
    IvNetwork,
    IvMouse,
    IvIDE = 0x2e,
    IvSyscall = 0x80,
}

/// Interrupt Vectors.
#[repr(C, packed)]
pub struct IV64{
    entries: [InterruptHandler; NUM_INTERRUPT_DESP_ENTRIES],
}

impl Index<usize> for IV64{
    type Output = InterruptHandler;

    fn index(&self, index: usize) -> &InterruptHandler{
        &self.entries[index]
    }
}

impl IndexMut<usize> for IV64{
    fn index_mut(&mut self, index: usize) -> &mut InterruptHandler{
        &mut self.entries[index]
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
    pub fn default_setup(&mut self){
    }

    /// Set one interrupt handler.
    pub fn set_handler(&mut self, intr_type: InterruptTypes, with_error_code: bool, handler: InterruptHandler){
        if with_error_code{
            self.entries[intr_type as usize] = handler_with_error_code!(handler);
        }else{
            self.entries[intr_type as usize] = handler_without_error_code!(handler);
        }
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
            offset_low: (offset & 0xffff) as u16,
            selector: gdt_selector,
            ist: ist_offset,
            gate_and_dpl: privilege,
            offset_mid: ((offset >> 16) & 0xffff) as u16,
            offset_high: ((offset >> 32) & 0xffff) as u32,
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
                (idt_base & 0xffffffff) as u32,
                ((idt_base >> 32) & 0xffffffff) as u32,
            ]
        }
    }

    /// Convert to pointer.
    pub fn to_ptr(&self) -> *mut u32{
        self.idtr.as_ptr() as *mut u32
    }

    /// Conver to u64.
    pub fn to_u64(&self) -> u64{
        self.idtr.as_ptr() as u64
    }

}

/// Interrupt Descriptor Table.
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
            idtr: IDTR::null(),
        }
    }

    /// Defaultly setup the idt.
    pub fn default_setup(&mut self){
        // Setup interrupt vector.
        self.iv.default_setup();

        for i in 0..NUM_INTERRUPT_DESP_ENTRIES{
            self.entries[i] = IDE64::new(self.iv[i] as u64, GDT_KERNEL_CODE as u16, 0, ATTR_INT_GATE);
        }
    }

    /// Set a particular interrupt handler.
    pub fn set_handler(&mut self, with_error_code: bool, 
        intr_type: InterruptTypes, handler: InterruptHandler){
        self.iv.set_handler(intr_type, with_error_code, handler);
        self.entries[i] = IDE64::new(self.iv[intr_type as usize] as u64, 
            GDT_KERNEL_CODE as u16, 0, ATTR_INT_GATE);
    }

    /// Enable the idt.
    pub fn enable(&mut self){
        self.idtr = IDTR::new(NUM_INTERRUPT_DESP_ENTRIES as u32, self.entries.as_ptr() as u64);
        let mut ptr: u64 = self.idtr.to_u64();
        lidt(ptr);
    }
}

/// Interrupt Stack Frame.
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IntrStackFrame{
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}

/// For test only.
pub extern "C" fn divide_by_zero_handler(intr_frame: &mut IntrStackFrame) -> !{
    println!("This is divide by zero handler.");
    loop{}
}