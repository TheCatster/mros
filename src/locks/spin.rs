#![allow(dead_code)]
#![allow(unused_variables)]

use asms::idt;

pub struct Spinlock{
    lock: u32,
}

impl Spinlock{
    pub fn new() -> Self{
        Self{ lock: 0}
    }
}