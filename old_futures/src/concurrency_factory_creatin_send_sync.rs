use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::rc::Rc;
use std::fmt::Debug;

#[derive(Debug)]
struct Foo {
    fooval: Rc<i32>
}

#[derive(Debug)]
struct Bar {
    barval: Arc<i32>
}

trait MyTrait {}

impl MyTrait for Foo {}

impl MyTrait for Bar {}


fn create_send_sync_data_non_sync_non_send() -> Box<dyn MyTrait> {
// fn create_send_sync_data_non_sync_non_send() -> Box<dyn MyTrait + Send + Sync> { //you might attempt this, but won't work because Foo is by default not Send+Sync, unless you unsafely make it so!
   Box::new(Foo { fooval: Rc::new(123) })
}

fn create_send_sync_data_sync_send() -> Box<dyn MyTrait + Send + Sync> {
    Box::new(Bar { barval: Arc::new(123) })
}

fn receive_mytrait(myData: Box<dyn MyTrait + Send + Sync>) {}

pub fn run() {
//    let m = create_send_sync_data_non_sync_non_send(); // receive_mytrait(m) will not accept this result, because it's not producing Send+Sync results!
    let m = create_send_sync_data_sync_send();
    receive_mytrait(m);
}