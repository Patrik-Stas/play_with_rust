use crate::closures;

#[derive(Debug)]
struct Foo { x: i32 }

fn non_copiable()
{
    let mut x = Foo { x: 10};

    let y = x;
    // can't borrow x, because value was moved to y and Foo does not implement copy
//    println!("x = {:?}, y = {:?}", x, y);
    let z = &y;
    println!("y = {:?}, *z = {:?}", y, *z);
}

#[derive(Debug, Copy, Clone)]
struct Bar { x: i32 }

fn copiable()
{
    let mut x = Bar {x: 10};

    let mut y = x;
    y.x = 12356;

    println!("x = {:?}, y = {:?}", x, y);
}


fn mut_non_copiable_to_closure()
{
    let mut x = Foo { x: 10};
    {
        println!("x before closure play; x={:?}", x);
        let mut cl = || {
            x.x += 1;
        };
        cl();
        cl();
        cl();
        println!("x after closure call; x={:?}", x);
    }
    println!("x after closure call; after closure died; x={:?}", x);
}


fn mut_copiable_to_closure()
{
    let mut x = Bar { x: 10};
    {
        println!("x before closure play; x={:?}", x);
        let mut cl = || {
            x.x += 1;
        };
        cl();
        cl();
        cl();
        println!("x after closure call; x={:?}", x);
    }
    println!("x after closure call; after closure died; x={:?}", x);
}

fn copiable_to_closure()
{
    let mut x = Bar { x: 10};
    println!("x before closure play; x={:?}", x);
    let cl = || {
        println!("value of x in closure is x={:?}", x)
    };
//    x.x = 123;
//    println!("x modified outside closure, x={:?}", x);
//    cl();
}

fn move_captured_variables()
{
    // `Vec` has non-copy semantics.
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

//     println!("There're {} elements in vec", haystack.len());
    // ^ Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.

    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.
}

fn closures_as_input_parameters()
{
//    Fn: the closure captures by reference (&T)
//    FnMut: the closure captures by mutable reference (&mut T)
//    FnOnce: the closure captures by value (T)

    fn apply<F>(f: F)
    // The closure takes no input and returns nothing.
        where F: FnOnce() {
        // ^ TODO: Try changing this to `Fn` or `FnMut`.

        f();
    }

    // A function which takes a closure and returns an `i32`.
    fn apply_to_3<F>(f: F) -> i32
    // The closure takes an `i32` and returns an `i32`.
        where F: Fn(i32) -> i32 {

        f(3)
    }

    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));

}

fn std_iterator_example()
{
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`.
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}

fn std_iterator_find_example()
{
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`.
    let mut into_iter = vec2.into_iter();

    // A reference to what is yielded is `&&i32`. Destructure to `i32`.
    println!("Find 2 in vec1: {:?}", iter     .find(|x| **x == 2));
    // A reference to what is yielded is `&i32`. Destructure to `i32`.
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));
}

pub fn play_with_copy()
{
//    non_copiable();
//    copiable();
//    mut_non_copiable_to_closure();
//    mut_copiable_to_closure();
//    copiable_to_closure();
//    move_captured_variables()
//    closures_as_input_parameters();
//    std_iterator_example()
//    patrik_test_iter()
    std_iterator_find_example()
}