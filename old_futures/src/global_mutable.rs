use std::sync::Mutex;

lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(Vec::new());
}

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

fn reset() {
    let mut vec = ARRAY.lock().unwrap();
    *vec = vec![]
}

pub fn run() {
    do_a_call();
    do_a_call();

//    let mut m: Vec<i32> = Vec::new();
//    m.push(123);

    do_a_call();

    println!("called {}", ARRAY.lock().unwrap().len());
    reset();
    println!("after reset {}", ARRAY.lock().unwrap().len());
    do_a_call();
    println!("called after reset {}", ARRAY.lock().unwrap().len());
}