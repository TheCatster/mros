#![allow(dead_code)]
#![allow(unused_variables)]

use core::arch::asm;
use core::ops::{Index, IndexMut};
use core::slice::from_raw_parts_mut;

use page_table_entry::PTE;
use phys_page::{SimpleAllocator};

use super::page_table_entry::{PhysAddr, VirtAddr, PteFlags};

/// Store value to cr0.
#[cfg(target_arch = "x86_64")]
pub fn lcr0(_val: i64){
    unsafe{
        asm!("movq {}, cr0", out(reg) _val);
    }
}

/// Read value from cr0.
#[cfg(target_arch = "x86_64")]
pub fn rcr0(_val: &mut i64){
    unsafe{
        asm!("movq cr0, {}", in(reg) _val);
    }
}

/// Store value to cr3.
#[cfg(target_arch = "x86_64")]
pub fn lcr3(_val: i64){
    unsafe{
        asm!("movq {}, cr3", out(reg) _val);
    }
}

/// Read value from cr3.
#[cfg(target_arch = "x86_64")]
pub fn rcr3(_val: &i64){
    unsafe{
        asm!("movq cr3, {}", in(reg) _val);
    }
}

/// Every page table holds 512 entries.
pub const NUM_PAGE_ENTRY: usize = 512;

pub struct PageTable{
    base: PhysAddr,
}

/// Provide index trait for page table.
impl Index<usize> for PageTable{
    type Output = PTE;

    fn index(&self, index: usize) -> &PTE{
        let ptes: &'static [PTE] = self.to_ptes();
        &ptes[index]
    }
}

/// Provide mutable index trait for page table.
impl IndexMut<usize> for PageTable{
    type Output = PTE;

    fn index(&mut self, index: usize) -> &mut PTE{
        let mut ptes: &'static [PTE] = self.to_mut_ptes();
        &mut ptes[index]
    }
}

impl PageTable{
    /// Create a new page table.
    pub fn new() -> Self{
        let page_table_base: PhysAddr = SimpleAllocator::alloc();
        Self{ base: page_table_base }
    }

    /// Convert page table to a pte array.
    pub fn to_ptes(&self) -> &'static [PTE]{
        unsafe{
            core::slice::from_raw_parts(self.base.to_raw_ptr() as *mut PTE, NUM_PAGE_ENTRY)
        }
    }

    /// Convert page table to a mutable pte array.
    pub fn to_mut_ptes(&self) -> &'static mut [PTE]{
        unsafe{
            core::slice::from_raw_parts_mut(self.base.to_mut_ptr() as *mut PTE, NUM_PAGE_ENTRY)
        }
    }

    /// Enable this page table.
    pub fn enable(){
        lcr3(self.base.phys_addr);
    }

    /// Find corresponding page table entry and make it mutable.
    fn get_mut_pte(&mut self, vaddr: VirtAddr) -> Option<&mut PTE>{
    }

    /// Access corresponding page table entry read-only.
    fn get_pte(&self, vaddr: VirtAddr) -> Option<&mut PTE>{

    }

    /// Map physical to virtual address in this page table.
    pub fn map(&mut self, vaddr: VirtAddr, paddr: PhysAddr, flags: PteFlags){
    }

    /// Unmap page.
    pub fn unmap(&mut self, vaddr: VirtAddr, paddr: PhysAddr, flags: PteFlags){

    }
}
