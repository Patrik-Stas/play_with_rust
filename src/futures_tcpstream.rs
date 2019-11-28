use std::collections::HashMap;
use tokio::prelude::future::{Either, err};
use futures::future::FutureResult;
use futures::future;
use std::net::TcpStream;
use std::io;
use std::io::prelude::*;
use futures::Future;

// from https://tokio.rs/docs/futures/combinators/
// but doesnt seem to be working well, got some errors
// todo: fix and pull request ot tokio.rs

fn tokio_sample_combinators() {
//    let addr = "127.0.0.1:1234".parse().unwrap();
//
//    let stream = TcpStream::connect(&addr);
//    let f = stream.unwrap()
//        .and_then(|socket| {
//            socket.write_all(socket, b"hello world")
//        })
//        .and_then(|(socket, _)| {
//            // read exactly 11 bytes
//            socket.read_exact(socket, vec![0; 11])
//        })
//        .and_then(|(socket, buf)| {
//            println!("got {:?}", buf);
//            Ok(())
//        })
//        .map_err(|_| println!("failed"));
//
//    tokio::run(future);
}

pub fn run()
{
    tokio_sample_combinators()
}
