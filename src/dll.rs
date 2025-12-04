use std::fmt::Debug;
use std::ptr::NonNull;

pub struct Node<T: Copy + Debug> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    value: T,
}

pub struct DLL<T: Copy + Debug> {
    len: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

impl<T: Copy + Debug> DLL<T> {
    fn new() -> Self {
        Self { len: 0, head: None, tail: None }
    }

    fn push(&mut self, value: T) {
        self.len += 1;
        match self.tail {
            None => {
                let node = Box::new(Node { next: None, prev: None, value });
                let node_ptr = unsafe { NonNull::new_unchecked(Box::into_raw(node)) };
                self.head = Some(node_ptr);
                self.tail = Some(node_ptr);
            }
            Some(tail) => {
                eprintln!("push tail: {:?}", tail);
                eprintln!("push self len {}", self.len);

                let node = Box::new(Node { next: None, prev: Some(tail), value });
                let node_ptr = unsafe { NonNull::new_unchecked(Box::into_raw(node)) };
                unsafe { (*tail.as_ptr()).next = Some(node_ptr) };
                self.tail = Some(node_ptr);
            }
        }
    }

    fn pop(&mut self) -> Option<T> {
        match self.tail {
            None => None,
            Some(tail) if self.len == 1 => {
                self.len = 0;
                self.tail = None;
                self.head = None;
                let value = unsafe { tail.as_ref() }.value;
                unsafe { tail.drop_in_place() };
                Some(value)
            }
            Some(tail) => {
                self.len -= 1;
                eprintln!("tail: {:?}", tail);
                eprintln!("self len {}", self.len);
                let value = unsafe { tail.as_ref() }.value.clone();
                dbg!(value);
                self.tail = unsafe { tail.as_ref() }.prev.clone();
                self.tail.map(|tail| unsafe { (*tail.as_ptr()).next = None });
                // unsafe { (*(*tail.as_ptr()).prev.unwrap().as_ptr()).next = None };
                unsafe { tail.drop_in_place() };
                Some(value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dll::DLL;

    #[test]
    fn it_works() {
        let mut dll = DLL::new();
        let n = 1000;
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
