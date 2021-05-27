use std::collections::HashMap;
use failure::_core::iter::{Filter, Map};

struct Foo {
    data: i32
}

trait Bar {
    fn calc_result(self) -> i32;
}

impl Bar for Foo {
    fn calc_result(self) -> i32{
        self.data
    }
}

// try to comment this out
impl Bar for &Foo {
    fn calc_result(self) -> i32{
        self.data + 1000
    }
}

// this demonstrates that you can have separate trait implementation for T and &T
// Also, you can implement trait for &T if there's no explicit T implementation, it can be also used
// for T. I suppose because the compiler can generate reference from value for himself
// However if you implement trait only for T, it won't work on &T, unless it implements Copy
pub fn run() {
    let f = Foo { data: 231 };
    let result = f.calc_result();

    let f2 = Foo { data: 555 };
    // let f2ref = &f2;
    let result2 = (&f2).calc_result();

    println!("Result = {}", result);
    println!("Result2 = {}", result2);
}