//https://dev.to/mindflavor/rust-futures-an-uneducated-short-and-hopefully-not-boring-tutorial---part-3---the-reactor-433

use std::collections::HashMap;
use tokio::prelude::future::{Either, err};
use futures::future::{FutureResult, ok};
use futures::future;
use std::net::TcpStream;
use std::{io, thread};
use std::io::prelude::*;
use futures::Future;
use futures::*;
use std::time::Duration;
use std::rc::Rc;
use chrono::prelude::*;
use std::error::Error;
use tokio_core::reactor::Core;
use crate::devto_future_custom_implementation2::WaitInAnotherThread;


struct MyStream {
    current: u32,
    max: u32,
    core_handle: tokio_core::reactor::Handle
}

impl MyStream {
    pub fn new(max: u32, core_handle: tokio_core::reactor::Handle) -> MyStream {
        MyStream {
            current: 0,
            max,
            core_handle
        }
    }
}

impl Stream for MyStream {
    type Item = u32;
    type Error = Box<Error>;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use futures::future::Executor;

        match self.current {
            ref mut x if *x < self.max => {
                *x = *x + 1;
                let f = WaitInAnotherThread::new(2,  format!("WAIT {:?}", x));
                self.core_handle.execute(f);
                Ok(Async::Ready(Some(*x)))
            }
            _ => Ok(Async::Ready(None)),
        }
    }
}

//pub fn run()
//{
//    let mut reactor = Core::new().unwrap();
//    let my_stream = MyStream::new(5);
//
//    let fut = my_stream.for_each(|num| {
//        println!("num === {}", num);
//        ok(())
//    });
//
//    println!("wait future started");
//    let ret = reactor.run(fut).unwrap();
//    println!("wait future completed. ret == {:?}", ret);
//}
pub fn run() {
    let mut reactor = Core::new().unwrap();

    // create a Stream returning 5 items
    // Each item will spawn an "inner" future
    // into the same reactor loop
    let my_stream = MyStream::new(5, reactor.handle());

    // we use for_each to consume
    // the stream
    let fut = my_stream.for_each(|num| {
        println!("num === {:?}", num);
        ok(())
    });

    // this is a manual future. it's the same as the
    // future spawned into our stream
    let wait = WaitInAnotherThread::new(3, "Manual3".to_owned());

    // we join the futures to let them run concurrently
    let future_joined = fut.map_err(|err| {}).join(wait);

    // let's run the future
    let ret = reactor.run(future_joined).unwrap();
    println!("ret == {:?}", ret);
}