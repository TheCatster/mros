#![allow(dead_code)]
#![allow(unused_variables)]

#![no_std]

use core::ops::{Add, AddAssign, Sub, SubAssign};
use bitflags::bitflags;

/// Physical address
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct PhysAddr {
    phys_addr: u64,
}

/// Virtual address
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct VirtAddr {
    virt_addr: u64,
}

impl PhysAddr{
    // Convert from const raw pointer
    #[inline]
    pub const fn from_raw_ptr(&mut self, raw_ptr: *const u8){
        self.phys_addr = raw_ptr as u64;
    }

    // Convert from mutable raw pointer
    #[inline]
    pub const fn from_mut_ptr(&mut self, mut_ptr: *mut u8){
        self.phys_addr = mut_ptr as u64;
    }

    // Convert to const raw pointer
    #[inline]
    pub const fn to_raw_ptr(self) -> *const u8{
        self.phys_addr as *const u8
    }

    // Convert to mutable raw pointer
    #[inline]
    pub const fn to_mut_ptr(self) -> *mut u8{
        self.phys_addr as *mut u8
    }

    // Convert to usize
    #[inline]
    pub const fn to_usize(self) -> usize{
        self.phys_addr as usize
    }
}

impl VirtAddr{
    // Convert to const raw pointer
    #[inline]
    pub const fn to_raw_ptr(self) -> *const u8{
        self.virt_addr as *const u8
    }

    // Convert to mutable raw pointer
    #[inline]
    pub const fn to_mut_ptr(self) -> *const u8{
        self.virt_addr as *mut u8
    }
}

/// Override 'from' trait for physical address
/// 
/// Convert raw pointer to physical address.
impl From<*const u8> for PhysAddr{
    #[inline]
    fn from(raw_ptr: *const u8) -> Self{
        Self{ phys_addr: raw_ptr as u64 }
    }
}

/// Convert mutable pointer to physical address.
impl From<*mut u8> for PhysAddr{
    #[inline]
    fn from(mut_ptr: *mut u8) -> Self{
        Self{ phys_addr: mut_ptr as u64 }
    }
}

/// Override '+' trait for physical address
impl Add<usize> for PhysAddr{
    type Output = Self;
    #[inline]
    fn add(self, offset: usize) -> Self{
        Self{ phys_addr: self.phys_addr + offset }
    }
}

/// Override '+=' trait for physical address
impl AddAssign<usize> for PhysAddr{
    #[inline]
    fn add_assign(&mut self, offset: usize) {
        *self = *self + offset;
    }
}

/// Override '-' trait for physical address
impl Sub<usize> for PhysAddr{
    type Output = Self;
    #[inline]
    fn sub(self, offset: usize) -> Self{
        Self{ phys_addr: self.phys_addr - offset }
    }
}

/// Override '-=' trait for physical address
impl SubAssign<usize> for PhysAddr{
    #[inline]
    fn sub_assign(&mut self, offset: usize){
        *self = *self - offset;
    }
}

/// Override 'from' trait for virtual address.
/// 
/// Convert raw pointer to virtual address.
impl From<*const u8> for VirtAddr{
    #[inline]
    fn from(raw_ptr: *const u8) -> Self{
        Self{ virt_addr: raw_ptr as u64 }
    }
}

/// Convert mutable pointer to virtual address.
impl From<*mut u8> for VirtAddr{
    #[inline]
    fn from(mut_ptr: *mut u8) -> Self{
        Self{ virt_addr: mut_ptr as u64 }
    }
}

/// Override '+' trait for virtual address
impl Add<usize> for VirtAddr{
    type Output = Self;
    #[inline]
    fn add(self, offset: usize) -> Self{
        Self{ virt_addr: self.virt_addr + offset }
    }
}

/// Override '+=' trait for virtual address
impl AddAssign<usize> for VirtAddr{
    #[inline]
    fn add_assign(&mut self, offset: usize){
        *self = *self + offset;
    }
}

/// Override '-' trait for virtual address
impl Sub<usize> for VirtAddr{
    type Output = Self;
    #[inline]
    fn sub(self, offset: usize) -> Self{
        Self{ virt_addr: self.virt_addr - offset }
    }
}

/// Override '-=' trait for virtual address
impl SubAssign<usize> for VirtAddr{
    type Output = Self;
    #[inline]
    fn sub_assign(&mut self, offset: usize){
        *self = *self - offset;
    }
}

/// Page Table Entry Flags

bitflags! {
    pub struct PteFlags: u64 {
        /// The page is in memory.
        const PRESENT =       1;
        /// The page is writable.
        const WRITABLE =      1 << 1;
        /// User mode codes have right to
        /// access this page.
        const USER =          1 << 2;
        /// Writes go directly to memory.
        const WRITE_THROUGH = 1 << 3;
        /// No cache is used for this page.
        const NO_CACHE =      1 << 4;
        /// CPU sets this bit when this page is 
        /// accessed.
        const ACCESSED =      1 << 5;
        /// CPU sets this bit when this page is 
        /// modified.
        const DIRTY =         1 << 6;
        /// Whether to use huge page. Must be 0 in
        /// level-1 and level-4 page table.
        const HUGE_PAGE =     1 << 7;
        /// Page isn't flushed from caches on address
        /// space switch. PGE bit of CR4 register must
        /// be set.
        const GLOBAL =        1 << 8;
        /// Forbid executing code on this page. The
        /// NXE bit in the EFER register must be set.
        const NO_EXECUTE =    1 << 63;
    }
}

/// 64bits page table entry.
#[derive(Clone)]
#[repr(transparent)]
pub struct PTE {
    entry: u64,
}

/// Bits12-52, used for physical address of page or
/// next level page table.
pub const PHYS_ADDR_MASK: u64 = 0x000f_ffff_ffff_f000;

impl PTE {
    /// Create an unused page table entry.
    #[inline]
    pub const fn new() -> Self {
        Self { entry: 0}
    }

    /// Set unused.
    #[inline]
    pub fn set_unused(&mut self){
        self.entry = 0;
    }

    /// Get physical address from page table entry.
    #[inline]
    pub const fn phys_addr(&self) -> PhysAddr{
        PhysAddr{ phys_addr: self.entry & PHYS_ADDR_MASK}
    }

    /// Set physical address.
    #[inline]
    pub fn set_phys_addr(&mut self, paddr: PhysAddr){
        self.entry = (self.entry & !PHYS_ADDR_MASK) | (paddr.phys_addr & PHYS_ADDR_MASK);
    }

    /// Set flags.
    #[inline]
    pub fn set_flags(&mut self, flags: PteFlags){
        self.entry = (self.entry & PHYS_ADDR_MASK) | flags.bits();
    }

    /// Create a page table entry for a new page.
    #[inline]
    pub fn new_page_entry(paddr: PhysAddr, flags: PteFlags) -> Self{
        Self{ entry: flags.bits() | paddr.phys_addr }
    }

    /// Create a page table entry for a new kernel page table.
    #[inline]
    pub fn new_table_entry(paddr: PhysAddr) -> Self{
        let flags: PteFlags = PteFlags::PRESENT | PteFlags::WRITABLE;
        Self{ entry: flags.bits() | paddr.phys_addr }
    }

    /// Create a page table entry for a new user page table.
    #[inline]
    pub fn new_user_table_entry(paddr: PhysAddr) -> Self{
        let flags: PteFlags = PteFlags::PRESENT | PteFlags::WRITABLE | PteFlags:: USER;
        Self{ entry: flags.bits() | paddr.phys_addr }
    }
}