// interior-mutability.rs
use std::cell::Cell;

struct Point {
    x: u8,
    y: u8,
//    cached_sum: Cell<Option<u8>>
    cached_broken: Option<u8>
}

//impl Point {
//    fn sum(&self) -> u8 {
//        match self.cached_sum.get() {
//            Some(sum) => {
//                println!("Got from cache: {}", sum);
//                sum
//            },
//            None => {
//                let new_sum = self.x + self.y;
//                self.cached_sum.set(Some(new_sum));
//                println!("Set cache: {}", new_sum);
//                new_sum
//            }
//        }
//    }
//}

impl Point {
    fn sum(&mut self) -> u8 {
        match self.cached_broken.as_ref() {
            Some(&sum) => {
                println!("Got from cache: {}", sum);
                sum
            },
            None => {
                let new_sum = self.x + self.y;
                self.cached_broken = Some(new_sum);
                println!("Set cache: {}", new_sum);
                new_sum
            }
        }
    }
}

pub fn run() {
//    let p = Point { x: 8, y: 9, cached_sum: Cell::new(None) };
//    println!("Summed result: {}", p.sum());
//    println!("Summed result: {}", p.sum());

    // though possible to implement cache on Point without cell, it forced us to make the sum method
    // to take mutable reference to self. As a result, whenever we call p.sum(), the p must be marked
    // as mutable, even though we do not really intend to modify the point "domain" value os of x, y,
    // but merely to get technical optimization to get working - to modify the Point's cache.
    let mut p = Point { x: 8, y: 9, cached_broken: None };
    println!("Summed result: {}", p.sum());
    println!("Summed result: {}", p.sum());
}