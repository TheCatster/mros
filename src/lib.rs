#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

#[macro_use]
extern crate lazy_static;
extern crate multiboot2;

mod drivers;
use drivers::console::console::{MultibootInfo, fb_init};

mod mm;
use mm::page_table_entry::{VirtAddr, PhysAddr, PTEFlags};
use mm::phys_page::{kernel_heap_init, phys_page_alloc, phys_page_free};
use mm::page_table::{kernel_phys_to_virt, identical_phys_to_virt, PageTable};
use mm::layout::{find_kernel_areas};

mod asms;
// use asms::idt::{IDT64};
// use asms::idt::divide_by_zero_handler;

mod utils;

#[no_mangle]
pub extern "C" fn kernel_start(multiboot_info: usize, free_mem_base: *mut u8){
    // Setup frame buffer.
    fb_init();

    println!("[+] Hello world! This is micro rust os.\n");

    // Setup kernel heap. Enable physical page allocation.
    println!("[+] Setup kernel heap.");
    println!("[+] Mem base: {:x}", free_mem_base as usize);
    let heap_base = PhysAddr::from(free_mem_base);
    kernel_heap_init(&heap_base);

    // Test allocate physical page.
    for i in 0..4{
        let frame = phys_page_alloc();
        match frame{
            Some(phys_page) => {
                println!("[+] Allocate frame: {:x}", phys_page.to_usize());
                if i < 2{
                    phys_page_free(phys_page);
                    println!("[+] Free frame.");
                }
            }
            _=>{
                println!("[Err] Failed allocate frame.");
                return ;
            }
        }
    }

    // Test paging. We map from 0x300000 to 0x10300000.
    println!("\n[+] Enable paging.");
    let create_page_table = PageTable::new();
    match create_page_table{
        Some(page_table) => {
            let mut new_table = page_table;

            let fb_paddr: PhysAddr = PhysAddr::from(0xb8000);
            let fb_vaddr: VirtAddr = identical_phys_to_virt(fb_paddr);
            new_table.map(fb_vaddr, fb_paddr, PTEFlags::new_kern_flags());

            let paddr: PhysAddr = PhysAddr::from(free_mem_base as usize + 0x100000);
            let vaddr: VirtAddr = kernel_phys_to_virt(paddr);
            new_table.map(vaddr, paddr, PTEFlags::new_kern_flags());

            //let curr_page_table: PageTable = new_table.swap();

            let mapped_paddr: PhysAddr = new_table.retrieve(vaddr);
            println!("[+] Virtual: {:x} to Physical: {:x}", vaddr.to_usize(), mapped_paddr.to_usize());

            //curr_page_table.enable();
        }
        _ => {
            println!("[Err] Failed allocate page table.");
        }
    }

    println!("\n[+] Mapping kernel memory areas.");
    find_kernel_areas(multiboot_info);

    // Setup interrupt descriptor table.
    // let idt: IDT64 = IDT64::new();
    // idt.default_setup();
    // println!("[+] Enable interruptions.");

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