use std::mem;

pub fn run() {
    let mut x: u8 = 1;

    let ref_1: &mut u8 = &mut x;
    let ref_2: &mut u8 = unsafe { mem::transmute(&mut *ref_1) };

    // oops, ref_1 and ref_2 point to the same piece of data (x) and are
    // both usable
    *ref_1 = 10;
    println!("*ref_1 = {}  address: {:p}", *ref_1, ref_1);
    println!("*ref_2 = {}  address: {:p}", *ref_2, ref_2);

    *ref_2 = 20;
    println!("*ref_1 = {}  address: {:p}", *ref_1, ref_1);
    println!("*ref_2 = {}  address: {:p}", *ref_2, ref_2);

    // transmute mess up example: Transmute from unsigned to signed type changes interpretation of the data
    let mut y: u8 = 250;

    let ref_y1: &mut u8 = &mut y;
    let ref_y2: &mut i8 = unsafe { mem::transmute(&mut *ref_y1) }; // Reinterprets the bits of a value of one type as another type.
    println!("*ref_y1 = {}  address: {:p}", *ref_y1, ref_y1); // *ref_y1 = 250  address: 0x7ffee0eb2c0f
    println!("*ref_y2 = {}  address: {:p}", *ref_y2, ref_y2); // *ref_y2 = -6  address: 0x7ffee0eb2c0f

    // transmute example: Turning a pointer into a function pointer.
    fn foo() -> i32 {
        println!("function foo() was called");
        0
    }
    let pointer = foo as *const ();
    let function = unsafe {
        std::mem::transmute::<*const (), fn() -> i32>(pointer)
    };
    assert_eq!(function(), 0);



}
