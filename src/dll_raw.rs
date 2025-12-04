use std::{mem, ptr};
use std::marker::PhantomData;

pub struct Node<T> {
    next: NodePtr<T>,
    prev: NodePtr<T>,
    data: T,
}

pub type NodePtr<T> = *mut Node<T>;

pub struct DLL<T> {
    len: usize,
    first: NodePtr<T>,
    last:  NodePtr<T>,
    _marker: PhantomData<T>,
}

impl<T> Drop for DLL<T> {
    fn drop(&mut self) {
        let mut node = self.first;
        while !node.is_null() {
            let next = unsafe { (*node).next };
            _ = unsafe { Box::from_raw(node) };
            node = next;
        }
    }
}

pub unsafe fn raw_into_box<T>(r: *mut T) -> Box<T> {
    unsafe { mem::transmute(r) }
}

pub unsafe fn box_into_raw<T>(b: Box<T>) -> *mut T {
    unsafe { mem::transmute(b) }
}

impl<T> DLL<T> {

    pub fn new() -> Self {
        DLL { len: 0, first: ptr::null_mut(), last: ptr::null_mut(), _marker: PhantomData }
    }

    pub fn push(&mut self, t: T) {
        self.len += 1;

        let new = Box::new( Node { data: t, next: ptr::null_mut(), prev: self.last } );
        let new = unsafe { box_into_raw(new) };

        if self.last.is_null() {
            // debug_assert!(self.first.is_null());
            self.first = new;
        } else {
            // debug_assert!(!self.first.is_null());
            unsafe { (*self.last).next = new; }
        }
        self.last = new;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.last.is_null() { return None; }
        self.len -= 1;
        if self.len == 0 {
            self.first = ptr::null_mut();
        }
        let old = unsafe { raw_into_box(self.last) };
        self.last = old.prev;
        Some(old.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut dll = DLL::new();
        for n in 0..20_000 {
            for i in 0..n {
                dll.push(i);
            }
            assert_eq!(dll.len, n);
            for i in 0..n {
                assert_eq!(dll.pop(), Some(n - i - 1));
            }
            assert_eq!(dll.pop(), None);
        }
    }
}
