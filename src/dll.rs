use std::ptr::NonNull;

pub struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    value: T,
}

pub struct DLL<T> {
    len: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

impl<T> Drop for DLL<T> {
    fn drop(&mut self) {
        let mut node = self.head;
        while let Some(n) = node {
            node = unsafe { (*n.as_ptr()).next };
            _ = unsafe { Box::from_raw(n.as_ptr()) };
        }
    }
}

impl<T> DLL<T> {
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
            Some(tail) => {
                let tail = unsafe { Box::from_raw(tail.as_ptr()) };
                self.len -= 1;

                if self.len == 0 {
                    self.head = None;
                } else {
                    tail.prev.map(|p| unsafe { (*p.as_ptr()).next = None });
                }
                self.tail = tail.prev;

                Some(tail.value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dll::DLL;

    #[cfg(not(miri))]
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

    #[test]
    fn miri() {
        let mut dll = DLL::new();
        for n in 0..10 {
            for i in 0..n {
                dll.push(i);
            }
            assert_eq!(dll.len, n);
            for i in 0..n {
                assert_eq!(dll.pop(), Some(n - i - 1));
            }
            assert_eq!(dll.pop(), None);
        }

        let n = 10;
        for i in 0..n {
            dll.push(i);
        }
        assert_eq!(dll.len, n);
        for i in 0..n - 2 {
            assert_eq!(dll.pop(), Some(n - i - 1));
        }
    }
}
