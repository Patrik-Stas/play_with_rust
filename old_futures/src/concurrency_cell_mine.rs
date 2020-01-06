use std::cell::Cell;

pub fn take_cell(cell: &Cell<i32>) {
    let mut xtaken = cell.take();
    xtaken = 123;
    cell.set(xtaken);
}

//https://blog.iany.me/2019/02/rust-cell-and-refcell/
pub fn cell_play_take_set()
{
    let x = Cell::new(1);
    let ref_to_x_1 = &x;
    let ref_to_x_2 = &x;

    let mut xtaken = ref_to_x_1.take();
    xtaken = 2;
    ref_to_x_1.set(xtaken);

    take_cell(&ref_to_x_1);


    println!("X is now {}", x.get());
}

use std::cell::RefCell;

pub struct Foo {
    pub number: u8
}

pub fn refcell_play_panics()
{
    let foo_one = RefCell::new(Foo { number: 1 });
    let mut ref_to_foo_1 = foo_one.borrow_mut();
    ref_to_foo_1.number = 2;
//    drop(ref_to_foo_1);

    // unless we drop ref_to_foo_1, we will be getting panic here! Normally rust would not allow
    // us to ever have to mutable references
    // but RefCell is way to go around it and it's up to use ot make sure in runtime,
    // 2 mutable refs will never exist!
    let mut ref_to_foo_2 = foo_one.borrow_mut();
//    ref_to_foo_2.number = 3;
}

pub fn modify_mutable_foo_reference(x: &mut Foo, y: &mut Foo) {
    x.number = 100;
}

pub fn more_mutable_references()
{
    let mut foo_one = Foo { number: 1 };
//    let ref_to_foo_1 = &mut foo_one; // we could do this with mutRef, but cant by default
    // we get error: "cannot borrow as mutable"
//    modify_mutable_foo_reference(&mut foo_one, &mut foo_one); // cant do this
    println!("Foo.number is now {}", foo_one.number);

}

pub fn run() {
    cell_play_take_set();
    more_mutable_references();
//    refcell_play();
}