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


pub fn example_struct_returning_wrapping_iterator() {
    #[derive(Debug)]
    struct DataStructure<I> {
        hashdata: HashMap<String, I>,
        vectordata: Vec<I>
    }

    // impl<I> DataStructure<I> where I : Clone {
    impl<I> DataStructure<I> {
        pub fn new() -> DataStructure<I> {
            DataStructure { hashdata: HashMap::new(), vectordata: Vec::new() }
        }
        pub fn return_reference_iterator_impl(&self) -> impl Iterator<Item = &I> {
            let iterator = IteratorWrapper::new(self.hashdata.values().take(2));
            iterator
        }
        // pub fn return_box<'b>(&'b self) -> Box<dyn Iterator<Item = I>+ 'b> {
        //     let v = vec!(1,2,3,4);
        //     Box::new(self.vectordata.into_iter())
        // }

        pub fn box_allocated_data(& self) -> Box<dyn Iterator<Item = i32>> {
            let v = vec!(1,2,3,4);
            Box::new(v.into_iter())
        }

        // pub fn build_boxed_half_iterator_src_cloned(&self) -> Box<dyn Iterator<Item = I>> {
        //     let iterator = IteratorWrapper::new(self.vectordata.clone().into_iter());
        //     Box::new(iterator)
        // }

        // using cloned() on iter(), so only works if I: Clone
        // pub fn build_boxed_half_iterator_cloned_vals<'b>(&'b self) -> Box<dyn Iterator<Item = I> + 'b> {
        //     let iterator = IteratorWrapper::new(self.vectordata.iter().cloned());
        //     Box::new(iterator)
        // }

        // does not need clone!
        pub fn build_boxed_half_iterator_refs<'b>(&'b self) -> Box<dyn Iterator<Item = &I> + 'b> {
            let iterator = IteratorWrapper::new(self.vectordata.iter());
            Box::new(iterator)
        }
    }

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

    let mut m = DataStructure::<i32>::new();
    m.hashdata.insert("Hello".into(), 12);
    m.hashdata.insert("hi".into(), 23);
    m.hashdata.insert("adios".into(), 34);
    m.hashdata.insert("bye".into(), 45);
    println!("m = {:?}", m);

    let mut balances: HashMap<String, u32> = HashMap::new();
    balances.insert("Daniel".to_string(), 1);
    balances.insert("Ashley".to_string(), 2);
    balances.insert("Katie".to_string(), 3);
    balances.insert("Robert".to_string(), 4);

    let iter_wrapper = IteratorWrapper::new(balances.iter().take(2));
    for item in iter_wrapper {
        println!("item={:?}", item);
    }
}


pub fn run() {
    // example_take();
    // example_hashmap_iteration();
    // example_iterator_wrapper();
    example_struct_returning_wrapping_iterator()
    // example_hashmap_iterator_wrapper();

}