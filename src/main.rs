mod dll;
mod dll_raw;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    #[test]
    fn it_works() {
        let mut dll = LinkedList::new();
        for n in 0..20_000 {
            for i in 0..n {
                dll.push_back(i);
            }
            assert_eq!(dll.len(), n);
            for i in 0..n {
                assert_eq!(dll.pop_back(), Some(n - i - 1));
            }
            assert_eq!(dll.pop_back(), None);
        }
    }
}
