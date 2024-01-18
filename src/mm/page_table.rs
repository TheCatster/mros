#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_doc_comments)]

use core::arch::asm;
use core::ops::{Index, IndexMut};
use core::slice::{from_raw_parts, from_raw_parts_mut};

use crate::println;

use super::page_table_entry::{PhysAddr, VirtAddr, PTE, PTEFlags, PRESENT, WRITABLE};
use super::phys_page::{phys_page_alloc, PAGE_SIZE};

/// Store value to cr0.
#[cfg(target_arch = "x86_64")]
pub fn lcr0(mut _val: i64){
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
pub fn lcr3(mut _val: i64){
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
    fn index_mut(&mut self, index: usize) -> &mut PTE{
        let mut ptes: &mut [PTE] = self.to_mut_ptes();
        &mut ptes[index]
    }
}

impl PageTable{
    /// Create a new page table.
    pub fn new() -> Option<Self>{
        let result = phys_page_alloc();
        match result{
            Some(phys_page)=>{
                Some(Self{base: phys_page})
            }
            _ => {
                None
            }
        }
    }

    /// Convert page table to a pte array.
    pub fn to_ptes(&self) -> &'static [PTE]{
        unsafe{
            from_raw_parts(self.base.to_raw_ptr() as *mut PTE, NUM_PAGE_ENTRY)
        }
    }

    /// Convert page table to a mutable pte array.
    pub fn to_mut_ptes(&self) -> &'static mut [PTE]{
        unsafe{
            from_raw_parts_mut(self.base.to_mut_ptr() as *mut PTE, NUM_PAGE_ENTRY)
        }
    }

    /// Enable this page table.
    pub fn enable(&self){
        lcr3(self.base.to_usize() as i64);
    }

    /// Get next level table.
    fn next_mut_table_as_array(&self, pte: PTE) -> &'static mut [PTE]{
        let next_table_addr: PhysAddr = pte.phys_addr();
        unsafe{
            from_raw_parts_mut(next_table_addr.to_mut_ptr() as *mut PTE, NUM_PAGE_ENTRY)
        }
    }

    /// Create next level table.
    fn create_next_table(&self) -> Option<PTE>{
        let new_table = phys_page_alloc();
        match new_table{
            Some(next_table) => {
                Some(PTE::new_table_entry(next_table))
            }
            _ => { return None; }
        }
    }

    /// Create and set up next level table. Return pte points to next table.
    fn setup_next_table(&self, pte: &mut PTE, pte_index: usize){
        if pte.is_unused(){
            let result = self.create_next_table();
            match result{
                Some(new_pte) => {
                    *pte = new_pte;
                }
                _ => {
                    println!("[Err] Create next level page table failed.");
                    return ;
                }
            }
        } else {
            pte.set_flags(PTEFlags::new(PRESENT | WRITABLE));
        }
    }

    /// Create and set up next level table. Return next level table as an array.
    fn setup_next_table_as_array(&self, pte: &mut PTE) -> Option<&'static mut [PTE]>{
        if pte.is_unused(){
            let result = self.create_next_table();
            match result{
                Some(new_pte) => {
                    *pte = new_pte;
                    Some(self.next_mut_table_as_array(new_pte))
                }
                _ => {
                    return None;
                }
            }
        }else{
            Some(self.next_mut_table_as_array(*pte))
        }
    }

    /// Get level 1 page table entry.
    pub fn get_level1_pte(&self, vaddr: VirtAddr) -> &mut PTE{
        let l4_table: & [PTE] = self.to_ptes();
        let l4_index: usize = vaddr.l4_index();
        let l4_pte = l4_table[l4_index];

        let l3_table = self.next_mut_table_as_array(l4_pte);
        let l3_index: usize = vaddr.l3_index();
        let l3_pte = l3_table[l3_index];

        let l2_table = self.next_mut_table_as_array(l3_pte);
        let l2_index: usize = vaddr.l2_index();
        let l2_pte: PTE = l2_table[l2_index];

        let l1_table = self.next_mut_table_as_array(l2_pte);
        let l1_index: usize = vaddr.l1_index();

        &mut l1_table[l1_index]
    }

    /// Map physical to virtual address in this page table.
    /// TODO: WE MUST ALIGN ADDRESSES!
    pub fn map(&mut self, vaddr: VirtAddr, paddr: PhysAddr, flags: PTEFlags){

        let l4_table = self.to_mut_ptes();
        let l4_index: usize = vaddr.l4_index();
        let l4_pte: &mut PTE = &mut l4_table[l4_index];
        self.setup_next_table(l4_pte, l4_index);

        let l3_table = self.next_mut_table_as_array(*l4_pte);
        let l3_index: usize = vaddr.l3_index();
        let l3_pte: &mut PTE = &mut l3_table[l3_index];
        self.setup_next_table(l3_pte, l3_index);

        let l2_table = self.next_mut_table_as_array(*l3_pte);
        let l2_index: usize = vaddr.l2_index();
        let l2_pte: &mut PTE = &mut l2_table[l2_index];
        self.setup_next_table(l2_pte, l2_index);

        let l1_table = self.next_mut_table_as_array(*l2_pte);
        let l1_index: usize = vaddr.l1_index();
        l1_table[l1_index] = PTE::new_page_entry(paddr, flags);
    }

    /// Unmap page.
    pub fn unmap(&mut self, vaddr: VirtAddr){
        let l1_pte = self.get_level1_pte(vaddr);
        l1_pte.set_unused();
    }

    /// Map a physical region [phys_start, phys_start + size) to [virt_start, virt_start + size).
    /// TODO: align addresses.
    pub fn map_region(&mut self, virt_start: VirtAddr, phys_start: PhysAddr, 
                      size: usize, flags: PTEFlags){
        let mut step: usize = 0;
        while step < size{
            let virt_step: VirtAddr = virt_start + step;
            let phys_step: PhysAddr = phys_start + step;
            self.map(virt_step, phys_step, flags);
            step += PAGE_SIZE;
        }
    }

    /// Unmap a virtual region [virt_start, virt_start + size).
    pub fn unmap_region(&mut self, virt_start: VirtAddr, size: usize){
        let mut step: usize = 0;
        while step < size{
            let virt_step: VirtAddr = virt_start + step;
            self.unmap(virt_step);
            step += PAGE_SIZE;
        }
    }

}
