extern crate futures_cpupool;

use futures_cpupool::CpuPool;
use std::collections::HashMap;
use futures::future::{FutureResult, ok, err, Either};
use futures::future;
use std::net::TcpStream;
use std::{io, thread};
use std::io::prelude::*;
use futures::Future;
use futures::*;
use std::time::Duration;
use std::rc::Rc;
use tokio_core::reactor::Core;


fn return_boxed_future_on_threadpool(cpu_pool: &CpuPool, x: i32) -> Box<dyn Future<Item=i32, Error=i32>> {
    let f = cpu_pool.spawn_fn(move || {
        println!("Started executing OK CPU future for {:}", x);
        thread::sleep(Duration::from_millis(2000));
        println!("Finishing executing OK CPU future for {:}", x);
        Ok(x)
    });
    Box::new(f)
}

fn return_boxed_future_ok(x: i32) -> Box<dyn Future<Item=i32, Error=i32>> {
    let f = future::ok(()).and_then(move |_| {
        println!("Started executing OK future for {:}", x);
        ok(x)
    });
    Box::new(f)
}

fn return_boxed_future_error(x: i32) -> Box<dyn Future<Item=i32, Error=i32>> {
    let f = future::ok(()).and_then(move |_| {
        println!("Started executing ERROR future for {:}", x);
        err(x)
    });
    Box::new(f)
}

fn retrun_box_future_ok_on_even(x: i32) -> Box<dyn Future<Item=i32, Error=i32>> {
    let f = match x % 2 {
        0 => Either::A(future::ok(()).and_then(move |_| {
            println!("Started executing OK future for {:}", x);
            ok(x)
        })),
        _ => Either::B(future::ok(()).and_then(move |_| {
            println!("Started executing ERROR future for {:}", x);
            err(x)
        }))
    };
    Box::new(f)
}

// generating vectors of futures
fn map_to_futures(x: Vec<i32>, create_future: fn(i32) -> Box<dyn Future<Item=i32, Error=i32>>) -> Vec<Box<dyn Future<Item=i32, Error=i32>>> {
    let myfutures = x
        .iter()
        .enumerate()
        .map(|(index, val)| {
            println!("Iter Mapping occurs on index {}.", index);
            create_future(val.clone())
        }).collect();
    myfutures
}

fn generateFuturesOk() -> Vec<Box<dyn Future<Item=i32, Error=i32>>> {
    let some_ints = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let mut futures_list = map_to_futures(some_ints, return_boxed_future_ok);
    futures_list
}

fn generateFuturesMixed() -> Vec<Box<dyn Future<Item=i32, Error=i32>>> {
    let some_ints = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let mut futures_list = map_to_futures(some_ints, retrun_box_future_ok_on_even);
    futures_list
}

//fn generate_stream() {
//    let nums = stream::iter_ok::<_, ()>(1..100)
//        .and_then(move |n| {
//            println!("producing future {}", n);
//            let future = pool.spawn_fn(move || {
//                thread::sleep(Duration::from_millis(1000));
//                Ok(n)
//            });
//            future
//        }).collect();
//    nums
//}

// Approaches to resolution of vectors of futures
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

//fn approach_stream(mut futures_list: Vec<Box<dyn Future<Item=i32, Error=i32>>>) {
//    let stream_unordered = futures::stream::futures_unordered(futures_list).into_future();
////    let stream = stream_unordered.into_buf();
//    let stream2 = futures::stream::buffer_unordered::new(stream_unordered, 23);
//
////    let stream = stream::iter_ok(unordered);
//    let future = stream2
//        .map_err(|err| {
//            println!("Error occured on future. Err = {:?}", err);
//        })
//        .for_each(|n| {
//            println!("Resolved future. Wallet handle = {:?}", n);
//            Ok(())
//        });
//}

fn experiment_single_future() {
    let f = return_boxed_future_ok(123);
    let res = f.wait().expect("future failed!");
    println!("result = {}", res);
}

fn experiment_threadpool() {
//    let mut core = Core::new().unwrap();
    let cpu_pool = CpuPool::new(10);

    let f1 = return_boxed_future_on_threadpool(&cpu_pool, 1);
    let f2 = return_boxed_future_on_threadpool(&cpu_pool, 2);
    let f3 = return_boxed_future_on_threadpool(&cpu_pool, 3);
    let fs = vec![f1, f2, f3];
    approach_parallel_join_all(fs)
}

fn experiment_future_list_approaches() {
//    let futures_list_ok = generateFuturesOk();
    let futures_list = generateFuturesMixed();
//    let futures_list = generateFuturesOk();
//    approach_parallel_join_all(futures_list);
//    approach_sequential_loop_and_pop(futures_list);
    approach_parellel_into_iter(futures_list);
//    approach_sequential_for_loop(futures_list);
//    approach_stream(futures_list);
}

fn experiment_range_to_iter() -> Vec<String> {
//    let v: Vec<i32> = (1000..1010).collect();
    let range = (1000..1010);
    let vector_no: Vec<i32> = range.clone().collect();
    let vector_str: Vec<String> = vector_no.iter().enumerate().map(|(k, v)| {
        format!("k={} v={}", k, v)
    }).collect();
    println!("From range {:?} generated vector {:?} remapped into vector {:?}", &range, &vector_no, &vector_str);
    vector_str
}

fn experiment_simple_stream() {
    let mut stream = stream::iter(vec![Ok(17), Err(false), Ok(19)]);
    assert_eq!(Ok(Async::Ready(Some(17))), stream.poll());
    assert_eq!(Err(false), stream.poll());
    assert_eq!(Ok(Async::Ready(Some(19))), stream.poll());
    assert_eq!(Ok(Async::Ready(None)), stream.poll());
}

fn experiment_simple_stream2() {
//    stream::iter_ok(0..10).for_each(|i| Ok(()));
    let mut stream = stream::iter_ok::<_, ()>(vec![17, 19]);
    stream.for_each(|res| {
        println!("Processing stream value {:?}", res);
        Ok(())
    }).wait();
}

fn experiment_simple_stream3() {
    let f1: FutureResult<FutureResult<i32, ()>, ()> = future::ok(future::ok(1));
    let f2: FutureResult<FutureResult<i32, ()>, ()> = future::ok(future::ok(2));
    let f3: FutureResult<FutureResult<i32, ()>, ()> = future::ok(future::ok(3));
    let f4: FutureResult<FutureResult<i32, ()>, ()> = future::ok(future::ok(4));

    let stream = stream::futures_unordered(vec![
        f1,
        f2,
        f3,
        f4
    ]);

    let mut next = stream.buffer_unordered(10).into_future(); // (1)
    let mut core = Core::new().unwrap();
    loop {
        match core.run(next) {
            Ok((None, _something)) => {
                println!("finished");
                break;
            }
            Ok((Some(response), mut next_requests)) => {
                {
                    let inner = next_requests.get_mut();
                    println!("{:?}", response);
                }
                next = next_requests.into_future();
            }
            Err((error, next_requests)) => {
                println!("error: {:?}", error);
                next = next_requests.into_future();
            }
        }
    }
}




pub fn run()
{
//    experiment_threadpool()
//    experiment_single_future()
//    experiment_future_list_approaches()
//    experiment_range_to_iter();
//    experiment_simple_stream();
//    experiment_simple_stream2();
    experiment_simple_stream3();


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
