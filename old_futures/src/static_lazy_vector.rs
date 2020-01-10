//// TODO: --------------- VERSION 0 ------------------- naive attempt to mutate static
//lazy_static! {
//    static ref mut VEC: Vec<i32> = {
//    //             ^^^ no rules expected this token in macro call
//        let mut m = Vec::with_capacity(1000);
//        m
//    };
//}
//
//pub fn run() {
//    VEC.push(3);
// //   ^^^ cannot borrow as mutable
//}

// this just doesn't make sense, that's not how the macro is defined.
// https://docs.rs/lazy_static/1.0.0/lazy_static/
// Aditionally, this :
// https://stackoverflow.com/questions/48114390/why-does-a-lazy-static-value-claim-to-not-implement-a-trait-that-it-clearly-impl/48115258#48115258
// provides some nice insights into how lazy_static works
// TODO: CONCULSION: lazy_static generates a new type which wraps your types. In addition to that,
// TODO: we are only working with these through reference, so that this static data wouldn't
// TODO: be moved out of static. Upon usage, the static is (I guess usually) automatically
// TODO: de-referenced and we get to the data we declared (wrapped by this special lazy_static wrapper type)


// the push method has signature: pub fn push(&mut self, value: T)
// See https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push
// but VEC was declared immutable. So we cant call this method!

//// TODO: --------------- VERSION 1 ------------------- can't we really modify lazy_static data without mut?
//lazy_static! {
//    static ref VEC: Vec<i32> = {
//        let mut m = Vec::with_capacity(1000);
//        m
//    };
//}
//
//pub fn run() {
//    VEC.push(3);
// //   ^^^ cannot borrow as mutable
//}

// TODO: CONCLUSION : the push method has signature: pub fn push(&mut self, value: T)
// TODO: CONCLUSION : See https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push
// TODO: CONCLUSION : but VEC was declared immutable. So we cant call this method!


//// TODO: --------------- VERSION 2 ------------------- Wrap data with mutex
//use std::sync::Mutex;
//lazy_static! {
//    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
//}
//
//fn do_a_call() {
//    ARRAY.lock().unwrap().push(1);
//}
//// in this version we lazily initialize immutable Mutex. We are only using lock() method on
//// mutex which has signature: pub fn lock(&self) -> LockResult<MutexGuard<T>>
//// https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.lock
//// So seems like Mutex as well can provide a form of interior mutability? Let's make sure in VERSION 3 - experiment
//
//pub fn run() {
//    do_a_call();
//    do_a_call();
//    do_a_call();
//
//    println!("called {}", ARRAY.lock().unwrap().len());
//}


//// TODO: --------------- VERSION 3 - experiment ------------------- which data types can provide interior mutability?
//use std::sync::Mutex;
//pub fn run() {
//    let y = vec![1,2,3];
//    y.push(4);
//    ^ cannot borrow as mutable
//    obviously I can't declare immutable vector and try to modify it.

    // but I can declare a vector inside immutable Mutex, then unwrap the Mutex and modify vector
//    let x : Mutex<Vec<u8>> = Mutex::new(vec![]);
//    {
//        let mut v = x.lock().unwrap();
//        v.push(1);
//        v.push(2);
//    }
//    println!("x = {:?}", x);
//
//    let y = vec![];

    //// TODO: Modifying content of Option VERSION-1
//    let x : Option<Vec<u8>> = Some(vec![]);
//    let mut v = x.unwrap();
//    v.push(1); // x was immutable, but we can mutate the inner content when we move it out into something we declare mutable
//    v.push(2);
    //// TODO CONCLUSION: problem --> we are moving vector out and only then modifying it. We cant reassing it inside the Option, because the option itself is immutable

    //// TODO: Modifying content of Option VERSION-2
//    let x : Option<Vec<u8>> = Some(vec![1,2,3]);
//    let y = x.map(|mut m| {m.push(4); m});
//    println!("y = {:?}", y)
    //// TODO CONCLUSION: problem --> we are creating copy, not modifying content of the Option


    //// TODO:  Modifying content of Option VERSION-3
//    let x : Option<Vec<u8>> = Some(vec![1,2,3]);
//    if let Some(mut v) = x.as_mut() { // this doesnt work
//        v.push(4);
//    }
//    println!("y = {:?}", x)
//}
//// TODO: CONCULUSION for VERSION 3 - experiment --------> You can't achieve interior mutability with Option

// TODO: CONCLUSION: You can get Interior mutabiltiy with Mutex, but definitely not with Option
// TODO CONCULSION: Additional reading https://www.ralfj.de/projects/rust-101/part15.html


//// TODO: --------------- VERSION 4 - experiment ------------------- RefCell interior mutability
//use std::cell::RefCell;
//
//pub fn run() {
//    let ref c = RefCell::new(5);
//    {
//        let m = c.borrow();
//        assert!(c.try_borrow_mut().is_err());
//    }
//
//    assert!(c.try_borrow_mut().is_ok());
//    println!("{:?}", c);
//}
//// TODO: CONCLUSION: yeah, we can easily have interior mutability with RefCell. So can we use it
//// TODO: to create non-mut static-lazy vector which is in fact actually mutable?


//// TODO: --------------- VERSION 5 -------------------
//// TODO: In VERSION 2 we used Mutex to achieve interior mutability on lazy_static,
//// TODO: but if know the code will run on single thread, wby don't we just use RefCell?
//use std::cell::RefCell;
//use std::sync::Arc;
//
//lazy_static! {
//    static ref ARRAY: Arc<RefCell<u8>> = Arc::new(RefCell::new(1));
//}
/////| |_^ `std::cell::RefCell<u8>` cannot be shared between threads safely
/////|
/////= help: the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<u8>`
/////= note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<std::cell::RefCell<u8>>`
/////= note: required by `lazy_static::lazy::Lazy`
/////= note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
//
///// The reason we get is:
///// "Arc<T> will implement Send and Sync as long as the T implements Send and Sync. " https://doc.rust-lang.org/std/sync/struct.Arc.html
///// And lazy_static requires that the types Sync, which is not our case here
//
//pub fn run() {
//    println!("{:?}", ARRAY);
//}
//// TODO: CONCLUSION: Nope, this doesn't work because lazy_static requires thread safe types and
//// TODO: refCell can only be used within single thread.
//// TODO: Although I know the program will be single threaded, the compiler doesn't and the requirements
//// TODO: of lazy_static are strictly enforced!

//// TODO: --------------- VERSION 5 -------------------



//
//lazy_static! {
//    static ref VEC: Vec<i32> = {
//        let mut m = Vec::with_capacity(1000);
//        m
//    };
//}
//
////use std::sync::Mutex;
//
//pub fn run() {
//    unsafe {
//        VEC.push(3);
//    }
//    //   ^^^ cannot borrow as mutable
//}