use core::slice::{from_raw_parts_mut};
use core::arch::asm;

pub const GDT_KERNEL_CODE: u64 = 0x08;
pub const GDT_KERNEL_DATA: u64 = 0x10;
pub const GDT_USER_DATA: u64 = 0x18;
pub const GDT_USER_CODE: u64 = 0x20;

/// Load global descriptor table.
#[cfg(target_arch = "x86_64")]
pub fn lgdt(mut _val: u64){
    unsafe{
        asm!("lgdt {}", in(reg) _val);
    }
}

/// Global Descriptor Entry.
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct GDE64{
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    flags_and_limit: u8,
    base_high: u8,
    base_extend: u32,
    reserved: u32,
}

impl GDE64{
    /// Create a new global descriptor entry.
    pub fn new(limit: u64, base: u64, flags: u8, privilege: u8) -> Self{
        Self{
            limit_low: limit & 0xffff,
            base_low: base & 0xffff,
            base_mid: (base >> 16) & 0xffff,
            access: privilege, 
            flags_and_limit: ((limit >> 16) & 0xf) | flags,
            base_high: (base >> 24) & 0xffff,
            base_extend: (base >> 32) &ffffffff,
            reserved: 0,
        }
    }
}

/// Global Descriptor Table Pointer.
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct GDTR{
    gdtr: [u32; 3],
}

impl GDTR{
    /// Create a new global descriptor table pointer.
    pub fn new(num_entries: u32, gdt_base: u64) -> Self{
        Self{
            gdtr: [
                num_entries,
                gdt_base & 0xffffffff,
                (gdt_base >> 32) & 0xffffffff,
            ]
        }
    }

    /// Convert to pointer.
    pub fn to_ptr(&self) -> *mut u32{
        self.gdtr.to_ptr()
    }

    /// Conver to u64.
    pub fn to_u64(&self) -> u64{
        self.gdtr.to_ptr() as u64
    }
}

/// Global Descriptor Table.
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct GDT64{
    entries: [GDE64; 4],
    gdtr: GDTR,
}

impl GDT64{
    /// Create a new global descriptor table.
    pub fn new() -> Self{
        Self{
            entries: [GDE64::new(0xffff, 0, 0xa, 0x9a),
                      GDE64::new(0xffff, 0, 0xc, 0x92),
                      GDE64::new(0xffff, 0, 0xa, 0xfa),
                      GDE64::new(0xffff, 0, 0xc, 0xf2)],
        }
    }

    /// Enable this gdt.
    pub fn enable(&mut self){
        self.gdtr = GDTR::new(self.entries.len(), entries.to_ptr() as u64);
        let mut ptr: u64 = self.gdtr.to_u64();
        lgdt(ptr);
    }
}