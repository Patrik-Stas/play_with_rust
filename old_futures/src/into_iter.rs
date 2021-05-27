use std::collections::HashMap;
use failure::_core::iter::{Filter, Map};
use failure::_core::fmt::Debug;

#[derive(Debug)]
struct Something {
    data: Vec<u32>
}

struct SomethingIterator<'a> {
    pos: usize,
    dataref: &'a Something
}

impl <'a>Iterator for SomethingIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pos < self.dataref.data.len() {
            true => {
                let val = self.dataref.data[self.pos];
                self.pos += 1;
                Some(val)
            }
            false => None
        }
    }
}

// it seems that only way to create intoIterator for something is by implementing it on
// Something's reference? Because SomethingIterator type requires lifetime parameter, but
// if we try to implement this on Someting's value rather than reference, there's nothing to
// to bound the 'a lifetime to?
// But if we do it as below, we can tie up iterator's lifetime with Something's reference....
// is that so?
impl<'a> IntoIterator for &'a Something {
    type Item = u32;
    type IntoIter = SomethingIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SomethingIterator { pos: 0, dataref: self}
    }
}

fn move_out(param: impl Iterator<Item=u32>) {
    for val in param {
        println!("val = {:?}", val);
    }
}

// following doesn't compile because by SomethingIterator definition, its lifetime
// is bounded to the lifetime of struct it's referring to. So we are not allowed to use
// the iterator after the data it refers to doesn't exist anymore.
// fn build_iterator_manually_see_lifetime_in_action() {
//     let mut iter;
//     {
//         let something = Something { data: vec!(1, 2, 3, 4, 5, 6) };
//         iter = SomethingIterator { pos: 0, dataref: &something };
//     }
//     move_out(iter);
// }

fn build_iterator_manually() {
    let something = Something { data: vec!(1, 2, 3, 4, 5, 6) };
    let iter = SomethingIterator { pos: 0, dataref: &something };
    for val in iter {
        println!("val = {:?}", val);
    }
}

fn use_into_iterator() {
    let something = Something { data: vec!(1, 2, 3, 4, 5, 6) };
    for val in something.into_iter() {
        println!("val = {:?}", val);
    }
}

pub fn iterate_something() {
    // use_into_iterator();
    // build_iterator_manually_see_lifetime_in_action();
    // build_iterator_manually();
    use_into_iterator();
}



pub fn run() {
    iterate_something()
}