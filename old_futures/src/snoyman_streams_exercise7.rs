extern crate tokio;
extern crate tokio_timer;

use std::time::{Duration, Instant};
use tokio::prelude::*;
use crate::ExpensiveFuture::ExpensiveFuture;


pub fn create_iterator(how_many: i32) -> Box<dyn Iterator<Item=i32>> {
    Box::new((0..how_many).map(|m| m * 2))
}

pub fn create_vector(how_many: i32) -> Vec<i32> {
    (0..how_many).map(|m| m * 2).collect()
}

//pub fn expensive_operation(how_many: i32) -> Vec<Box<dyn Future<Item=(), Error=()>>> {
//    let mapped = (0..how_many)
//        .map(|i| {
//            let f = futures::future::ok(())
//                .and_then(|_| {
//                    println!("Starting expensive stuff");
//                    future::ok(())
//                }).and_then(|_| {
//                    tokio_timer::sleep(Duration::new(1, 0)).map_err(|e| ())
//            }).and_then(|_| {
//                println!("Finished expensive stuff");
//                future::ok(())
//            });
//            Box::new(f)
//        })
//        .collect();
//    mapped
//}

//pub fn expensive_operation(how_many: i32) {
//    let mapped: Vec<Box<dyn Future<Item=(), Error=()>>> = (0..how_many)
//        .map(|i| {
//            let f = futures::future::ok(());
//            Box::new(f)
//        })
//        .collect();
//}



//pub fn expensive_operations_2(how_many: i32) {
//    let mapped: Vec<Box<dyn Future<Item=i32, Error=()>>> = (0..how_many)
//        .map(|i| {
//            Box::new(futures::future::ok(132))
//        })
//        .collect();
//}

pub fn collect_bunch_of_integers(how_many: i32) {
    let mapped: Vec<Box<i32>> = (0..how_many)
        .map(|i| {
            Box::new(666i32)
        })
        .collect();
}

pub fn create_future_cheap_send() -> Box<dyn Future<Item=(), Error=()> + Send> {
    let f = future::ok(())
        .and_then(|_| {
            println!("Starting cheap stuff");
            future::ok(())
        }).and_then(|_| {
            println!("Finished cheap stuff");
            future::ok(())
    });
    Box::new(f)
}

pub fn create_future_cheap() -> Box<dyn Future<Item=(), Error=()>> {
    let f = future::ok(())
        .and_then(|_| {
            println!("Started cheap Future");
            future::ok(())
        }).and_then(|_| {
            println!("Finished cheap Future");
            future::ok(())
    });
    Box::new(f)
}

pub fn create_future_expensive_send(id: u32) -> Box<dyn Future<Item=(), Error=()> + Send> {
    let f = future::ok(())
        .and_then(|_| {
            println!("Started expensive Future+Send");
            future::ok(())
        }).and_then(move |_| {
            tokio_timer::sleep(Duration::new(1, 0)).map_err(|e| ())
        }).and_then(|_| {
            println!("Finished expensive Future+Send");
            future::ok(())
    });
    Box::new(f)
}

pub fn create_future_expensive(id: u32) -> Box<dyn Future<Item=(), Error=()>> {
    let f = future::ok(())
        .and_then(|_| {
            println!("Started expensive Future");
            future::ok(())
        }).and_then(move |_| {
            ExpensiveFuture::new(3, format!("expensive future {}", &id))
        }).and_then(|_| {
            println!("Finished expensive Future");
            future::ok(())
        });
    Box::new(f)
}


pub fn create_futures_expensive(n: u32) -> Vec<Box<dyn Future<Item=(), Error=()>>>
{
    let mut futures: Vec<Box<dyn Future<Item=(), Error=()>>> = vec![];
    for i in 0..n {
        let f = create_future_expensive(n);
        futures.push(f);
    };
    futures
//    vec![create_future_expensive(0), create_future_expensive(1), create_future_expensive(2), create_future_expensive(3), create_future_expensive(4)]
}


//pub fn create_futures<T>(n: u32, create_future: Box<dyn Fn(u32) -> T>) -> Vec<T> // this also works if you box the clojure
// The advantage of above is we could pass clojure, whereas with fn only functions
pub fn create_futures<T>(n: u32, create_future: fn(u32) -> T) -> Vec<T>
{
    let mut futures: Vec<T> = vec![];
    for i in 0..n {
        let f = create_future(n as u32);
        futures.push(f);
    };
    futures
}

pub fn simple() {
    let range = 1..100;
    let stream = stream::iter_ok::<_, ()>(range);
    let multiplier = stream
        .map(|val| {
            println!("Stage -1- reached by: {}", val);
            val
        })
        .filter(|val| *val % 2 == 0)
        .map(|val| val + 10)
        .for_each(|val| {
            println!("Resolved stream value {}", val);
            future::ok(())
        });
    tokio::run(multiplier);
}

