#![allow(dead_code)]
#![allow(unused_variable)]
extern crate chrono;

#[macro_use]
extern crate futures;

mod futures_async;
use std::mem;

fn main() {
    futures_async::run();
}
