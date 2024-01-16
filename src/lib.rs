#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

#[macro_use]
extern crate lazy_static;
extern crate multiboot2;

mod drivers;
use crate::drivers::console::console::MultibootInfo;

mod mm;
use drivers::console::fb::FrameBuffer;
use drivers::console::fb_no_font::FrameBufferNoFont;
use mm::page_table_entry::PhysAddr;
use mm::phys_page::kernel_heap_init;

mod utils;

#[no_mangle]
pub extern "C" fn kernel_start(info: *mut MultibootInfo, free_mem_base: *mut u8){
    println!("hello world");
    println!("mem base: {}", free_mem_base as usize);

    loop{}
}

#[lang = "eh_personality"] 
#[no_mangle] 
pub extern fn eh_personality() {}

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}