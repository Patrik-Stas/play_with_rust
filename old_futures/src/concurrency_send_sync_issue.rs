use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::rc::Rc;
use std::fmt::Debug;

#[derive(Debug)]
struct Foo {
    fooval : Rc<i32>
}

//unsafe impl Send for Foo {}
//unsafe impl Sync for Foo {}

#[derive(Debug)]
struct Bar {
    barval : Arc<i32>
}

fn accept_send_sync<T: Send+Sync+Debug>(t: T) {
    println!("accept_send_sync: {:?}", t);
}

pub fn run() {
    let foo = Foo{ fooval: Rc::new(123) };
    let bar = Bar{ barval: Arc::new(123) };
    accept_send_sync(bar);
    // works for Bar, because Send and Sync is automatically derived by compiler wherever possible
    // - for example, if all subtype implements Send+Sync, which is case for Arc<i32> used by Bar struct
//    accept_send_sync(foo);
    // ^^^^^^^^^^^^^^^^ `std::rc::Rc<i32>` cannot be shared between threads safely
    // because Send+Sync is not derived by compiler for Foo - it cannot be, because it contains
    // non Send, non Sync type Rc<T>!
    // We could make this compile by explicitly unsafely implementing Send and Sync for Foo
    // but that would probably lead to runtime errors!
}