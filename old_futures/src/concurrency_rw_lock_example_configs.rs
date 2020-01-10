use std::sync::{Arc, RwLock};
use std::thread;

#[derive(Default, Debug)]
struct ConfigInner {
    debug_level: u8,
}

#[derive(Debug)]
struct Config {
    inner: RwLock<ConfigInner>,
}

impl Config {
    pub fn new() -> Arc<Config> {
        Arc::new(Config { inner: RwLock::new(Default::default()) })
    }
    pub fn current() -> Arc<Config> {
        CURRENT_CONFIG.with(|c| c.clone())
    }
    pub fn debug_level(&self) -> u8 {
        self.inner.read().unwrap().debug_level
    }
    pub fn set_debug_level(&self, value: u8) {
        self.inner.write().unwrap().debug_level = value;
    }
}

thread_local! {
    static CURRENT_CONFIG: Arc<Config> = Config::new();
}

// from
// https://blog.sentry.io/2018/04/05/you-cant-rust-that#refcounts-are-not-dirty
// but adjusted - the original version is using rwlock, yet the data are local to threads and not shared
// so seem like rwlock was not needed in original version. But in this modified version
// the main thread config is passed to all its child threads and mutated within!
pub fn run() {
    let mainThreadConfig = Config::current();
    mainThreadConfig.set_debug_level(0);
    println!("Main thread Config = {:?}", mainThreadConfig);
    let mut handles = vec![];
    for i in 0..5 {
        let mainThreadConfig = mainThreadConfig.clone();
        let handle = thread::spawn(move || {
            let config = Config::current();
            config.set_debug_level(i);
            for j in 0..50 {
                mainThreadConfig.set_debug_level(i);
                println!("In thread {}, Config = {:?}, Modified MainThreadConfig = {:?}", i, config, &mainThreadConfig);
            }
        });
        handles.push(handle)
    }
    for i in handles {
        i.join();
    }
}