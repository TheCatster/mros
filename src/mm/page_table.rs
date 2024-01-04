use core::arch::asm;
use core::ops::{Index, IndexMut};

use page_table_entry::PTE;
use phys_page;

use super::page_table_entry::{PhysAddr, VirtAddr};

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
    ptes: [PTE;  NUM_PAGE_ENTRY],
}

/// Provide index trait for page table.
impl Index<usize> for PageTable{
    type Output = PTE;

    fn index(&self, index: usize) -> &PTE{
        &self.ptes[index]
    }
}

/// Provide mutable index trait for page table.
impl IndexMut<usize> for PageTable{
    type Output = PTE;

    fn index(&mut self, index: usize) -> &mut PTE{
        &mut self.ptes[index]
    }
}

impl PageTable{
    /// Create a new page table.
    pub fn new() -> Self{
        for pte in self.ptes.iter_mut() {
            pte.set_unused();
        }
    }

    /// Enable this page table.
    pub fn enable(){

    }
}
