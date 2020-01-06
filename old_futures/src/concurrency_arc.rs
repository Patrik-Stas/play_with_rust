use std::thread;
use std::sync::Arc;
use std::time::Duration;

pub fn run() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let mut arcedVector: Arc<Vec<u8>> = Arc::from(v);
    println!("{:#?}", arcedVector);

    let mut handles = vec![];
    for i in 0..5 {
        let vectorCopy = arcedVector.clone();
        let handle = thread::spawn(move || {
            for j in 0..10 {
                println!("Vector in thread {} reading thread-wide shared vector stored on heap === {:?}", i, vectorCopy);
//                assert_eq!(get_player_name(), format!("player{}", i));
            }
        });
        handles.push(handle)
    }
    for i in handles {
        i.join();
    }
}