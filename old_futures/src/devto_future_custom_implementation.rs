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


#[derive(Debug)]
struct WaitForIt {
    message: String,
    until: DateTime<Utc>,
    polls: u64,
}

impl WaitForIt {
    pub fn new(message: String, delay_seconds: i64) -> WaitForIt {
        WaitForIt {
            polls: 0,
            message: message,
            until: Utc::now() +  chrono::Duration::seconds(delay_seconds)
        }
    }
}

impl Future for WaitForIt {
    type Item = String;
    type Error = Box<Error>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let now = Utc::now();
        if self.until < now {
            Ok(Async::Ready(
                format!("{} after {} polls!", self.message, self.polls),
            ))
        } else {
            self.polls += 1;
            println!("not ready yet --> {:?}", self);
            futures::task::current().notify();
            Ok(Async::NotReady)
        }
    }
}

pub fn run()
{
    let mut reactor = Core::new().unwrap();

    let wfi_1 = WaitForIt::new("I'm done:".to_owned(), 3);
    println!("wfi_1 == {:?}", wfi_1);
    let wfi_2 = WaitForIt::new("I'm done too:".to_owned(), 3);
    println!("wfi_2 == {:?}", wfi_2);

    let v = vec![wfi_1, wfi_2];

    let sel = futures::future::join_all(v);

    let ret = reactor.run(sel).unwrap();
    println!("ret == {:?}", ret);
}
