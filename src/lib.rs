#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

#[macro_use]
extern crate lazy_static;
extern crate multiboot2;

mod drivers;
use crate::drivers::console::console::{MultibootInfo, fb_init};

mod mm;
use mm::page_table_entry::PhysAddr;
use mm::phys_page::{kernel_heap_init, phys_page_alloc, phys_page_free};

mod utils;

#[no_mangle]
pub extern "C" fn kernel_start(info: *mut MultibootInfo, free_mem_base: *mut u8){
    // Setup frame buffer.
    fb_init();

    println!("[+] Hello world! This is micro rust os.");

    // Setup kernel heap. Enable physical page allocation.
    println!("[+] Setup kernel heap.");
    println!("[+] Mem base: {:x}", free_mem_base as usize);
    let heap_base = PhysAddr::from(free_mem_base);
    kernel_heap_init(&heap_base);

    // Test allocate physical page.
    for i in 0..5{
        let frame = phys_page_alloc();
        match frame{
            Some(phys_page) => {
                println!("[+] Allocate frame: {:x}", phys_page.to_usize());
                if i < 3{
                    phys_page_free(phys_page);
                }
            }
            _=>{
                println!("[Err] Failed allocate frame.");
                return ;
            }
        }
    }

    // Enable paging.
    

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