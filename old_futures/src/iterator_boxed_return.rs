use std::collections::HashMap;

use failure::_core::iter::{Filter, Map};

#[derive(Debug)]
struct DataStructure<I> {
    hashdata: HashMap<String, I>,
    vectordata: Vec<I>,
}

impl<I> DataStructure<I> {
    pub fn new() -> DataStructure<I> {
        DataStructure { hashdata: HashMap::new(), vectordata: Vec::new() }
    }

    // This does not work because return of "Box<dyn Iterator<Item=&I>>" equals to Box<dyn Iterator<Item=&I> + static>
    // by default
    // Whereas the Item=&I by does not have static lifetime inferred
    // See SO https://stackoverflow.com/questions/42028470/why-is-adding-a-lifetime-to-a-trait-with-the-plus-operator-iteratoritem-foo
    // for details
    // pub fn build_boxed_half_iterator_refs_broken(&self) -> Box<dyn Iterator<Item=&I>> {
    //     let iterator = IteratorWrapper::new(self.vectordata.iter());
    //     Box::new(iterator)
    // }
}

trait HasBoxedIterator<I> {
    fn build_boxed_half_iterator_refs<'a>(&'a self) -> Box<dyn Iterator<Item = &'a I> + 'a>;
}

impl<I> HasBoxedIterator<I> for DataStructure<I> {
    // I suppose lifetime of "Item = &I" is inferred to 'a, so we don't have to specify that one?
    // However, you might as, why do we return Boxed iterator in a first place? Why don't we
    // just return "impl Iterator<Item = &I>" instead?
    // Because if the signature is part of Trait definiton, in trait it's not currently possible
    // to return values in style of "impl Something". Instead if you want to return a trait based value
    // from method defined in Trait, you have to do it via boxing trait object.
    fn build_boxed_half_iterator_refs<'a>(&'a self) -> Box<dyn Iterator<Item = &'a I> + 'a> {
        let iterator = IteratorWrapper::new(self.vectordata.iter());
        Box::new(iterator)
    }
}

struct IteratorWrapper<I> {
    iter: I
}

impl<I> Iterator for IteratorWrapper<I> where I: Iterator {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<I> IteratorWrapper<I> {
    fn new(iter: I) -> IteratorWrapper<I> {
        IteratorWrapper { iter }
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
    for item in m.build_boxed_half_iterator_refs() {
        println!("Value = {}", item);
    }
}


pub fn run() {
    example_iteratore_datastructure();
}