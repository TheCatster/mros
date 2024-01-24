#![allow(dead_code)]
#![allow(unused_variables)]


use core::ops::{Add, AddAssign, Sub, SubAssign, BitAnd, BitOr, BitAndAssign, BitOrAssign};

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
    /// Convert from const raw pointer
    #[inline]
    pub fn from_raw_ptr(&mut self, raw_ptr: *const u8){
        self.phys_addr = raw_ptr as u64;
    }

    /// Convert from mutable raw pointer
    #[inline]
    pub fn from_mut_ptr(&mut self, mut_ptr: *mut u8){
        self.phys_addr = mut_ptr as u64;
    }

    /// Convert to const raw pointer
    #[inline]
    pub fn to_raw_ptr(&self) -> *const u8{
        self.phys_addr as *const u8
    }

    /// Convert to mutable raw pointer
    #[inline]
    pub fn to_mut_ptr(&self) -> *mut u8{
        self.phys_addr as *mut u8
    }

    /// Convert to usize
    #[inline]
    pub fn to_usize(&self) -> usize{
        self.phys_addr as usize
    }

    /// Conver to u64
    #[inline]
    pub fn to_u64(&self) -> u64{
        self.phys_addr
    }
}

impl VirtAddr{
    /// Convert to const raw pointer
    #[inline]
    pub const fn to_raw_ptr(&self) -> *const u8{
        self.virt_addr as *const u8
    }

    /// Convert to mutable raw pointer
    #[inline]
    pub const fn to_mut_ptr(&self) -> *const u8{
        self.virt_addr as *mut u8
    }

    /// Conver to usize.
    #[inline]
    pub fn to_usize(&self) -> usize{
        self.virt_addr as usize
    }

    /// Get level 4 index.
    #[inline]
    pub const fn l4_index(&self) -> usize{
        (self.virt_addr as usize) >> 39 & 0x1ff
    }

    /// Get level 3 index.
    #[inline]
    pub const fn l3_index(&self) -> usize{
        (self.virt_addr as usize) >> 30 & 0x1ff
    }

    /// Get level 2 index.
    #[inline]
    pub const fn l2_index(&self) -> usize{
        (self.virt_addr as usize) >> 21 & 0x1ff
    }

    /// Get level 1 index.
    #[inline]
    pub const fn l1_index(&self) -> usize{
        (self.virt_addr as usize) >> 12 & 0x1ff
    }

    /// Given level, get index.
    #[inline]
    pub const fn index(&self, level: u32) -> usize{
        match level{
            4 => {self.l4_index()}
            3 => {self.l3_index()}
            2 => {self.l2_index()}
            1 => {self.l1_index()}
            _ => {0x200usize}
        }
    }

    /// Get physical offset.
    #[inline]
    pub const fn offset(&self) -> usize{
        (self.virt_addr as usize) & 0xfff
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

impl From<*mut usize> for PhysAddr{
    #[inline]
    fn from(rhs: *mut usize) -> Self{
        Self{ phys_addr: rhs as u64 }
    }
}

/// Conver usize to physical address.
impl From<usize> for PhysAddr{
    #[inline]
    fn from(rhs: usize) -> Self{
        Self{ phys_addr: rhs as u64 }
    }
}

/// Override '+' trait for physical address
impl Add<usize> for PhysAddr{
    type Output = Self;
    #[inline]
    fn add(self, offset: usize) -> Self{
        Self{ phys_addr: self.phys_addr + offset as u64 }
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
        Self{ phys_addr: self.phys_addr - offset as u64 }
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

/// Convert usize to virtual address
impl From<usize> for VirtAddr{
    #[inline]
    fn from(addr: usize) -> Self{
        Self{ virt_addr: addr as u64 }
    }
}

/// Override '+' trait for virtual address
impl Add<usize> for VirtAddr{
    type Output = Self;
    #[inline]
    fn add(self, offset: usize) -> Self{
        Self{ virt_addr: self.virt_addr + offset as u64 }
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
        Self{ virt_addr: self.virt_addr - offset as u64 }
    }
}

/// Override '-=' trait for virtual address
impl SubAssign<usize> for VirtAddr{
    #[inline]
    fn sub_assign(&mut self, offset: usize){
        *self = *self - offset;
    }
}

#[derive(Clone, Copy)]
pub struct PTEFlags{
    flags: u64,
}

impl PTEFlags{
    #[inline]
    pub fn new(bits: u64) -> Self{
        Self{ flags: bits}
    }

    #[inline]
    pub fn as_u64(&self) -> u64{
        self.flags
    }

    #[inline]
    pub fn is_contain(&self, bit: u64) -> bool{
        if self.flags & bit != 0{
            true
        }
        else{
            false
        }
    }

    #[inline]
    pub fn new_kern_flags() -> Self{
        Self{ flags: PRESENT | WRITABLE }
    }

    #[inline]
    pub fn new_user_flags() -> Self{
        Self{ flags: PRESENT | WRITABLE | USER }
    }
}

impl From<usize> for PTEFlags{
    #[inline]
    fn from(bits: usize) -> Self{
        Self{ flags: bits as u64}
    }
}

impl BitAnd<PTEFlags> for u64{
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: PTEFlags) -> Self::Output {
        self | rhs.as_u64()
    }
}

impl BitAnd<u64> for PTEFlags{
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: u64) -> Self::Output {
        Self{flags: self.flags & rhs}
    }
}

impl BitAndAssign<u64> for PTEFlags{
    #[inline]
    fn bitand_assign(&mut self, rhs: u64) {
        *self = Self{flags: self.flags & rhs}
    }
}

impl BitOr<u64> for PTEFlags{
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: u64) -> Self::Output {
        Self{flags: self.flags | rhs}
    }
}

impl BitOr<PTEFlags> for u64{
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: PTEFlags) -> Self::Output {
        self | rhs.as_u64()
    }
}

