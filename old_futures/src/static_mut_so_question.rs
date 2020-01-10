struct SomeData {}

struct SimpleFoo {}

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

impl GreetingTrait for ComplexFoo
{
    fn hello(&self) {
        println!("Hello, this is ComplexFoo");
    }
}

static mut SELECTED_STRATEGY: &dyn GreetingTrait = &SimpleFoo {};

pub fn run()
{
//    unsafe {
//        SELECTED_STRATEGY = &ComplexFoo { some_data: SomeData {} };
//        let some_data = SomeData {};
//        SELECTED_STRATEGY = &ComplexFoo { some_data };
//    }

    unsafe {
//        let some_data = SomeData {};
//        let strategy = &ComplexFoo { some_data };
//        let strategy = ComplexFoo { some_data: SomeData {} };
//        SELECTED_STRATEGY = &strategy;
    }

}