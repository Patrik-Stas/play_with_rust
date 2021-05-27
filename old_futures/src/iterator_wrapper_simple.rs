use std::collections::HashMap;
use failure::_core::iter::{Filter, Map};

pub fn example_iterator_wrapper() {
    struct IteratorWrapper<I> {
        iter: I
    }

    impl<I> Iterator for IteratorWrapper<I> where I: Iterator {
        type Item =  I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next()
        }
    }

    impl<I> IteratorWrapper<I> {
        fn new(iter: I) -> IteratorWrapper<I> {
            IteratorWrapper { iter }
        }
    }

    for item in IteratorWrapper::new("abcdefghijkl".chars()) {
        println!("item={}", item);
    }

    let mut balances: HashMap<String, u32> = HashMap::new();

    balances.insert("Daniel".to_string(), 1);
    balances.insert("Ashley".to_string(), 2);
    balances.insert("Katie".to_string(), 3);
    balances.insert("Robert".to_string(), 4);

    for item in IteratorWrapper::new(balances.iter().take(2)) {
        println!("item={:?}", item);
    }
}


pub fn run() {
    example_iterator_wrapper();
}