pub fn v1() {
    let range = 1..100;
    let stream = stream::iter_ok::<_, ()>(range);
    let multiplier = stream
        .map(|val| {
            println!("Stage -1- reached by: {}", val);
            future::ok(val)
        })
        .buffer_unordered(5)
        .and_then(|val| {
            println!("Stage -2- starting: {}", val);
            tokio_timer::sleep(Duration::new(1, 0)).map_err(|e| ()).map(move |_| val)
        })
        .for_each(|val| {
            println!("Resolved stream value {}", val);
            tokio_timer::sleep(Duration::new(1, 0)).map_err(|e| ())
        });
    tokio::run(multiplier);
}


pub fn v2() {
    let PAR = 20;
    let range = 1..100;
    let stream = stream::iter_ok::<_, ()>(range);
    // todo: The differenec from my real case is that here we built stream on top of iterator if i32s. In real case we have iterator of futures
    let multiplier = stream
        .map(|val| future::ok(val))
        .buffer_unordered(PAR)
        .map(|val| val)  // this is alright, just value mapping
        .map(|val| val) // this is alright, just value mapping
        .map(|val| val) // this is alright, just value mapping
        .map(|val| {
            println!("1-PRE {}", &val);
            tokio_timer::sleep(Duration::new(1, 0))
                .map_err(|e| ())
                .map(move |_| {
                    println!("1-POST{}", &val);
                    val
                }) // but here we are returning futures - we have to setup buffer_unordered to specify how many of these can run in parallel!
        })
        .buffer_unordered(2)   // if we comment this out
        //.and_then(|val| val) // we need to enable this combinator - previous map is returning Future, so next map() whould receive this Future as paramater an try to map it
        .map(|val| {
            println!("2-PRE {}", &val);
            tokio_timer::sleep(Duration::new(1, 0))
                .map_err(|e| ())
                .map(move |_| {
                    println!("2-POST {}", &val);
                    val
                })
        })
        .buffer_unordered(PAR)
//        .and_then(|val| val)
        .map(|val| {
            println!("3-PRE {}", &val);
            tokio_timer::sleep(Duration::new(1, 0))
                .map_err(|e| ())
                .map(move |_| {
                    println!("3-POST {}", &val);
                    val
                })
        })
        .buffer_unordered(PAR)
//        .and_then(|val| val)
        .for_each(|val| {
            println!("Resolved stream value {}", val);
            future::ok(())
        });
    tokio::run(multiplier);
}


pub fn foreach_stream_no_buffer() {
    let PAR = 20;
    let range = 1..100;
    let stream = stream::iter_ok::<_, ()>(range);
    let multiplier = stream
        .for_each(|val| {
            println!("Resolved stream value {}", val);
            future::ok(())
        });
    tokio::run(multiplier);
}

pub fn stream_expensive_futures_with_send_on_tokio() {
    let PAR = 20;
//    let futures = create_5_futures_expensive_send();
    let futures = create_futures(6, create_future_expensive_send);

    let stream = stream::iter_ok::<_, ()>(futures);
//    let stream = stream::futures_unordered(futures);
    let buffered_stream = stream
        .buffer_unordered(5)
        .for_each(|_| {
            println!("Resolved stream value",);
            future::ok(())
        });
    tokio::run(buffered_stream);
}

pub fn stream_expensive_futures_blocking() {
    let PAR = 20;
    let futures = create_futures(6, create_future_expensive);

    let stream = stream::iter_ok::<_, ()>(futures);
    let buffered_stream = stream
        .buffer_unordered(2)
        .for_each(|_| {
            println!("Resolved stream value",);
            future::ok(())
        });
    buffered_stream.wait(); // if future does not implement Send trait, it can't be executed on tokio
}

pub fn run_expensive_future()
{
    let f = create_future_expensive_send(0);
    tokio::run(f);
    println!("Finished tokio interval example");
}

pub fn run_tokio_sleep_with_wait()
{
    println!("Starting");
    tokio_timer::sleep(Duration::new(2, 0)).map_err(|e| ()).wait();
    println!("Finished");
}

pub fn run_tokio_sleep_with_tokio()
{
    println!("Starting");
    let f = tokio::timer::Delay::new(Instant::now() + Duration::from_millis(1000))
        .map(|_| ())
        .map_err(|err|panic!("Problem with delay! {:?}", err));
    tokio::run(f);
//    f.wait();  // This doesnt work because: "The instance must be used on a task that is spawned onto the Tokio runtime." << https://tokio.rs/docs/going-deeper/timers/
    println!("Finished");
}

pub fn run_tokio_interval()
{
    println!("Started tokio interval example");
    let s = tokio::timer::Interval::new(Instant::now(), Duration::from_millis(100))
        .take(10)
        .for_each(|instant| {
            println!("Now is: {:?}", instant);
            future::ok(())
        })
        .map_err(|err|panic!("Problem with interval! {:?}", err));
    tokio::run(s);
    println!("Finished tokio interval example");
}

pub fn run() {
//    simple();
//    v1();
//    v2();
//    foreach_stream_no_buffer();

//    run_expensive_future();
//    run_tokio_sleep_with_wait();
//    run_tokio_sleep_with_tokio();
//    run_tokio_interval();
    stream_expensive_futures_with_send_on_tokio();
    stream_expensive_futures_blocking();
}
