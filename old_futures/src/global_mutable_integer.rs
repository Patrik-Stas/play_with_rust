use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::thread::JoinHandle;

static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

fn do_a_call() {
    CALL_COUNT.fetch_add(1, Ordering::SeqCst);
}

pub fn run() {
    do_a_call();
    do_a_call();

    println!("global atomic integer value: {}", CALL_COUNT.load(Ordering::SeqCst));

    let mut children: Vec<JoinHandle<()>> = vec![];
    for i in 0..3 {
        let child: JoinHandle<()> = thread::spawn( move || {
            for j in 0..10 {
                println!("Thread {} about to increment value, now = {}", i, CALL_COUNT.load(Ordering::SeqCst));
                do_a_call();
                println!("Thread {} incremented value, now = {}", i, CALL_COUNT.load(Ordering::SeqCst));
            }
        });
        children.push(child);
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
    println!("global atomic integer value after all threads ran {}", CALL_COUNT.load(Ordering::SeqCst));

}