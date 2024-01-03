use core::arch::asm;

/// Store value to cr0.
#[cfg(target_arch = "x86_64")]
pub fn lcr0(_val: i64){
    unsafe{
        asm!("movq {}, cr0", out(reg) _val);
    }
}

/// Read value from cr0.
#[cfg(target_arch = "x86_64")]
pub fn rcr0(_val: &mut i64){
    unsafe{
        asm!("movq cr0, {}", in(reg) _val);
    }
}

/// Store value to cr3.
#[cfg(target_arch = "x86_64")]
pub fn lcr3(_val: i64){
    unsafe{
        asm!("movq {}, cr3", out(reg) _val);
    }
}

/// Read value from cr3.
#[cfg(target_arch = "x86_64")]
pub fn rcr3(_val: &i64){
    unsafe{
        asm!("movq cr3, {}", in(reg) _val);
    }
}
