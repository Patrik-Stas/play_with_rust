use std::collections::HashMap;
use failure::_core::iter::{Filter, Map};

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

    pub fn build_ref_values_iterator(&self, skip: usize, take: usize) -> impl Iterator<Item = &I> {
        IteratorWrapper::new(self.hashdata.values().skip(skip).take(take))
    }

    pub fn build_ref_keypair_iterator(&self, skip: usize, take: usize) -> impl Iterator<Item = (&String, &I)> {
        IteratorWrapper::new(self.hashdata.iter().skip(skip).take(take))
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


pub fn example_use_iterator_directly() {
    println!("\nIteratorWrapper iterating string");
    for item in IteratorWrapper::new("abcdefghijkl".chars()) {
        println!("item={}", item);
    }

    println!("\nIteratorWrapper iterating first 2 (key,val) pairs of HashMap.");
    let mut balances: HashMap<String, u32> = HashMap::new();
    balances.insert("Daniel".to_string(), 1);
    balances.insert("Ashley".to_string(), 2);
    balances.insert("Katie".to_string(), 3);
    balances.insert("Robert".to_string(), 4);
    let iter_wrapper = IteratorWrapper::new(balances.iter().take(2));
    for item in iter_wrapper {
        println!("Pair = {:?}", item);
    }
}

pub fn example_iteratore_datastructure() {
    let mut m = DataStructure::<i32>::new();
    m.hashdata.insert("Hello".into(), 12);
    m.hashdata.insert("hi".into(), 23);
    m.hashdata.insert("adios".into(), 34);
    m.hashdata.insert("bye".into(), 45);
    m.hashdata.insert("hola".into(), 55);
    println!("\nIterating over (1..3) values returned from DataStructure<i32>");
    for item in m.build_ref_values_iterator(1, 3) {
        println!("Value = {}", item);
    }
    println!("\nIterating over (1..3) keypairs returned from DataStructure<i32>");
    for item in m.build_ref_keypair_iterator(2, 4) {
        println!("Value = {:?}", item);
    }
}


pub fn run() {
    example_use_iterator_directly();
    example_iteratore_datastructure();

}