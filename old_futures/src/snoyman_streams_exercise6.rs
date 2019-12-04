// Streams: exercise 6 - Only for strings
//extern crate tokio;
//
//use tokio::prelude::*;
//
//struct MyOk (String);
//
//impl MyOk
//{
//    fn new(name: String) -> MyOk
//    {
//        MyOk(name)
//    }
//}
//
//impl Future for MyOk {
//    type Item = String;
//    type Error = ();
//
//    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
//        Ok(Async::Ready(self.0.clone()))
//    }
//}
//
//pub fn run() {
//    let name = String::from("Alice");
//    let future = MyOk::new(name).and_then(|name| {
//        println!("Name: {}", name);
////        MyOk::new(())
//        futures::future::ok(())
//    });
//
//    tokio::run(future)
//}


// Streams: exercise 6 - Generic but must be cloneable
//extern crate tokio;
//
//use tokio::prelude::*;
//
//struct MyOk<T>(T) where T: Clone;
//
//impl<T> MyOk<T> where T: Clone
//{
//    fn new(okvalue: T) -> MyOk<T>
//    {
//        MyOk(okvalue)
//    }
//}
//
//impl <T>futures::Future for MyOk<T> where T: Clone{
//    type Item = T;
//    type Error = ();
//
//    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
//        Ok(Async::Ready(self.0.clone()))
//    }
//}
//
//pub fn run() {
//    let name = String::from("Alice");
//    let future = MyOk::new(name).and_then(|name| {
//        println!("Name: {}", name);
//        MyOk::new(())
//    });
//
//    tokio::run(future)
//}

// Correct solution
extern crate tokio;

use tokio::prelude::*;

struct MyOk<T>(Option<T>);

impl<T> MyOk<T> {
    fn new(t: T) -> MyOk<T> {
        MyOk(Some(t))
    }
}

impl<T> Future for MyOk<T> {
    type Item = T;
    type Error = ();
    fn poll(&mut self) -> Poll<T, ()> {
        Ok(Async::Ready(self.0.take().unwrap())) // we can take out value out of Option leaving None inside!
    }
}


pub fn run() {
    let name = String::from("Alice");
    let future = MyOk::new(name).and_then(|name| {
        println!("Name: {}", name);
        MyOk::new(())
    });

    tokio::run(future)
}
