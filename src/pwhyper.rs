//use hyper::Client;
//use hyper::client::connect::HttpConnector;
//use hyper::rt::Future;
//use hyper::{Body, Method, Request, header};
//use futures::*;
//use http::*;

extern crate hyper;

use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};


//pub fn play_with_hyper()
//{
//    println!("sending data to webhook https://webhook.site/a3ff8e75-7af3-401f-8f46-a469fd574b30");
////    let msg = String::new().add("12345-msg-uuid");
//    let mut client = Client::new();
////    let json = r#"{"library":"hyper"}"#;
//    let uri: hyper::Uri = "https://webhook.site/a3ff8e75-7af3-401f-8f46-a469fd574b30".parse().unwrap();
//    let mut req = Request::new(Body::from("12345-msg-uuid".to_string()));
//    *req.method_mut() = Method::POST;
//    *req.uri_mut() = uri.clone();
//    let post = client.request(req)
//        .and_then(|res| {
//            println!("POST: {}", res.status());
//            res.into_body().concat2()
//        })
//        .map_err(|e| {
//            println!("Error! {:?}", e)
//        });
//
//}

//pub fn play_with_hyper()
//{
//    let client = Client::new();
//    let url = "http://httpbin.org/status/201".to_string();
//    let uri = uri::Builder::new()
//        .scheme("https")
//        .authority("hyper.rs")
//        .path_and_query("/")
//        .build()
//        .unwrap();
//    let mut response = match client.get(uri).send() {
//        Ok(response) => response,
//        Err(_) => panic!("Whoops."),
//    };
//    let mut buf = String::new();
//    match response.read_to_string(&mut buf) {
//        Ok(_) => (),
//        Err(_) => panic!("I give up."),
//    };
//    println!("buf: {}", buf);
//}

pub fn play_with_hyper()
{
        // This is main future that the runtime will execute.
        //
        // The `lazy` is because we don't want any of this executing *right now*,
        // but rather once the runtime has started up all its resources.
        //
        // This is where we will setup our HTTP client requests.
//        let client = Client::new();
//
//        let uri = "http://httpbin.org/ip".parse().unwrap();
//
//        client
//            .get(uri)
//            .map(|res| {
//                println!("Response: {}", res.status());
//            })
//            .map_err(|err| {
//                println!("Error: {}", err);
//            })

        // still inside rt::run...

//    rt::run(fetch_url(url));
    rt::run(fetch_url());
}

fn fetch_url() -> impl Future<Item=(), Error=()> {

    let client = Client::new();

    let uri = "http://httpbin.org/ip".parse().unwrap();

    client
        .get(uri)
        .and_then(|res| {
            println!("Response: {}", res.status());
            res
                .into_body()
                // Body is a stream, so as each chunk arrives...
                .for_each(|chunk| {
                    io::stdout()
                        .write_all(&chunk)
                        .map_err(|e| {
                            panic!("example expects stdout is open, error={}", e)
                        })
                })
        })
        .map_err(|err| {
            println!("Error: {}", err);
        })
}