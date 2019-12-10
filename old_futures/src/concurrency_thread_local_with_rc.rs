use std::rc::Rc;

#[derive(Default)]
struct Config {
    pub debug_mode: bool,
}

impl Config {
    pub fn current() -> Rc<Config> {
        CURRENT_CONFIG.with(|c| c.clone())
    }
}

thread_local! {
    static CURRENT_CONFIG: Rc<Config> = Rc::new(Default::default());
}

pub fn run() {
    let config = Config::current();
    if config.debug_mode {
        // do something
    }
}