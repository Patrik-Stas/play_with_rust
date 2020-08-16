use std::mem;

// https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/unsafe.html#references-and-raw-pointers
pub fn run() {
    let i: u32 = 1;
    println!("i; Reference address {:p}", &i);
    println!("i; Value {}", i);
    // explicit cast
    let p_imm: *const u32 = &i as *const u32;
    println!("i; Raw pointer {:p}", p_imm);
    unsafe {
        println!("i; Raw pointer derefernced {}", *p_imm);
    }


    let mut m: u32 = 2;
    // implicit coercion
    let p_mut: *mut u32 = &mut m;

    unsafe {
        let ref_imm: &u32 = &*p_imm;
        let ref_mut: &mut u32 = &mut *p_mut; // now we have 2 mutable references to the same data!!!

        println!("m; Reference address {:p}", &m);
        println!("m; Value {}", m);


        println!("ref_mut; Reference address {:p}", ref_mut);
        println!("ref_mut; Value {}", ref_mut);

        m=10; //
        println!("ref_mut; Reference address {:p}", ref_mut);
        println!("ref_mut; Value {}", ref_mut);
    }


}
