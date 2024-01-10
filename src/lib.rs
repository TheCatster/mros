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
    let result = find_fb(info);
    match result{
        Some(buffer_base)=>{
            STDOUT.lock().set_base(buffer_base as *mut u32);
        }
        _=>{

        }
    }

    loop{}
}

// #[lang = "eh_personality"] 
// #[no_mangle] 
// pub extern fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}