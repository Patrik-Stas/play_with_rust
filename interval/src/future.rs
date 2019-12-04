
extern crate futures;

use super::interval::Interval;
use futures::prelude::*;

pub struct IntervalFuture {
    interval: Interval,
    last: usize,
}

impl IntervalFuture {
    pub fn new(interval: Interval) -> IntervalFuture {
        let last = interval.get_counter();
        IntervalFuture { interval, last }
    }
}

impl Future for IntervalFuture {
    type Item = usize;
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        println!("IntervalFuture::poll");
        let curr = self.interval.get_counter();
        if curr == self.last {
            println!("IntervalFuture::poll --> Not ready");
            let task = futures::task::current();
            self.interval.set_task(task);
            Ok(Async::NotReady)
        } else {
            println!("IntervalFuture::poll --> Ready!");
            self.last = curr;
            Ok(Async::Ready(curr))
        }
    }
}