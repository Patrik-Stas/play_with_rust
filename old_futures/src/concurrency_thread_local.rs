use std::cell::RefCell;
use std::thread;
use std::sync::atomic::AtomicUsize;


//lazy_static! {
//    pub static ref atomic_forty_two = AtomicUsize::new(42);
//}

thread_local! {
    pub static PLAYER_NAME: RefCell<String>
        = RefCell::new("noname".to_string());
}

fn set_player_name(name: String) {
    PLAYER_NAME.with(|player_name| {
        *player_name.borrow_mut() = name
    });
}

fn get_player_name() -> String {
    PLAYER_NAME.with(|player_name| player_name.borrow().clone())
}

pub fn run() {

    for i in 0..5 {
        thread::spawn(move || {
            set_player_name(format!("player{}", i));
            for j in 0..50000 {
//                println!("Player name in thread {} is {}", i, get_player_name()); // this is crashing for some reason, seems like some problem using pritnln! concurrently?
                assert_eq!(get_player_name(), format!("player{}", i));
            }
        });
    }
}