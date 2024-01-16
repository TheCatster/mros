#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

#[macro_use]
extern crate lazy_static;
extern crate multiboot2;

mod drivers;
use crate::drivers::console::console::{MultibootInfo, find_fb, STDOUT};

mod mm;
use drivers::console::fb::FrameBuffer;
use drivers::console::fb_no_font::FrameBufferNoFont;
use mm::page_table_entry::PhysAddr;
use mm::phys_page::kernel_heap_init;

mod utils;

#[no_mangle]
pub extern "C" fn kernel_start(info: *mut MultibootInfo, free_mem_base: *mut u8){
    let mut fb: FrameBufferNoFont = FrameBufferNoFont{
        _width: 80,
        _height: 25,
        _pos_x: 0,
        _pos_y: 0,
        _buffer: 0xb8000 as usize,
    };

    let hello: &str = "hello world";
    fb.print_str(hello);

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