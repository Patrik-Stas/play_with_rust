use std::collections::HashMap;
use failure::_core::iter::{Filter, Map};

struct Empty;

struct PatrikList {
    main: Vec<u32>
}

pub struct PatrikHash {
    pub datahash: HashMap<String, u32>,
    pub datavector: Vec<u32>
}

struct DivisableIterator {
    // data: Box<[u32]>, // how to declare slice
    data: Vec<u32>, // how to declare slice
    position: usize,
    divisable_by: u32,
}

struct PatrikHashValueIterator {
    iterator: Box<dyn Iterator<Item = u32>>
}

impl PatrikHash {
    // pub fn get_patrikhash_iterator(&mut self) -> PatrikHashValueIterator {
    //     return PatrikHashValueIterator { iterator: Box::new(self.datavector.iter()) }
    // }
}

impl Iterator for PatrikHashValueIterator {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

impl Iterator for DivisableIterator {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        while self.position < self.data.len() {
            let val = self.data[self.position];
            if val % self.divisable_by == 0 {
                self.position = self.position + 1;
                return Some(val);
            }
            self.position = self.position + 1;
        }
        None
    }
}

impl PatrikList {
    fn getDivisableIterator(&self, divisable_by: u32) -> DivisableIterator {
        DivisableIterator { divisable_by, data: self.main.clone(), position: 0 }
    }
}

impl IntoIterator for PatrikList {
    type Item = u32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.main.into_iter()
    }
}

pub fn divisable_iterator_demo() {
    let m = PatrikList { main: vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20) };
    for item in m.getDivisableIterator(4) {
        println!("Item = {}", item);
    }
}

pub fn example_take() {
    let n = vec!(1,2,3,4,5,6,7,8,9,10);
    let iterator = n.iter();

    for x in n.iter().take(5) {
        println!("x = {}", x);
    }
}

pub fn example_hashmap_iteration() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    for (key, val) in contacts.iter() {
        println!("key={} val={}", key, val);
    }

    for (key, val) in contacts.iter().take(2) {
        println!("key={} val={}", key, val);
    }
}

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

fn foo() -> impl Iterator {
    vec![1, 2, 3]
        .into_iter()
        .map(|x| x + 1)
        .filter(|x| x % 2 == 0)
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




pub fn example_hashmap_iterator_wrapper() {
    // let mut balances: HashMap<String, u32> = HashMap::new();
    //
    // balances.insert("Daniel".to_string(), 1);
    // balances.insert("Ashley".to_string(), 2);
    // balances.insert("Katie".to_string(), 3);
    // balances.insert("Robert".to_string(), 4);
    //
    // let pathhash = PatrikHash {datavector: vec!(1,2,3,4,5), datahash: balances };
    // pathhash.get
}


pub fn run() {
    // divisable_iterator_demo();
    // example_take();
    // example_hashmap_iteration();
    // example_iterator_wrapper();
    example_struct_returning_wrapping_iterator()
    // example_hashmap_iterator_wrapper();

}