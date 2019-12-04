// v1 - polling / 'executing" future on our own

// extern crate futures;
//
// mod future;
// mod interval;
//
// use self::interval::Interval;
// use self::future::IntervalFuture;
// use futures::prelude::*;
//
// fn main() {
//     let interval = Interval::from_millis(500); // half a second
//     let mut interval_future = IntervalFuture::new(interval);
//     let duration = std::time::Duration::from_millis(100); // 0.1 seconds
//
//     for i in 1..51 {
//         match interval_future.poll() {
//             Ok(Async::Ready(curr)) => {
//                 println!("Iteration number {}, counter is {}", i, curr);
//             }
//             Ok(Async::NotReady) => (),
//             Err(()) => unreachable!(),
//         }
//
//         std::thread::sleep(duration);
//     }
// }


// v2 - attemp to utilize tokio insttead of writing our own "executor" like above

//extern crate futures;
//extern crate tokio;
//
//mod future;
//mod interval;
//
//use self::interval::Interval;
//use self::future::IntervalFuture;
//use tokio::prelude::*;
//
//fn main() {
//    let interval = Interval::from_millis(500); // half a second
//    let mut interval_future = IntervalFuture::new(interval);
//
//    tokio::run(interval_future); // doesn't work because we need to return () from future we pass to tokio
//}


// initial implementation to fulfill the tokio requirements
//extern crate futures;
//extern crate tokio;
//
//mod future;
//mod interval;
//
//use self::interval::Interval;
//use self::future::IntervalFuture;
//use tokio::prelude::*;
//
//struct IntervalPrinter(IntervalFuture);
//
//impl Future for IntervalPrinter {
//    type Item = ();
//    type Error = ();
//    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
//        match self.0.poll() {
//            Ok(Async::Ready(curr)) => {
//                println!("Counter is: {}", curr);
//                Ok(Async::Ready(()))
//            }
//            Ok(Async::NotReady) => Ok(Async::NotReady),
//            Err(e) => Err(e),
//        }
//    }
//}
//
//fn main() {
//    let interval = Interval::from_millis(100); // half a second
//    let interval_future = IntervalFuture::new(interval);
//    let interval_printer = IntervalPrinter(interval_future);
//
//    tokio::run(interval_printer)
//}



// simplified with macro
//#[macro_use]
//extern crate futures;
//extern crate tokio;
//
//mod future;
//mod interval;
//
//use self::interval::Interval;
//use self::future::IntervalFuture;
//use tokio::prelude::*;
//
//struct IntervalPrinter(IntervalFuture);
//
//impl Future for IntervalPrinter {
//    type Item = ();
//    type Error = ();
//    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
//        let curr = try_ready!(self.0.poll());
//        println!("Counter is: {}", curr);
//        Ok(Async::Ready(()))
//    }
//}
//
//fn main() {
//    let interval = Interval::from_millis(10); // half a second
//    let interval_future = IntervalFuture::new(interval);
//    let interval_printer = IntervalPrinter(interval_future);
//
//    tokio::run(interval_printer)
//}

//
//extern crate futures;
//extern crate tokio;
//
//mod future;
//mod interval;
//
//use self::interval::Interval;
//use self::future::IntervalFuture;
//use tokio::prelude::*;
//
//fn main() {
//    let interval = Interval::from_millis(500); // half a second
//    let interval_future = IntervalFuture::new(interval);
//    let interval_printer = interval_future.map(|curr| {
//        println!("Counter is: {}", curr);
//    });
//
//    // but this thing only prints one number, unlike previous!
//    tokio::run(interval_printer)
//}




// "Wonky" attempt to repeatedly call the future, doesnt compile even
//extern crate futures;
//extern crate tokio;
//
//mod future;
//mod interval;
//
//use self::interval::Interval;
//use self::future::IntervalFuture;
//use tokio::prelude::*;
//use std::sync::{Arc, Mutex};
//use futures::future::loop_fn;
//
//fn main() {
//    let interval = Interval::from_millis(500); // half a second
//    let interval_future = Arc::new(Mutex::new(IntervalFuture::new(interval)));
//    let interval_printer = loop_fn(interval_future, |interval_future| {
//        let interval_future_clone = interval_future.clone();
//        interval_future.lock().unwrap().and_then(|curr| {
//            println!("Counter: {}", curr);
//            futures::future::ok(futures::future::Loop::Continue(interval_future_clone))
//        })
//    });
//
//    tokio::run(interval_printer)
//}


// working attempt to keep polling our IntervalFuture
//#[macro_use]
//extern crate futures;
//extern crate tokio;
//
//mod future;
//mod interval;
//
//use self::future::IntervalFuture;
//use self::interval::Interval;
//use tokio::prelude::*;
//
//struct KeepPrinting(IntervalFuture);
//
//impl Future for KeepPrinting {
//    type Item = ();
//    type Error = ();
//    fn poll(&mut self) -> Poll<(), ()> {
//        loop {
//            println!("Keep printing: loop starts");
//            let curr = try_ready!(self.0.poll());
//            println!("Keep printing polled with ready! So counter is: {}", curr);
//        }
//    }
//}
//
//fn main() {
//    let interval = Interval::from_millis(500); // half a second
//    let interval_future = IntervalFuture::new(interval);
//    let keep_printing = KeepPrinting(interval_future);
//
//    tokio::run(keep_printing)
//}
//
//



// working attempt to keep polling our IntervalFuture
#[macro_use]
extern crate futures;
extern crate tokio;

mod future;
mod interval;
mod stream;

use self::future::IntervalFuture;
use self::interval::Interval;
use tokio::prelude::*;
use crate::stream::IntervalStream;

struct KeepPrinting(IntervalFuture);

impl Future for KeepPrinting {
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Poll<(), ()> {
        loop {
            println!("Keep printing: loop starts");
            let curr = try_ready!(self.0.poll());
            println!("Keep printing polled with ready! So counter is: {}", curr);
        }
    }
}

fn main() {
    let interval = Interval::from_millis(500); // half a second
    let interval_stream = IntervalStream::new(interval);
    let keep_printing = interval_stream.for_each(|val| {
        println!("Resolved stream value {}", val);
        futures::future::ok(())
    });
    tokio::run(keep_printing)
}

