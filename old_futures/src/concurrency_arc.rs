use std::thread;
use std::sync::Arc;
use std::time::Duration;

pub fn run() {
    let mut v: Arc<Vec<u8>> = Arc::from(Vec::new());
//    v.push(1);
//    v.push(2);
    println!("{:#?}", v);
}