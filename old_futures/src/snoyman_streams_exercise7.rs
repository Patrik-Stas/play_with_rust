extern crate tokio;
extern crate tokio_timer;

use std::time::{Duration};
use tokio::prelude::*;

pub fn simple() {
    let range = 1..100;
    let stream = stream::iter_ok::<_, ()>(range);
    let multiplier = stream
        .map(|val| {
            println!("Stage -1- reached by: {}", val);
            val
        })
        .filter(|val| *val % 2 == 0 )
        .map(|val| val +  10)
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
    let multiplier = stream
        .map(|val| future::ok(val))
        .buffer_unordered(PAR)
        .map(|val| val)  // this is alright, just value mapping
        .map(|val| val) // this is alright, just value mapping
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


pub fn run() {
//    simple();
//    v1();
    v2();
}
