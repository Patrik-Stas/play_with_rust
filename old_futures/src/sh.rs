#![allow(dead_code)]
use std::mem;

struct Point
{
    x: f64,
    y: f64
}

fn origin() -> Point
{
    Point{x: 1.0, y: 2.0}
}

pub fn stack_and_heap()
{
    println!("stack_and_heap");
    let p1 = origin(); // allocates on stack
    let p2 = Box::new (origin()); // allocates on heap, "boxing"

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));


    let p3 =  *p2; // follow the pointer, "unboxing"
    println!("p3.x = {}", p3.x);
}

