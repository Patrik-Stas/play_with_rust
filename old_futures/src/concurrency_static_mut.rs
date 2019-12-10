struct SomeData { }

struct SimpleFoo {}
struct SimpleBar {}
struct ComplexFoo { some_data: SomeData }

trait GreetingTrait {
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

static mut SELECTED_STRATEGY: &dyn GreetingTrait = &SimpleFoo {};
//static mut SELECTED_STRATEGY_2: dyn GreetingTrait = SimpleFoo {};
//static mut SELECTED_STRATEGY_3: Box<dyn GreetingTrait> = Box::new(SimpleFoo {});


fn print_unsafe() {
    unsafe {
        SELECTED_STRATEGY.hello()
    }
}

fn modify_unsafe_to_bar() {

    unsafe {
        SELECTED_STRATEGY = &SimpleBar {};
    }
}

fn modify_unsafe_to_complex() {

    unsafe {
        let foo = ComplexFoo { some_data: SomeData {} };
//        SELECTED_STRATEGY = &foo;
    }
}


pub fn run()
{
    print_unsafe();
    modify_unsafe_to_bar();
    print_unsafe();
    modify_unsafe_to_complex();
//    unsafe {
//        SELECTED_STRATEGY = &ComplexFoo { some_data: some_data }
//    }
}