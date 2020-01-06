struct SomeData { id: u32 }

struct ComplexFoo { some_data: SomeData }

static mut SELECTED_STRATEGY: &ComplexFoo = &ComplexFoo {some_data: SomeData{ id: 1 }};

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref EXAMPLE: u8 = 42;

}

//fn not_created_at_compile_time() -> &'static ComplexFoo
//{
//    let instance = ComplexFoo { some_data: SomeData { id: 3 } };
//    return &instance
//}
//
//fn created_at_compile_time() -> &'static ComplexFoo
//{
//    return &ComplexFoo { some_data: SomeData { id: 3 } }
//}
//
pub fn run()
{
//    let statically_promoted = created_at_compile_time();
    unsafe {
        let some_data = &SomeData { id: 2 };
//        SELECTED_STRATEGY = &ComplexFoo { some_data: *some_data };
    }
}