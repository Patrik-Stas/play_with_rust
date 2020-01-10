#[derive(Clone, Copy)]
struct Point { x: i32, y: i32 }

pub fn run() {
    let c = 'Q';

    // A `ref` borrow on the left side of an assignment is equivalent to
    // an `&` borrow on the right side.
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // `ref` is also valid when destructuring a struct.
    let _copy_of_x = {
        // `ref_to_x` is a reference to the `x` field of `point`.
        let Point { x: ref ref_to_x, y: _ } = point;

        // Return a copy of the `x` field of `point`.
        *ref_to_x
    };

    // A mutable copy of `point`
    let mut mutable_point = point;
    println!("original copy of mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    {
        // `ref` can be paired with `mut` to take mutable references.
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // Mutate the `y` field of `mutable_point` via a mutable reference.
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // A mutable tuple that includes a pointer
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // Destructure `mutable_tuple` to change the value of `last`.
        let (_, ref mut last) = mutable_tuple;
        // it's important we use _ at the beginning and not proper variable name - otherwise
        // we would be attempting to move out Box, the first element of tuple
        // if we change Box::new(5u32) to 5u32, we don't have to worry about moving anymore, because
        // u32 is Copy (trait)

        *last = 2u32;
        // if we wouldn't destructure the 'last' as reference, but value, it would make copy of u32
        // and hence we wouldnt really modify mutable_tuple itself
    }

    println!("tuple is {:?}", mutable_tuple);

}
