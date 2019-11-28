//use std::collections::HashMap;
//use tokio::prelude::Future;
//use tokio::prelude::future::{ok, Either, err};
//use futures::future::FutureResult;
//
//#[derive(Debug)]
//struct SimpleRequest {
//    age: u8,
//}
//
//pub trait MyService {
//    /// Requests handled by the service.
//    type Request;
//    /// Responses given by the service.
//    type Response;
//    /// The future response value.
//    type Future: Future<Item = Self::Response, Error = ()>;
//    fn call(&mut self, req: Self::Request) -> Self::Future;
//}
//
//
//pub struct SayHiMiddleware<S> {
//    service: S,
//}
//
//impl<S, B> MyService for SayHiMiddleware<S>
//    where
//        S: MyService<Request=ServiceRequest, Response=ServiceResponse<B>, Error=()>,
//        S::Future: 'static,
//        B: 'static,
//{
//    type Request = SimpleRequest;
//    type Response = ServiceResponse<B>;
//    type Future = Box<dyn Future<Item=Self::Response, Error=()>>;
//
//    fn call(&mut self, req: ServiceRequest) -> Self::Future {
//
//    }
//}
//
//pub fn run()
//{
////    let myfuture = ok(1)
////        .and_then(|_| {
////            println!("Hello world!")
////        });
//    let myfuture =
//        ok(())
//            .and_then(|_| ok(123))
//            .and_then(|res| {
//                println!("{:?}", res);
//                ok(())
//            })
//            .map_err(|err| println!("It went bad!! {:?}", err))
//            .and_then(|res| {
//                println!("Result of my async code {:?}", res);
//                ok(())
//            });
//    tokio::run(myfuture);
//}