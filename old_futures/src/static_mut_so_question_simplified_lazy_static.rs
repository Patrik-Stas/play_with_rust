#[derive(Debug)]
struct SomeData { id: u32 }

#[derive(Debug)]
struct ComplexFoo { some_data: SomeData }

static mut SELECTED_STRATEGY: &ComplexFoo = &ComplexFoo {some_data: SomeData{ id: 1 }};

use ::std::sync::RwLock;

lazy_static! {
    static ref SELECTED_STRATEGY_LAZY: RwLock< Option<ComplexFoo> > = RwLock::new(None);
}

pub fn run()
{
    unsafe {
        let some_data = &SomeData { id: 2 };
//        SELECTED_STRATEGY = &ComplexFoo { some_data: *some_data };
        //creates a temporary which is freed while still in use

        // This doesn't work, although SELECTED_STRATEGY is mutable pointer which by default
        // point to static instance of ComplexFoo (created at compile time with static lifetime)
        // You could only change value of this to other ComplexFoos with static lifetimes.
        // However lifetime of created ComplexFoo instance is very short. It depends on
        // some_data, which is created on stack, so insteance of ComplexFoo is as well created
        // on stack and is dropped as soon as this unsafe block ends.
    }
    unsafe {
        SELECTED_STRATEGY = &ComplexFoo { some_data: SomeData { id: 3 } };
        // in contrast to above, this actually works, because of static promotion of ComplexFoo.
        // Although it seems like we are creating ComplexFoo in this block, thanks to static promotion
        // instance of ComplexFoo { some_data: SomeData { id: 3 } } is created in compile time
        // as static data! So by doing this assignment here, we are literally switching a pointer
        // from pointing from default static data to another static data
    }
    {
        let mut w = SELECTED_STRATEGY_LAZY.write().unwrap();
        let new_foo: ComplexFoo = ComplexFoo { some_data: SomeData { id: 4 } };
        *w = Some(new_foo);

        // We just need to make sure we never have open SELECTED_STRATEGY_LAZY for both write and read
        // otherwise we get runtime panic (uncomment and see)
//        let mut r0 = SELECTED_STRATEGY_LAZY.read().unwrap();
//        println!("SELECTED_STRATEGY_LAZY = {:?}", r0);
    }
    {
        // but we are able to have many readers simultaneously
        let mut r1 = SELECTED_STRATEGY_LAZY.read().unwrap();
        println!("SELECTED_STRATEGY_LAZY = {:?}", r1);

        let mut r2 = SELECTED_STRATEGY_LAZY.read().unwrap();
        println!("SELECTED_STRATEGY_LAZY = {:?}", r2);

        let mut r3 = SELECTED_STRATEGY_LAZY.read().unwrap();
        println!("SELECTED_STRATEGY_LAZY = {:?}", r3);
    }
}