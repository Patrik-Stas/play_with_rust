use std::sync::mpsc::channel;
use std::thread;

pub fn run()
{
    let (sender, receiver) = channel();
    let sender2 = sender.clone();

    thread::spawn(move | | {
        for x in 0..100 {
            sender.send( String::from("from 1")).unwrap();
        }
    });

    thread::spawn(move | | {
        for x in 0..100 {
            sender2.send(String::from("from 2")).unwrap();
        }
    });


    let t = thread::spawn(move | | {
        for x in 0..1000 {
            let msg = receiver.recv().unwrap();
            println!("result = {} ", msg);
        }
    });

    t.join();

}