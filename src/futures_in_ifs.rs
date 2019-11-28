use std::collections::HashMap;
use tokio::prelude::Future;
use tokio::prelude::future::{ok, Either, err};
use futures::future::FutureResult;


//pub fn my_async_code(number:i32) -> Future<Item = i32, Error = failure::Error> {
//pub fn my_async_code(number:i32) -> FutureResult<i32, failure::Error> {

// How to i rewrite this so that I return future rather than futureResult?
// What is difference between future and futureResult?
// how can I keep using futureResult and do mapping in one if branch, but not in the other?
pub fn my_async_code(number: i32) -> FutureResult<i32, String> {
//    ok(number)
    if (number > 100) {
        ok(number + 100)
//            .and_then(|m| {
//                ok(number*100)
//            })
    } else if (number > 0) {
        ok(number + 10)
//            .map(|n| {
//                ok(n)
//            })
    } else {
        err("You can't use ngative numbers here!".to_string())
    }
}

pub fn run()
{
//    let myfuture = ok(1)
//        .and_then(|_| {
//            println!("Hello world!")
//        });
    let myfuture =
        ok(())
            .and_then(|_| ok(123))
            .and_then(|res| {
                println!("{:?}", res);
                ok(())
            })
            .and_then(|_| my_async_code(-321))
            .map_err(|err| println!("It went bad!! {:?}", err))
            .and_then(|res| {
                println!("Result of my async code {:?}", res);
                ok(())
            });
    tokio::run(myfuture);
}