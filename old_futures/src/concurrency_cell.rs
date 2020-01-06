use std::cell::Cell;

// https://learning.oreilly.com/library/view/mastering-rust/9781785885303/b9b96c8d-f19b-4923-bbd0-abb4d6e0d071.xhtml
pub fn run() {
    let x = Cell::new(1);
    let ref_to_x_1 = &x;
    let ref_to_x_2 = &x;

    ref_to_x_1.set(ref_to_x_1.get() + 1);
    ref_to_x_2.set(ref_to_x_2.get() + 1);

    println!("X is now {}", x.get());
}