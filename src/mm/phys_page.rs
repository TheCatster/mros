#![allow(dead_code)]

use page_table_entry::PhysAddr;
use utils::LinkedList;

use core::slice::from_raw_parts_mut;
use core::ptr::write_bytes;
use spin::Mutex;

use super::page_table_entry::VirtAddr;

/// Set normal page size as 4k.
pub const PAGE_SIZE: usize = 4096;
pub const PHYS_TO_VIRT_BASE: usize = 0;

lazy_static!{
    pub static ref FREE_MEM_BASE: Mutex<PhysAddr> = Mutex::new(PhysAddr{phys_addr: 0});
    pub static ref KERNEL_HEAP_TOP: usize = 0;
}

/// Initialize kernel heap.
#[inline]
pub fn kernel_heap_init(free_mem_base: PhysAddr){
    FREE_MEM_BASE = free_mem_base;
    KERNEL_HEAP_TOP = FREE_MEM_BASE;
}

/// Allocate next physical page direcly from kernel heap.
#[inline]
pub fn alloc_next_frame() -> PhysAddr{
    KERNEL_HEAP_TOP = KERNEL_HEAP_TOP + PAGE_SIZE;
    PhysAddr::from(KERNEL_HEAP_TOP)
}

/// Set the whole physical page to a value.
#[inline]
pub fn set_frame(frame: PhysAddr, val: u8){
    let frame_content: *mut u8 = unsafe{ slice::from_raw_parts_mut(frame.to_mut_ptr(), PAGE_SIZE)};
    unsafe{core::ptr::write_bytes(frame_content, val, PAGE_SIZE)};
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
pub struct SimpleAllocator{
    free_list: LinkedList,
}

impl SimpleAllocator{
    /// Create a new simple physcal page allocator.
    pub fn new() -> Self{
        Self{ free_list: LinkedList::new() }
    }

    /// Allocate a new physical page.
    pub fn alloc() -> PhysAddr{
        if !free_list.is_empty(){
            let mut free_page: usize = *free_list.pop();
            PhysAddr::from(free_page)
        }
        else{
            let frame: PhysAddr = {
                let next_frame: PhysAddr = alloc_next_frame();
                set_frame(next_frame, 0);
                next_frame
            };
            next_frame
        }
    }

    /// Free a physical page.
    pub fn free(paddr: PhysAddr){
        let free_node: *mut usize = (&paddr.to_usize()) as *mut usize;
        free_list.push(free_node);
    }
}

/// TODO: Buddy Allocator.