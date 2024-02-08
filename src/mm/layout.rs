#![allow(dead_code)]
#[macro_use]

use multiboot2::BootInformationHeader;

use super::page_table::PageTable;
use crate::println;

/// Find all memory area from the boot information.
pub fn find_kernel_areas(multiboot_info: usize)
{
    let boot_info = unsafe{
        multiboot2::BootInformation::load(multiboot_info as *const BootInformationHeader)};
    let binding_boot_info = boot_info.expect("Map info exists.");
    let memory_map_tag = binding_boot_info.memory_map_tag();

    let binding_memory_tag = memory_map_tag.expect("Map area exist.");
    let memory_areas = binding_memory_tag.memory_areas();
    for area in memory_areas{
        println!("[+] start: 0x{:x}, end: 0x{:x}", area.start_address(), area.end_address());
    }

    // let kernel_sections = binding_boot_info.elf_sections();
    // let binding_kernel_sections = kernel_sections.expect("Kernel sections.");
    // for section in binding_kernel_sections{
    //     println!("[+] start: {:x}, end: {:x}, flags: {:x}", section.start_address(), section.end_address(), section.flags());
    // }
}


pub struct KernelLayout{
    page_table: PageTable,
}

impl KernelLayout{
    // TODO: we should read the original segments from
    // multiboot info.
}