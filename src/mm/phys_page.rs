#![allow(dead_code)]

use page_table_entry::PhysAddr;
use utils::LinkedList;

use spin::Mutex;

/// Set normal page size as 4k.
pub const PAGE_SIZE: usize = 4096;

lazy_static!{
    pub static ref FREE_MEM_BASE: Mutex<PhysAddr> = Mutex::new(PhysAddr{phys_addr: 0});
}

#[inline]
pub fn kernel_heap_init(free_mem_base: PhysAddr){
    FREE_MEM_BASE = free_mem_base;
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
            /// Allocate a new page from kernel heap.
            
        }
    }

    /// Free a physical page.
}

/// Buddy Allocator.
pub struct BuddyAllocator{

}