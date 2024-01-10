 /// Based on buddy_system_allocator crate.
 use core::ptr;

#[derive(Copy, Clone)]
pub struct LinkedList {
    head: *mut usize,
}

unsafe impl Send for LinkedList {}

impl LinkedList{
    /// Create a new LinkedList.
    pub const fn new() -> Self{
        Self{ head: ptr::null_mut(), }
    }

    /// Check empty.
    pub fn is_empty(&self) -> bool{
        self.head.is_null()
    }

    /// Push to front.
    pub fn push(&mut self, item: *mut usize){
        unsafe{*item = self.head as usize;}
        self.head = item;
    }

    /// Pop from front.
    pub fn pop(&mut self) -> Option<*mut usize>{
        match self.is_empty() {
            true => None,
            false => {
                let item: *mut usize = self.head;
                self.head = unsafe { *item as *mut usize };
                Some(item)
            }
        }
    }

    /// Get iterator.
    pub fn iter(&self) -> Iter{
        Iter{
            curr: self.head,
            list: self,
        }
    }

    /// Get mutable iterator.
    pub fn iter_mut(&mut self) -> IterMut{
        IterMut{
            prev: (&mut self.head as *mut *mut usize) as *mut usize,
            curr: self.head,
            list: self, 
        }
    }
}

/// An iterator over the linked list.
pub struct Iter<'a>{
    curr: *mut usize,
    list: &'a LinkedList,
}

impl<'a> Iterator for Iter<'a>{
    type Item = *mut usize;

    fn next(&mut self) -> Option<Self::Item>{
        if self.curr.is_null(){
            None
        }
        else{
            let item: *mut usize = self.curr;
            let next: *mut usize = unsafe {*item as *mut usize};
            self.curr = next;
            Some(item)
        }
    }
}

/// Represent a mutable node in 'LinkedList'
pub struct ListNode{
    prev: *mut usize,
    curr: *mut usize,
}

impl ListNode{
    /// Remove the node from the list
    pub fn pop(self) -> *mut usize{
        // Skip the current node
        unsafe {
            *(self.prev) = *(self.curr);
        }
        self.curr
    }

    /// Returns the pointed address
    pub fn value(&self) -> *mut usize{
        self.curr
    }
}

/// A mutable iterator over the linked list
pub struct IterMut<'a>{
    list: &'a mut LinkedList,
    prev: *mut usize,
    curr: *mut usize,
}

impl<'a> Iterator for IterMut<'a>{
    type Item = ListNode;

    fn next(&mut self) -> Option<Self::Item>{
        if self.curr.is_null(){
            None
        }
        else{
            let res: ListNode = ListNode{
                prev: self.prev,
                curr: self.curr,
            };
            self.prev = self.curr;
            self.curr = unsafe {*self.curr as *mut usize};
            Some(res)
        }
    }
}