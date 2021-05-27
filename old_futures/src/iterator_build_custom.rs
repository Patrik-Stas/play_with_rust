use std::collections::HashMap;
use failure::_core::iter::{Filter, Map};

#[derive(Debug)]
struct DataStructure<I> {
    hashdata: HashMap<String, I>,
    vectordata: Vec<I>
}

impl<I> DataStructure<I> where I : Clone {
// impl<I> DataStructure<I>  {
    pub fn new() -> DataStructure<I> {
        DataStructure { hashdata: HashMap::new(), vectordata: Vec::new() }
    }
    pub fn return_hashvals_impl_iterator_itemref(&self) -> impl Iterator<Item = &I> {
        let iterator = IteratorWrapper::new(self.hashdata.values().take(2));
        iterator
    }

    // does not need clone!
    // without lifetime specification we get error that IteratorWrapper creation is capturing data
    // with lifetime same as &self has whilst saying Box::new requires static lifetime.
    // Why is this and what are the implicit lifetimes like?
    pub fn return_vec_boxed_trait_iterator_itemref<'b>(&'b self) -> Box<dyn Iterator<Item = &I> + 'b> {
        let iterator = IteratorWrapper::new(self.vectordata.iter());
        Box::new(iterator)
    }

    pub fn return_vec_impl_iterator_mut(&mut self) -> impl Iterator<Item = &mut I> {
        return self.vectordata.iter_mut();
    }

    // using cloned() on iter(), so only works if: I : Clone
    pub fn return_vec_boxed_trait_iterator_item_cloned<'b>(&'b self) -> Box<dyn Iterator<Item = I> + 'b> {
        let iterator = IteratorWrapper::new(self.vectordata.iter().cloned());
        Box::new(iterator)
    }

    // using clone on Vec<I>, so it mustbe that: I : Clone
    pub fn return_vec_boxed_trait_iterator_cloned_movedout<'b>(&'b self) -> Box<dyn Iterator<Item = I>+ 'b> {
        let iterator = IteratorWrapper::new(self.vectordata.clone().into_iter());
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

pub fn example_struct_returning_wrapping_iterator() {
    // building custom iterator directly by ourselves
    for item in IteratorWrapper::new("abcdefghijkl".chars().take(5)) {
        println!("item={}", item);
    }

    let mut m = DataStructure::<i32>::new();
    m.hashdata.insert("Hello".into(), 12);
    m.hashdata.insert("hi".into(), 23);
    m.hashdata.insert("adios".into(), 34);
    m.hashdata.insert("bye".into(), 45);
    m.vectordata.push(1);
    m.vectordata.push(20);
    m.vectordata.push(300);
    m.vectordata.push(4000);

    let it = m.return_hashvals_impl_iterator_itemref();
    for val in it {
        println!("Hashmap value = {}", val);
    }

    let it2 = m.return_vec_boxed_trait_iterator_cloned_movedout();
    for val in it2 {
        println!("Moved out vector values = {}", val);
    }

    let it3 = m.return_vec_boxed_trait_iterator_itemref();
    for val in it3 {
        println!("Iterated vector references = {}", val);
    }

    let it4_mut = m.return_vec_impl_iterator_mut();
    for mut val in it4_mut {
        *val += 1;
        println!("Iterating mutable val {}", val);
    }

}



pub fn run() {
    example_struct_returning_wrapping_iterator()
}