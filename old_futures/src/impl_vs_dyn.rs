use std::collections::HashMap;
use failure::_core::iter::{Filter, Map};
use std::fmt::Debug;

fn return_iterator_i32() -> impl Iterator<Item=i32> {
    vec![1, 2, 3]
        .into_iter()
        .map(|x| x + 1)
        .filter(|x| x % 2 == 0)
}

fn return_iterator_i32_2() -> impl Iterator<Item=i32> {
    vec![4, 5, 6].into_iter()
}

fn return_iterator_string() -> impl Iterator<Item=String> {
    vec!["foo".to_string(), "bar".to_string()]
        .into_iter()
}

fn accept_impl(iterator: impl Iterator) {
    for val in iterator {
        println!("Value ..");
    }
}

/**
 return different types, won't compile
 */
// fn return_iterator_i32_or_string(is_string: bool) -> impl Iterator {
//     match is_string {
//         true => return_iterator_i32(),
//         false => return_iterator_string()
//     }
// }


/**
return different types, won't compile
altough seemingly same return types "impl Iterator<Item=i32>" returned from both function
we call here, every "impl ..." from different function is treated as a unique "opaque" type,
so from rustc point of view, we are trying to return 2 different types from this function
*/
// fn return_iterator_i32_two_branches(foo: bool) -> impl Iterator {
//     match foo {
//         true => return_iterator_i32(),
//         false => return_iterator_i32_2()
//     }
// }


pub fn run() {
    let iterator_i32 = return_iterator_i32();
    let iterator_string = return_iterator_string();
    // for foo in foo_iterator {
    //     println!("foo = {:?}", foo);
    // }

    // one function taking impl argument can receive many different actual types
    accept_impl(iterator_i32);
    accept_impl(iterator_string);
    accept_impl([1.0, 2.0].iter());

    // let some_iterator = return_iterator_i32_or_string(true);
    // return_iterator_i32_two_branches(true);
}