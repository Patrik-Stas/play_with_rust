// use std::collections::HashMap;
// use failure::_core::iter::{Filter, Map};
//
// #[derive(Debug)]
// struct Something {
//     data: Vec<u32>
// }
//
// struct SomethingIterator<'a> {
//     pos: usize,
//     dataref: &'a Something
// }
//
// impl Iterator for SomethingIterator {
//     type Item = u32;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.pos < self.dataref.data.len() {
//             true => {
//                 val = self.dataref.data[self.pos];
//                 self.pos += 1;
//                 Some(val)
//             }
//             false => None
//         }
//     }
// }
//
// impl IntoIterator for Something {
//     type Item = u32;
//     type IntoIter = SomethingIterator;
//
//     fn into_iter(self) -> Self::IntoIter {
//         SomethingIterator { pos: 0, dataref: &self}
//     }
// }
//
// pub fn iterate_something() {
//     // building custom iterator directly by ourselves
//     let something = Something { data: vec!(1,2,3,4,5,6) };
//     for some in something.into_iter() {
//         println!("some= {}", some);
//     }
// }



pub fn run() {
    // iterate_something()
}