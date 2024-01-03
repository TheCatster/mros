use core::arch::asm;

use asms::msr::{rdmsr, wrmsr};
use asms::gdt::{GDT_KERNEL_CODE, GDT_KERNEL_DATA, GDT_USER_CODE, GDT_USER_DATA};

#[cfg(target_arch = "x86_64")]
fn syscall(n: i64)->i64
{
    let mut ret: u64 = 0;
    unsafe
    {
        asm!("syscall");
    }
    ret
}

pub fn syscall_init()
{
    // Enable SYSCALL/SYSRET
    wrmsr(MSR_EFER, rdmsr(MSR_EFER) | 0x1);

    // GDT descriptor for SYSCALL/SYSRET (USER descriptor are implicit)
    wrmsr(MSR_STAR, GDT_KERNEL_DATA << 48 | GDT_KERNEL_CODE << 32);

    // TODO: register a system call entry point

    // Disable interrupts (IF) while in s syscall
    wrmsr(MSR_SFMASK, (0x1 << 9));
}

