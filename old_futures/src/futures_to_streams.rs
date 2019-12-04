extern crate futures;
extern crate tokio_core;
extern crate reqwest;

use futures::*;
use tokio_core::reactor::Core;
use self::reqwest::r#async::{Client, Response, Decoder};
use self::reqwest::Error;

//pub fn get_futures() -> Vec<Box<dyn Future<Item=Box<dyn Future<Item=Response, Error=reqwest::Error>>, Error=()>>>{
//    let client = Client::new();
//    let hyper1 = Box::new(future::ok(Box::new(client.get("https://hyper.rs").send())));
//    let hyber2 = Box::new(future::ok(Box::new(client.get("https://google.com").send())));
//    let hyper3 = Box::new(future::ok(Box::new(client.get("https://cnn.com").send())));
//    let hyper4 = Box::new(future::ok(Box::new(client.get("https://amazon.com").send())));
//    let hyper5 = Box::new(future::ok(Box::new(client.get("https://reddit.com").send())));
//    let hyper6 = Box::new(future::ok(Box::new(client.get("https://hojko.com").send())));
//    let hyper7 = Box::new(future::ok(Box::new(client.get("https://facebook.com").send())));
//    let hyper8 = Box::new(future::ok(Box::new(client.get("https://twitter.com").send())));
//    let hyper9 = Box::new(future::ok(Box::new(client.get("https://9gag.com").send())));
//    let witherr = Box::new(future::ok(Box::new(client.get("http://fofobar123.par/foo").send())));
//    vec![hyper1, hyber2, hyper3, hyper4, hyper5, hyper6, hyper7, hyper8, hyper9, witherr]
//}


pub fn get_futures() -> Vec<Box<dyn Future<Item=Response, Error=reqwest::Error>>>{
    let client = Client::new();
    let hyper1 = client.get("https://hyper.rs").send().and_then(|res| {println!("Res={:?}", res); future::ok(res)});
    let hyber2 = client.get("https://google.com").send().and_then(|res| {println!("Res={:?}", res); future::ok(res)});
    let hyper3 = client.get("https://cnn.com").send().and_then(|res| {println!("Res={:?}", res); future::ok(res)});
    let hyper4 = client.get("https://amazon.com").send().and_then(|res| {println!("Res={:?}", res); future::ok(res)});
    let hyper5 = client.get("https://reddit.com").send().and_then(|res| {println!("Res={:?}", res); future::ok(res)});
    let hyper6 = client.get("https://hojko.com").send().and_then(|res| {println!("Res={:?}", res); future::ok(res)});
    let hyper7 = client.get("https://facebook.com").send().and_then(|res| {println!("Res={:?}", res); future::ok(res)});
    let hyper8 = client.get("https://twitter.com").send().and_then(|res| {println!("Res={:?}", res); future::ok(res)});
    let hyper9 = client.get("https://9gag.com").send().and_then(|res| {println!("Res={:?}", res); future::ok(res)});
    let witherr = client.get("http://fofobar123.par/foo").send().and_then(|res| {println!("Res={:?}", res); future::ok(res)});
    vec![Box::new(hyper1), Box::new(hyber2), Box::new(hyper3), Box::new(hyper4),
         Box::new(hyper5), Box::new(hyper6), Box::new(hyper7), Box::new(hyper8),
         Box::new(hyper9), Box::new(witherr)
    ]
}
//
//pub fn loop_stream(stream: Box<dyn stream::StreamFuture<Item=i32, Error=i32>>) {
//    let mut core = Core::new().unwrap();
//    loop {
//        match core.run(stream) {
//            Ok((None, _something)) => {
//                println!("finished");
//                break;
//            }
//            Ok((Some(response), mut next_requests)) => {
//                {
//                    let inner = next_requests.get_mut();
//                    println!("{}", inner.is_empty());
//                    println!("{}", response.status());
//                    println!("{:?}", response);
//                }
//                next = next_requests.into_future();
//            }
//            Err((error, next_requests)) => {
//                println!("error: {:?}", error);
//                next = next_requests.into_future();
//            }
//        }
//    }
//}