impl BitOrAssign<u64> for PTEFlags{
    #[inline]
    fn bitor_assign(&mut self, rhs: u64) {
        *self = Self{flags: self.flags | rhs}
    }
}


/// Page Table Entry Flags
/// The page is in memory.
pub const PRESENT: u64 =       1;
/// The page is writable.
pub const WRITABLE: u64 =      1 << 1;
/// User mode codes have right to
/// access this page.
pub const USER: u64 =          1 << 2;
/// Writes go directly to memory.
pub const WRITE_THROUGH: u64 = 1 << 3;
/// No cache is used for this page.
pub const NO_CACHE: u64 =      1 << 4;
/// CPU sets this bit when this page is 
/// accessed.
pub const ACCESSED: u64 =      1 << 5;
/// CPU sets this bit when this page is 
/// modified.
pub const DIRTY: u64 =         1 << 6;
/// Whether to use huge page. Must be 0 in
/// level-1 and level-4 page table.
pub const HUGE_PAGE: u64 =     1 << 7;
/// Page isn't flushed from caches on address
/// space switch. PGE bit of CR4 register must
/// be set.
pub const GLOBAL: u64 =        1 << 8;
/// Forbid executing code on this page. The
/// NXE bit in the EFER register must be set.
pub const NO_EXECUTE: u64 =    1 << 63;

/// 64bits page table entry.
#[derive(Clone, Copy)]
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
    pub fn set_flags(&mut self, flags: PTEFlags){
        self.entry = (self.entry & PHYS_ADDR_MASK) | flags;
    }

    /// Create a page table entry for a new page.
    #[inline]
    pub fn new_page_entry(paddr: PhysAddr, flags: PTEFlags) -> Self{
        Self{ entry: flags.as_u64() | paddr.phys_addr }
    }

    /// Create a page table entry for a new kernel page table.
    #[inline]
    pub fn new_table_entry(paddr: PhysAddr) -> Self{
        let flags: PTEFlags = PTEFlags{flags: PRESENT | WRITABLE};
        Self{ entry: flags.as_u64() | paddr.phys_addr }
    }

    /// Create a page table entry for a new user page table.
    #[inline]
    pub fn new_user_table_entry(paddr: PhysAddr) -> Self{
        let flags: PTEFlags = PTEFlags{flags: PRESENT | WRITABLE | USER};
        Self{ entry: flags.as_u64() | paddr.phys_addr }
    }

    /// Determine if the pte contains a specific flag.
    #[inline]
    pub fn is_contain(&self, flag: u64) -> bool{
        return self.entry & flag != 0;
    }

    /// Determine if the page is present.
    #[inline]
    pub fn is_present(&self) -> bool{
        self.is_contain(PRESENT)
    }

    /// Determine if the pte is used.
    #[inline]
    pub fn is_unused(&self) -> bool{
        return self.entry & 0 == 0;
    }

}