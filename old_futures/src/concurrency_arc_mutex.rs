use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const N: usize = 10;

pub fn run() {

    // Spawn a few threads to increment a shared variable (non-atomically), and
    // let the main thread know once all increments are done.
    //
    // Here we're using an Arc to share memory among threads, and the data inside
    // the Arc is protected with a mutex.
    let sharedData = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for i in 0..N {
        let sharedData = sharedData.clone();
        let handle = thread::spawn(move || {
            for j in 0..10 {
                // The shared state can only be accessed once the lock is held.
                // Our non-atomic increment is safe because we're the only thread
                // which can access the shared state when the lock is held.
                //
                // We unwrap() the return value to assert that we are not expecting
                // threads to ever fail while holding the lock.
                let mut data = sharedData.lock().unwrap();
                *data += 1;
                println!("Thread {} on iteration {} is updated shared data to {:?}", i, j, data);
            }
            // the lock is unlocked here when `data` goes out of scope.
        });
        handles.push(handle)
    }
    for i in handles {
        i.join();
    }
}