pub fn website_futures3() {
    let client = Client::new();
    /// TODO: create simple sample future which would have some outputs and sleep. It's hard to work with these hyber futures,
    /// because if I try to add .map() add some printlns, each of these hyber futures will esentially contain closure
    /// and then it's trouble to use them as group - can't put themm into vector cause each closure is unique type.
    /// So it end up asking you for boxing and things get messy
    ///
    /// TODO: Is it possible to create vector which would be made of things like: future::ok(42).map(|no| 42 + whatever)
    /// I guess it shoul be possible if I can somehow specify the return type of each of these closures?
    let hyper1 = client.get("https://hyper.rs").send();
    let hyber2 = client.get("https://hyper.rs").send();
    let hyber3 = client.get("https://google.com").send();
    let hyber4 = client.get("https://cnn.com").send();
    let hyber5 = client.get("https://amazon.com").send();
    let hyber6 = client.get("https://reddit.com").send();
    let hyber7 = client.get("https://hojko.com").send();
    let hyber8 = client.get("https://facebook.com").send();
    let witherr = client.get("http://fofobar123.par/foo").send();
    let stream = stream::futures_unordered(vec![
        future::ok(hyper1),
        future::ok(hyber2),
        future::ok(hyber3),
        future::ok(hyber4),
        future::ok(hyber5),
        future::ok(hyber6),
        future::ok(hyber7),
        future::ok(hyber8),
        future::ok(witherr),
    ]);
    let mut next = stream.buffer_unordered(1);
//    let results = next.for_each(move |res| {
//        println!("processing");
//        future::ok(()) // TODO: Why do I have to return futures in for_each? ???
//    }); // TODO: this does not execute. How to consume stream? ??????
    let mut core = Core::new().unwrap();
    println!("Start consuming streams");
//    core.run();
    next.into_future().wait();
    println!("Finished consuming streams");

}

pub fn website_futures2() {
    let client = Client::new();
    let hyper1 = client.get("https://hyper.rs").send();
    let hyber2 = client.get("https://google.com").send();
    let witherr = client.get("http://fofobar123.par/foo").send();
    let stream = stream::futures_unordered(vec![
        future::ok(hyper1),
        future::ok(hyber2),
        future::ok(witherr),
    ]);
    let mut next = stream.buffer_unordered(10).into_future(); // (1)

    let mut core = Core::new().unwrap();
    loop {
        match core.run(next) {
            Ok((None, _something)) => {
                println!("All futures completed.");
                break;
            }
            Ok((Some(response), mut next_requests)) => {
                {
                    let inner = next_requests.get_mut();
                    println!("Response: {:?}", response);
                }
                next = next_requests.into_future();
            }
            Err((error, next_requests)) => {
                println!("Error: {:?}", error);
                next = next_requests.into_future();
            }
        }
    }
}

pub fn website_futures() {
    let mut core = Core::new().unwrap();
    let myfutures = get_futures();
    let stream = stream::futures_unordered(myfutures);

//    let stream = stream::futures_unordered(vec![
//        future::ok(hyper1),
//        future::ok(hyber2),
//        future::ok(hyper3),
//    ]);

//    let mut next = stream.buffer_unordered(10); // (1)
//    let mut next = stream.buffer_unordered(10).into_future(); // (1)
//    loop {
//        match core.run(next) {
//            Ok((None, _something)) => {
//                println!("finished");
//                break;
//            }
//            Ok((Some(response), mut next_requests)) => {
//                {
//                    let inner = next_requests.get_mut();
//                    println!("{}", inner.is_empty());
//                    println!("{}", response.status());
//                    println!("{:?}", response);
//                }
//                next = next_requests.into_future();
//            }
//            Err((error, next_requests)) => {
//                println!("error: {:?}", error);
//                next = next_requests.into_future();
//            }
//        }
//    }
//    let mut next = stream.buffer_unordered(5).into_future(); // (1)
//    loop {
//        match core.run(next) {
//
//            Ok((None, _something)) => {
//                println!("finished");
//                break;
//            },
//            Ok((Some(response), mut next_requests)) => {
//                {
//                    let inner = next_requests.get_mut();
//                    println!("{}", inner.is_empty());
//                    println!("{}", response.status());
//                }
//                next = next_requests.into_future();
//            }
//            Err((error, next_requests)) => {
//                next = next_requests.into_future();
//            }
//        }
//    }
}

pub fn run() {
//    website_futures()
//    website_futures2();
    website_futures3();
}