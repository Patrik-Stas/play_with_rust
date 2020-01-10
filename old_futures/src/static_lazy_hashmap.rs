#[macro_use]
use std::collections::HashMap;
use std::sync::Mutex;
// https://rust-lang-nursery.github.io/rust-cookbook/mem/global_static.html

//lazy_static! {
//    // store a PostgresStorage object (contains a connection)
//    static ref STATIC_HASHMAP: HashMap<i32, i32> = Default::default();
//}


lazy_static! {
    // store a PostgresStorage object (contains a connection)
    static ref STATIC_HASHMAP: Mutex<HashMap<i32, i32>> = Default::default();
}

lazy_static! {
    static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("James", vec!["user", "admin"]);
        map.insert("Jim", vec!["user"]);
        map
    };
}

fn show_access(name: &str) {
    let access = PRIVILEGES.get(name);
    println!("{}: {:?}", name, access);
}

pub fn run() {
    let access = PRIVILEGES.get("James");
    println!("James: {:?}", access);
    show_access("Jim");
    {
        let mut hashmap = STATIC_HASHMAP.lock().unwrap();
        hashmap.insert(12, 34);
        let res = hashmap.get(&12);
        println!("my hashmap result {:?}", res);
    }
}