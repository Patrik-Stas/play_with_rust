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

pub struct ExpensiveFuture {
    end_time: DateTime<Utc>,
    running: bool,
    thread_name: String
}

impl ExpensiveFuture {
    pub fn new(delay_seconds: i64, thread_name: String) -> ExpensiveFuture {
        ExpensiveFuture {
            end_time:  Utc::now() +  chrono::Duration::seconds(delay_seconds),
            running: false,
            thread_name: thread_name
        }
    }

    pub fn wait_blocking(&self) {
        while Utc::now() < self.end_time {
            let delta_sec = self.end_time.timestamp() - Utc::now().timestamp();
            if delta_sec > 0 {
                thread::sleep(::std::time::Duration::from_secs(delta_sec as u64));
            }
        }
        println!("the time has come == {:?}!", self.end_time);
    }

    fn run(&mut self, task: task::Task) {
        let lend = self.end_time;

        thread::spawn(move || {
            while Utc::now() < lend {
                let delta_sec = lend.timestamp() - Utc::now().timestamp();
                if delta_sec > 0 {
                    thread::sleep(::std::time::Duration::from_secs(delta_sec as u64));
                }
                task.notify();
            }
            println!("the time has come == {:?}!", lend);
        });
    }
}

impl Future for ExpensiveFuture {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if Utc::now() < self.end_time {
            println!("{} :: not ready yet! parking the task.", self.thread_name);

            if !self.running {
                println!("{} :: side thread not running! starting now!", self.thread_name);
                self.run(task::current());
                self.running = true;
            }

            Ok(Async::NotReady)
        } else {
            println!("{} :: ready! the task will complete.", self.thread_name);
            Ok(Async::Ready(()))
        }
    }
}
