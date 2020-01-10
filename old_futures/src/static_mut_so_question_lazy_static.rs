use ::std::sync::RwLock;

#[derive(Debug)]
struct SomeData  { id: u32 }

#[derive(Debug)]
struct SimpleFoo {}

#[derive(Debug)]
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
        println!("Hello, this is ComplexFoo with SomeData={}", self.some_data.id);
    }
}

lazy_static! {
    static ref SELECTED_STRATEGY_LAZY: RwLock< Option<Box<dyn GreetingTrait + Send + Sync>> > = RwLock::new(None);
}

pub fn getI32(foo: i32) {

}

pub fn run()
{
    {
        let mut w = SELECTED_STRATEGY_LAZY.write().unwrap();
        let new_foo: ComplexFoo = ComplexFoo { some_data: SomeData { id: 4 } };
        *w = Some(Box::new(new_foo));
    }
    {
        // but we are able to have many readers simultaneously
        let r1 = SELECTED_STRATEGY_LAZY.read().unwrap();
        match r1.as_ref() {
            Some(foo) => {
                println!("Found some strategy!");
                foo.hello();
            },
            None => println!("None")
        };
    }
    {
        let mut w = SELECTED_STRATEGY_LAZY.write().unwrap();
        let new_foo: ComplexFoo = ComplexFoo { some_data: SomeData { id: 5 } };
        *w = Some(Box::new(new_foo));
    }
    {
        // but we are able to have many readers simultaneously
        let r1 = SELECTED_STRATEGY_LAZY.read().unwrap();
        match r1.as_ref() {
            Some(foo) => {
                println!("Found some strategy!");
                foo.hello();
            },
            None => println!("None")
        };
    }
}