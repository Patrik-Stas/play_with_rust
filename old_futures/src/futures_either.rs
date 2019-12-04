use std::collections::HashMap;
use tokio::prelude::future::{Either, err};
use futures::future::FutureResult;
use futures::future;
use std::net::TcpStream;
use std::io;
use std::io::prelude::*;
use futures::Future;


//pub fn my_async_code(number:i32) -> Future<Item = i32, Error = failure::Error> {
//pub fn my_async_code(number:i32) -> FutureResult<i32, failure::Error> {

fn f() -> impl Future<Item=usize, Error=()> {
    if 1 > 0 {
        let f1 = future::ok(2).map(|x| x);
        Either::A(f1)
    } else {
        let f2 = future::ok(10).and_then(|x| future::ok(x + 2));
        Either::B(f2)
    }
}


// How to i rewrite this so that I return future rather than futureResult?
// What is difference between future and futureResult?
// how can I keep using futureResult and do mapping in one if branch, but not in the other?
fn my_async_code(number: i32) -> impl Future<Item=i32, Error=()> {
//    ok(number)
    if (number > 100) {
        let f1 = future::ok(number + 100).and_then(|m| { future::ok(m * 100)});
//        let f1 = future::ok(2).map(|x| x);
        Either::A(f1)
    } else {
        let f2 = future::ok(number + 10).map(|n| n);
//        let f2 = future::ok(10).and_then(|x| future::ok(x + 2));
        Either::B(f2)
    }
}



pub fn testMine() {
//    let myfuture =
//        ok(())
//            .and_then(|_| ok(123))
//            .and_then(|res| {
//                println!("{:?}", res);
//                ok(())
//            })
//            .and_then(|_| my_async_code(-321))
//            .map_err(|err| println!("It went bad!! {:?}", err))
//            .and_then(|res| {
//                println!("Result of my async code {:?}", res);
//                ok(())
//            });
//    tokio::run(myfuture);
}

pub fn testFromStakOverflow() {
    let the_future = f().map(|result| {
        println!("Future returned result = {}", result);
        ()
    });
    tokio::run(the_future);
}

pub fn run()
{
    testMine()
//    testFromStakOverflow();
}
