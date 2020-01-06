use std::sync::RwLock;
use std::sync::Mutex;

struct SomeData { }

struct SimpleFoo {}
struct SimpleBar {}
struct ComplexFoo { some_data: SomeData }

pub trait GreetingTrait {
    fn hello(&self);
}

impl GreetingTrait for SimpleFoo
{
    fn hello(&self) {
        println!("Hello, this is SimpleFoo");
    }
}

impl GreetingTrait for SimpleBar
{
    fn hello(&self) {
        println!("Hello, this is SimpleBar");
    }
}

impl GreetingTrait for ComplexFoo
{
    fn hello(&self) {
        println!("Hello, this is ComplexFoo");
    }
}

lazy_static! {
    pub static ref SELECTED_STRATEGY_LAZY : Mutex<Box<dyn GreetingTrait + Send + Sync>> = Mutex::new(Box::new(SimpleFoo {}));
}

//static mut SELECTED_STRATEGY : RwLock<Option<Box<dyn GreetingTrait>>> = RwLock::new(None);
//static mut SELECTED_STRATEGY: &dyn GreetingTrait = &SimpleFoo {};
//static mut SELECTED_STRATEGY_2: dyn GreetingTrait = SimpleFoo {};
//static mut SELECTED_STRATEGY_3: Box<dyn GreetingTrait> = Box::new(SimpleFoo {});

fn print_unsafe() {
    SELECTED_STRATEGY_LAZY.lock().unwrap().hello()
}

fn modify_unsafe_to_bar() {
    let bar = SimpleBar {};
    let mut val = SELECTED_STRATEGY_LAZY.lock().unwrap();
    *val = Box::new(bar);
}

fn modify_unsafe_to_complex() {

    let foo = ComplexFoo { some_data: SomeData {} };
    let mut val = SELECTED_STRATEGY_LAZY.lock().unwrap();
    *val = Box::new(foo);
}


pub fn run()
{
    print_unsafe();
    modify_unsafe_to_bar();
    print_unsafe();
    modify_unsafe_to_complex();
    print_unsafe();
//    unsafe {
//        SELECTED_STRATEGY = &ComplexFoo { some_data: some_data }
//    }
}