fn factory() -> Box<Fn(i32) -> i32> {
    let num = 5;

    Box::new(move |x| x + num)
}

fn create_vector_modifier(m: i32) -> Box<Fn(&mut Vec<i32>) -> ()>
{
    let c = move |v: &mut Vec<i32>| {
//        v.into_iter().map(|x| x * m ).collect();
        for i in v {
            *i *= m;
        }

    };
    Box::new(c)
}

fn test_create_vector_modified()
{
    let f = factory();

    let answer = f(1);
    assert_eq!(6, answer);

    let mut v:Vec<i32> = vec!(1,2,3,4,5,6,7,8,9,10);
    let modify = create_vector_modifier(10);
    modify(&mut v);
    println!("Vector after modification is {:?}", v);
    create_vector_modifier(10)(&mut v);
    println!("Veotr after next midification {:?}", v)
}

fn sample1()
{
    // Increment via closures and functions.
    fn  function            (i: i32) -> i32 { i + 1 }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}

fn sample_capturing()
{
    use std::mem;

    let mut color = "green";

    // A closure to print `color` which immediately borrows (`&`)
    // `color` and stores the borrow and closure in the `print`
    // variable. It will remain borrowed until `print` goes out of
    // scope. `println!` only requires `by reference` so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();
    print();

    let mut count = 0;

    // A closure to increment `count` could take either `&mut count`
    // or `count` but `&mut count` is less restrictive so it takes
    // that. Immediately borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside.
    // Thus, calling the closure mutates the closure which requires
    // a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure.
    inc();
    inc();

    let _reborrow = &mut count;
//    inc();
    // ^ TODO: try uncommenting this line.

    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    //consume();
    // ^ TODO: Try uncommenting this line.
}

pub fn play_with_closures()
{
//    test_create_vector_modified()
//    sample1()
    sample_capturing()
}


