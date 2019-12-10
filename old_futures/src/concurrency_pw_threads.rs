//use std::thread;
//
//pub fn run() {
//    let handle = thread::spawn(|| {
//        "Hello from a thread!"
//    });
//
//    println!("{}", handle.join().unwrap());
//}
//
//
//
//use std::thread;
//use std::time::Duration;
//
//pub fn run() {
//    let mut data = vec![1, 2, 3];
//
//    for i in 0..3 {
//        thread::spawn(move || {
//            data[i] += 1;
//        });
//    }
//
//    thread::sleep(Duration::from_millis(50));
//}

//use std::thread;
//use std::sync::Arc;
//use std::time::Duration;
//
//pub fn run() {
//    let mut data = Arc::new(vec![1, 2, 3]);
//
//    for i in 0..3 {
//        let data = data.clone();
//        thread::spawn(move || {
//            data[i] += 1;
//            println!("synced data {:?} in thread {}", data, i);
//        });
//    }
//
//    thread::sleep(Duration::from_millis(50));
//}
