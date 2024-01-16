#![allow(dead_code)]

use core::slice::from_raw_parts_mut;
use spin::Mutex;

use super::page_table_entry::{PhysAddr, VirtAddr};
use crate::utils::linked_list::LinkedList;

/// Set normal page size as 4k.
pub const PAGE_SIZE: usize = 4096;
pub const PHYS_TO_VIRT_BASE: usize = 0;

lazy_static!{
    // Start of kernel free memory.
    pub static ref FREE_MEM_BASE: Mutex<PhysAddr> = Mutex::new(PhysAddr::from(0usize));
    // Top of kernel heap.
    pub static ref KERNEL_HEAP_TOP: Mutex<usize> = Mutex::new(0usize);
    // Free memory list.
    pub static ref FREE_MEM_LIST: Mutex<LinkedList> = Mutex::new(LinkedList::new());
}

/// Initialize kernel heap.
#[inline]
pub fn kernel_heap_init(free_mem_base: &PhysAddr){
    FREE_MEM_BASE.lock().clone_from(free_mem_base);
    *KERNEL_HEAP_TOP.lock() = FREE_MEM_BASE.lock().to_usize();
}

/// Allocate next physical page direcly from kernel heap.
#[inline]
pub fn alloc_next_frame() -> PhysAddr{
    *KERNEL_HEAP_TOP.lock() += PAGE_SIZE;
    PhysAddr::from(*KERNEL_HEAP_TOP.lock())
}

/// Set the whole physical page to a value.
#[inline]
pub fn set_frame(frame: PhysAddr, val: u8){
    let mut frame_content: &mut [u8] = unsafe{from_raw_parts_mut(frame.to_mut_ptr(), PAGE_SIZE)};
    for i in 0..PAGE_SIZE{
        frame_content[i] = val;
    }
}

/// Simple physical to virtual translation by direct-mapping.
pub fn simple_phys_to_virt(paddr: PhysAddr) -> VirtAddr{
    VirtAddr::from(paddr.to_usize() + PHYS_TO_VIRT_BASE)
}

/// TODO: Delicated physical to virtual translation.
pub fn phys_to_virt(paddr: PhysAddr) -> VirtAddr{
    simple_phys_to_virt(paddr)
}

/// Simple allocator: use a linked list as free memory pool.
pub fn phys_page_alloc() -> Option<PhysAddr>{
    if !FREE_MEM_LIST.lock().is_empty(){
        let result: Option<*mut usize> = FREE_MEM_LIST.lock().pop();
        match result{
            Some(free_page) => {
                Some(PhysAddr::from(free_page))
            }
            _ =>{
                None
            }
        }
    }
    else{
        let frame: PhysAddr = {
            let next_frame: PhysAddr = alloc_next_frame();
            set_frame(next_frame, 0);
            next_frame
        };
        Some(frame)
    }
}

/// Free a physical page.
pub fn phys_page_free(paddr: PhysAddr){
    let free_node: *mut usize = paddr.to_mut_ptr() as *mut usize;
    FREE_MEM_LIST.lock().push(free_node);
}