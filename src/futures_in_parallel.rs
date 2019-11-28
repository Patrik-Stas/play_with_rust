use std::collections::HashMap;
use tokio::prelude::future::{Either, err};
use futures::future::{FutureResult, ok};
use futures::future;
use std::net::TcpStream;
use std::io;
use std::io::prelude::*;
use futures::Future;
use futures::*;


fn return_boxed_future_ok(x: i32) -> Box<dyn Future<Item=i32, Error=i32>> {
    let f = future::ok(()).and_then(move|_| {
        println!("Started executing OK future for {:}", x);
        ok(x)
    });
    Box::new(f)
}


fn return_boxed_future_error(x: i32) -> Box<dyn Future<Item=i32, Error=i32>> {
    let f = future::ok(()).and_then(move|_| {
        println!("Started executing ERROR future for {:}", x);
        err(x)
    });
    Box::new(f)
}

fn map_to_futures(x: Vec<i32>) -> Vec<Box<dyn Future<Item=i32, Error=i32>>>{
    let myfutures = x
        .iter()
        .enumerate()
        .map(|(index, val)| {
            println!("Iter Mapping occurs on index {}.", index);
            return_boxed_future_ok(val.clone())
        }).collect();
    myfutures
}

fn map_to_futures_with_error_on_even(x: Vec<i32>) -> Vec<Box<dyn Future<Item=i32, Error=i32>>>{
    let myfutures = x
        .iter()
        .enumerate()
        .map(|(index, val)| {
            if index % 2 == 0 {
                println!("Iter Mapping occurs on index {}.", index);
                return_boxed_future_ok(val.clone())
            } else {
                return_boxed_future_error(val.clone())
            }

        }).collect();
    myfutures
}


fn generateFuturesOk() -> Vec<Box<dyn Future<Item=i32, Error=i32>>>{
    let some_ints = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17 ,18, 19, 20];
    let mut futures_list = map_to_futures(some_ints);
    futures_list
}

fn generateFuturesMixed() -> Vec<Box<dyn Future<Item=i32, Error=i32>>>{
    let some_ints = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17 ,18, 19, 20];
    let mut futures_list = map_to_futures_with_error_on_even(some_ints);
    futures_list
}

fn approach_parallel_join_all(futures_list: Vec<Box<dyn Future<Item=i32, Error=i32>>>) {
    let big_future = future::join_all(futures_list);
    let result = big_future.wait();
    println!("Resolved join_all future. Result = {:?}", result);
}

fn approach_sequential_loop_and_pop(mut futures_list: Vec<Box<dyn Future<Item=i32, Error=i32>>>) {
    while (futures_list.len() > 0) {
        let f = futures_list.pop();
        let result = f.wait();
        println!("Future result. Result = {:?}", result);
    }
}

fn approach_parellel_into_iter(mut futures_list: Vec<Box<dyn Future<Item=i32, Error=i32>>>) {
    println!("approach_parellel_into_iter >> ");
//    Map<IntoIter<Box<>>>
    let res = futures_list.into_iter().map(|one_future| {
        let m = one_future.wait();
        m
    });
    // Note: Important! Map is lazy, hence If we don't call collect, the map closures are never evaluated, futures won't be executed!
    // Since all futures are executed once collect() is called, the futures are running in parallel
    let res: Vec<Result<i32, i32>> = res.collect();
    println!("approach_parellel_into_iter >> result = {:?}", res);
}

fn approach_sequential_for_loop(mut futures_list: Vec<Box<dyn Future<Item=i32, Error=i32>>>) {
    println!("approach_sequential_for_loop >> ");
    for one_future in futures_list {
        let one_result = one_future.wait();
        println!("Future result = {:?}", one_result);
    }
}

fn approach_stream(mut futures_list: Vec<Box<dyn Future<Item=i32, Error=i32>>>) {
    //    let s = futures::stream::futures_unordered(futuresList);
//    let stream = stream::iter_ok(futuresList);
//    let future = stream
//        .buffer_unordered(5)
//        .map_err(|err| {
//            error!("Error occured on future. Err = {:?}", err);
//        })
//        .for_each(|n| {
//            info!("Resolved future. Wallet handle = {:?}", n);
//            Ok(())
//        });

}




pub fn run()
{
//     TODO : have list of u32, map them into futures and collect. How can I collect without having them executed implicitly. I want to buffer them and limit parallelization
//    let f = return_boxed_future(123);
//    let res = f.wait().expect("future failed!");
//    println!("result = {}", res);

//    let futures_list_ok = generateFuturesOk();
    let futures_list = generateFuturesMixed();
//    let futures_list = generateFuturesOk();
//    approach_parallel_join_all(futures_list);
//    approach_sequential_loop_and_pop(futures_list);
//    approach_parellel_into_iter(futures_list);
    approach_sequential_for_loop(futures_list);


//    let m = futures_list.wait();
//    println!("m = {:?}", m);
//    let ff = future::join_all(futures_list);
//    ff.then(|x| {
//        println!(x)
//    });

//    let myfutures = vec![
//        finished::<u32, u32>(1),
//        finished::<u32, u32>(2),
//        finished::<u32, u32>(3),
//        finished::<u32, u32>(4),
//    ];
//    println!("Running: future in parallel");
//    let f = collect(myfutures);
//    let f = myfutures.iter().collect();
//    println!("before f.wait()' f == {:?}", &f);
//    let result = f.wait();
//    println!("after f.wait()' f == {:?}", &result);
//    println!("collected vector of futures {:?}", result);
//    let f = f.map(|x| {
//        println!("map f; x={:?}", x);
//        assert_eq!(x, [1, 2, 3]);
//    });
//
//    let f = collect(vec![
//        finished::<u32, u32>(1),
//        failed::<u32, u32>(2),
//        finished::<u32, u32>(3),
//    ]);
//    let f = f.then(|x| {
//        assert_eq!(x, Err(2));
//        x
//    });
//    let f = f.map(|x| {
//
//        assert_eq!(x, [1, 2, 3]);
//    });
}
