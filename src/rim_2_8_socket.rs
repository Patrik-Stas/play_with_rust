use std::thread;
use std::time::Duration;
use std::net::TcpListener;

fn open_socket_and_sleep(sleep_seconds: u32)
{
    let _listener = TcpListener::bind("127.0.0.1:5123").unwrap();
    thread::sleep(Duration::from_secs(sleep_seconds.into()))
}

pub fn run()
{
    println!("Starting main");
    open_socket_and_sleep(5);
    println!("Back in main");
    thread::sleep(Duration::from_secs(10));
    println!("Fin");
}

// observe the port with this command:
// watch -n 0.2 'lsof -nP -i4TCP:5123| grep LISTEN'