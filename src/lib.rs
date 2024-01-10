#![no_std]

use core::panic::PanicInfo;

#[macro_use]
extern crate lazy_static;
extern crate multiboot2;

mod drivers;
use crate::drivers::console::console::{MultibootInfo, find_fb, STDOUT};

mod mm;
use mm::page_table_entry::PhysAddr;
use mm::phys_page::kernel_heap_init;

mod utils;

#[no_mangle]
pub extern "C" fn rust_main(info: *mut MultibootInfo, free_mem_base: *mut u8)
{
    // Initialize FrameBuffer
    let buffer_base: *mut u32 = find_fb(info) as *mut u32;
    STDOUT.set_base(buffer_base);

    // Set kernel heap start
    kernel_heap_init(PhysAddr::from(free_mem_base));

    loop{}
}

// #[lang = "eh_personality"] 
// #[no_mangle] 
// pub extern fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}