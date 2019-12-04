
extern crate futures;

use super::interval::Interval;
use futures::prelude::*;

pub struct IntervalStream {
    interval: Interval,
    last: usize,
}

impl IntervalStream {
    pub fn new(interval: Interval) -> IntervalStream {
        let last = interval.get_counter();
        IntervalStream { interval, last }
    }
}

impl Stream for IntervalStream {
    type Item = usize;
    type Error = ();

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        println!("IntervalStream::poll");
        let curr = self.interval.get_counter();
        if (curr > 10) {
            Ok(Async::Ready(None))
        } else if curr == self.last {
            println!("IntervalStream::poll --> Not ready");
            let task = futures::task::current();
            self.interval.set_task(task);
            Ok(Async::NotReady)
        } else {
            println!("IntervalStream::poll --> Ready!");
            self.last = curr;
            Ok(Async::Ready(Some(curr)))
        }
    }
}