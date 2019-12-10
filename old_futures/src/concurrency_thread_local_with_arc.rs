use std::sync::Arc;

#[derive(Default)]
struct Config {
    pub debug_mode: bool,
}

impl Config {
    pub fn current() -> Arc<Config> {
        CURRENT_CONFIG.with(|c| c.clone())
    }
}

thread_local! {
    static CURRENT_CONFIG: Arc<Config> = Arc::new(Default::default());
}

pub fn run() {
    let mut config = Config::current();
    if config.debug_mode {
        // do something
    }

    //error[E0594]: cannot assign to data in a `&` reference   https://doc.rust-lang.org/nightly/error-index.html#E0594
//    config.debug_mode = false;
    //^^^^^^^^^^^^^^^^^^^^^^^^^ cannot assign

}